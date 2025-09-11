use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{
    Fields, Ident, ItemStruct, Token, parse::Parser, parse_macro_input, parse_quote,
    punctuated::Punctuated,
};

mod routing;

#[proc_macro_attribute]
pub fn expose_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    // MARK: 🔖 Init

    // Define all possible actions
    const ALL_ACTIONS: &[&str] = &[
        "read_one",     // GET
        "read_many",    // GET
        "create_one",   // POST
        "create_many",  // POST
        "replace_one",  // PUT
        "replace_many", // PUT
        "update_one",   // PATCH
        "update_many",  // PATCH
        "delete_one",   // DELETE
        "delete_many",  // DELETE
    ];

    // Parse macro arguments
    let args = Punctuated::<Ident, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse macro arguments");

    // Create boolean flags based on the parsed arguments
    let enabled_actions: HashSet<String> = if args.is_empty() {
        ALL_ACTIONS.iter().map(|s| s.to_string()).collect()
    } else {
        args.iter().map(|ident| ident.to_string()).collect()
    };

    // MARK: 🔖 Struct
    // Get the struct and related information
    let mut struct_ast: ItemStruct = parse_macro_input!(item as ItemStruct);
    let original_struct_name: &syn::Ident = &struct_ast.ident;
    let full_payload_name: syn::Ident = format_ident!("{}FullPayload", original_struct_name);
    let original_fields = if let Fields::Named(fields) = &struct_ast.fields {
        &fields.named
    } else {
        panic!("This macro only works on structs with named fields");
    };

    // Create the code for the payload-struct
    let payload_struct_code = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #full_payload_name {
            #original_fields
        }
    };

    // Create the code for the from/into implementation for the payload-struct
    let field_names = original_fields.iter().map(|f| &f.ident);
    let payload_from_into_code = quote! {
        impl From<#full_payload_name> for #original_struct_name {
            fn from(payload: #full_payload_name) -> Self {
                #original_struct_name {
                    id: String::new(),
                    created_at: chrono::Utc::now(),
                    #(#field_names: payload.#field_names,)*
                }
            }
        }
    };

    // Add metadata to the original struct
    if let Fields::Named(fields) = &mut struct_ast.fields {
        let id_field: syn::Field = syn::Field::parse_named
            .parse_str("id: String")
            .expect("Failed to parse named field");
        fields.named.insert(0, id_field);

        let created_at_field = syn::Field::parse_named
            .parse_str("created_at: chrono::DateTime<chrono::Utc>")
            .expect("Failed to parse created_at field");
        fields.named.insert(1, created_at_field);
    }

    // Add derive attributes to the original struct
    let derives: syn::Attribute = parse_quote! {
        #[derive(Clone, Debug, serde::Serialize)]
    };
    struct_ast.attrs.push(derives);

    // MARK: 🔖 Routers
    let snake_name: String = to_snake_case(&original_struct_name.to_string());
    let route_path: String = format!("/{}", snake_name);
    let route_path_with_id: String = format!("/{}/{{id}}", snake_name);

    // Create code for the router "create_one" (POST)
    let create_one_router_code = enabled_actions
        .contains("create_one")
        .then(|| routing::get_create_one_router(&route_path));

    // Create code for the router "read_one" (GET)
    let read_one_router_code = enabled_actions
        .contains("read_one")
        .then(|| routing::get_read_one_router(&route_path_with_id));

    // Create code for the router "read_many" (GET)
    let read_many_router_code = enabled_actions
        .contains("read_many")
        .then(|| routing::get_read_many_router(&route_path));

    // Create code for the router "update_one" (PATCH)
    let update_one_router_code = enabled_actions
        .contains("update_one")
        .then(|| routing::get_update_one_router(&route_path_with_id));

    // Create code for the router "replace_one" (PUT)
    let replace_one_router_code = enabled_actions
        .contains("replace_one")
        .then(|| routing::get_replace_one_router(&route_path_with_id));

    // Create code for the router "delete_one" (DELETE)
    let delete_one_router_code = enabled_actions
        .contains("delete_one")
        .then(|| routing::get_delete_one_router(&route_path_with_id));

    // Create code for the handler "create_one" (GET)
    let create_one_handler_code = enabled_actions.contains("create_one").then(|| {
        routing::get_create_one_handler(&original_struct_name, &full_payload_name, &snake_name)
    });

    // MARK: 🔖 Handlers
    // Create code for the handler "read_one" (GET)
    let read_one_handler_code = enabled_actions
        .contains("read_one")
        .then(|| routing::get_read_one_handler(&original_struct_name, &snake_name));

    // Create code for the handler "read_many" (GET)
    let read_many_handler_code = enabled_actions
        .contains("read_many")
        .then(|| routing::get_read_many_handler(&original_struct_name, &snake_name));

    // Create code for the handler "update_one" (PATCH)
    let update_one_handler_code = enabled_actions.contains("update_one").then(|| {
        routing::get_update_one_handler(&original_struct_name, &full_payload_name, &snake_name)
    });

    // Create code for the handler "replace_one" (PATCH)
    let replace_one_handler_code = enabled_actions.contains("replace_one").then(|| {
        routing::get_replace_one_handler(&original_struct_name, &full_payload_name, &snake_name)
    });

    // Create code for the handler "delete_one" (PATCH)
    let delete_one_handler_code = enabled_actions
        .contains("delete_one")
        .then(|| routing::get_delete_one_handler(&original_struct_name, &snake_name));

    // --------------------------------------------------------

    // Create code for the module's get_routes()
    let get_routes_code = quote! {
        pub fn get_routes() -> Router {
            let router = Router::new();
            #create_one_router_code
            #read_one_router_code
            #read_many_router_code
            #update_one_router_code
            #replace_one_router_code
            #delete_one_router_code

            router
        }

        #create_one_handler_code
        #read_one_handler_code
        #read_many_handler_code
        #update_one_handler_code
        #replace_one_handler_code
        #delete_one_handler_code
    };

    // MARK: 🔖 Build
    // Generate code
    let output = quote! {
        #struct_ast
        #payload_struct_code
        #payload_from_into_code
        #get_routes_code
    };

    output.into()
}

/// Converts a string from lowerCamelCase to snake_case
///
/// # Examples
/// ```
/// let snake_case: String = to_snake_case("lowerCamelCase");
/// ```
///
/// # Note
/// This function was created by Claude Code
/// TODO: Verify that it's doing what it should do
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();

    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            // Add underscore if not the first character
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}
