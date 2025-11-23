use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use strum::IntoEnumIterator;
use syn::{
    Fields, Ident, ItemStruct, Token, parse::Parser, parse_macro_input, parse_quote,
    punctuated::Punctuated,
};

mod routing;

struct StructNames {
    original: syn::Ident,
    snake_case: String,
    post_payload_name: syn::Ident,
    patch_payload_name: syn::Ident,
}
impl StructNames {
    fn new(ast: &ItemStruct) -> Self {
        let struct_name: syn::Ident = ast.ident.clone();
        let snake_case = to_snake_case(&struct_name.to_string());
        StructNames {
            post_payload_name: format_ident!("Post{}", &struct_name),
            patch_payload_name: format_ident!("Patch{}", &struct_name),
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
    let post_payload_name: &syn::Ident = &struct_names.post_payload_name;
    let patch_payload_name: &syn::Ident = &struct_names.patch_payload_name;
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

    // Create the code for the POST payload struct
    let post_payload_code: proc_macro2::TokenStream = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #post_payload_name {
            #original_fields
        }
    };

    // Create the code for the PATCH payload struct
    let patch_payload_code: proc_macro2::TokenStream = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #patch_payload_name {
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
    let impl_route_builder_code: proc_macro2::TokenStream =
        routing::get_route_builder(&struct_names);

    let impl_endpoint_code = quote! {
        impl Endpoint for #original_struct_name {
            type Id = String;
            type PostPayload = #post_payload_name;
            // type PatchPayload = #patch_payload_name;

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
        #post_payload_code
        #patch_payload_code
        #impl_endpoint_code
        #impl_route_builder_code
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
