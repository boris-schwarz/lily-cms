use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use syn::{Ident, Token, parse::Parser, punctuated::Punctuated};

use crate::StructNames;

pub mod payload;
pub mod route_gen;

pub fn expand_shorthand(action: &str) -> Vec<String> {
    match action {
        "read" => vec!["read_single".into(), "read_multiple".into()],
        other => vec![other.into()],
    }
}

pub fn parse_macro_args(attr: TokenStream) -> HashSet<String> {
    // Parse macro arguments
    let args = Punctuated::<Ident, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse macro arguments");

    // Create boolean flags based on the parsed arguments
    let enabled_actions: HashSet<String> = if args.is_empty() {
        route_gen::Routes::iter()
            .map(|route| route.as_snake_case().to_owned())
            .collect()
    } else {
        args.iter()
            .flat_map(|ident| crate::endpoint::expand_shorthand(&ident.to_string()))
            .collect()
    };

    enabled_actions
}

pub fn generate_endpoint_tokens(
    struct_ast: syn::ItemStruct,
    struct_names: &StructNames,
    attr: TokenStream,
) -> proc_macro2::TokenStream {
    // Parse macro arguments
    let enabled_actions: HashSet<String> = parse_macro_args(attr);

    // Create payloads
    let payload_tokens = payload::generate_payload(struct_ast, &struct_names);

    // Create routes
    let route_builder_tokens: proc_macro2::TokenStream =
        route_gen::get_route_builder(&struct_names, &enabled_actions);

    // Create implementation for Endpoint trait
    let original_struct_name: &syn::Ident = &struct_names.original;
    let create_payload_name: &syn::Ident = &struct_names.create_payload_name;
    let update_payload_name: &syn::Ident = &struct_names.update_payload_name;
    let snake_name: &String = &struct_names.snake_case;
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

    quote! {
        #payload_tokens
        #route_builder_tokens
        #impl_endpoint_tokens
    }
}
