use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use strum::IntoEnumIterator;
use syn::{
    Fields, Ident, ItemStruct, Token, parse::Parser, parse_macro_input, parse_quote,
    punctuated::Punctuated,
};
use util::to_snake_case;

mod routing;
mod util;

struct StructNames {
    original: syn::Ident,
    snake_case: String,
    create_payload_name: syn::Ident,
    update_payload_name: syn::Ident,
}
impl StructNames {
    fn new(ast: &ItemStruct) -> Self {
        let struct_name: syn::Ident = ast.ident.clone();
        let snake_case = to_snake_case(&struct_name.to_string());
        StructNames {
            create_payload_name: format_ident!("Create{}", &struct_name),
            update_payload_name: format_ident!("Update{}", &struct_name),
            original: struct_name,
            snake_case: snake_case,
        }
    }
}

#[proc_macro_attribute]
pub fn endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    // MARK: 🔖 Init
    let mut struct_ast: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_names = StructNames::new(&struct_ast);
    let original_struct_name: &syn::Ident = &struct_names.original;
    let create_payload_name: &syn::Ident = &struct_names.create_payload_name;
    let update_payload_name: &syn::Ident = &struct_names.update_payload_name;
    let snake_name: &String = &struct_names.snake_case;

    // Parse macro arguments
    let args = Punctuated::<Ident, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse macro arguments");

    // Create boolean flags based on the parsed arguments
    let enabled_actions: HashSet<String> = if args.is_empty() {
        routing::Routes::iter()
            .map(|route| route.get_path().to_owned())
            .collect()
    } else {
        args.iter().map(|ident| ident.to_string()).collect()
    };

    // MARK: 🔖Struct
    // Get the struct and related information
    let original_fields = if let Fields::Named(fields) = &struct_ast.fields {
        &fields.named
    } else {
        panic!("This macro only works on structs with named fields");
    };

    // Create the code for the create-payload (POST) struct
    let create_payload_tokens: proc_macro2::TokenStream = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #create_payload_name {
            #original_fields
        }
    };

    // Create the code for the update-payload (PATCH) struct
    let update_payload_tokens: proc_macro2::TokenStream = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #update_payload_name {
            // TODO: Make all Option<OriginalType>
            #original_fields
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

    // MARK: 🔖Endpoints
    let impl_route_builder_tokens: proc_macro2::TokenStream =
        routing::get_route_builder(&struct_names, &enabled_actions);

    let impl_endpoint_tokens = quote! {
        impl Endpoint for #original_struct_name {
            type Id = String;
            type CreatePayload = #create_payload_name;
            type UpdatePayload = #update_payload_name;

            fn get_name() -> String {
                #snake_name.to_owned()
            }
            fn get_path() -> String {
                format!("/{}", Self::get_name())
            }
            fn get_path_with_id() -> String {
                format!("/{}/{{id}}", Self::get_name())
            }
        }

    };

    // MARK: 🔖Build
    // Generate code
    let output = quote! {
        #struct_ast
        #create_payload_tokens
        #update_payload_tokens
        #impl_endpoint_tokens
        #impl_route_builder_tokens
    };

    output.into()
}
