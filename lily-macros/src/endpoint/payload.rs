use crate::{StructNames, util::is_option};
use quote::quote;
use syn::{Fields, parse::Parser, parse_quote};

pub fn generate_payload(
    mut struct_ast: syn::ItemStruct,
    struct_names: &StructNames,
) -> proc_macro2::TokenStream {
    // let original_struct_name: &syn::Ident = &struct_names.original;
    let create_payload_name: &syn::Ident = &struct_names.create_payload_name;
    let update_payload_name: &syn::Ident = &struct_names.update_payload_name;
    // let snake_name: &String = &struct_names.snake_case;

    // Get all struct fields
    let original_fields = if let syn::Fields::Named(fields) = &struct_ast.fields {
        &fields.named
    } else {
        panic!("This macro only works on structs with named fields");
    };

    // Derive set of optional fields from original struct fields
    let optional_fields: Vec<proc_macro2::TokenStream> = original_fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            let attrs = &field.attrs;

            if is_option(ty) {
                quote! { #(#attrs)* #name: #ty }
            } else {
                quote! { #(#attrs)* #name: Option<#ty> }
            }
        })
        .collect();

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
            #(#optional_fields),*
        }
    };

    // Add metadata to the original struct
    // TODO: Check if this makes sense here, maybe in a later step move it to persistence module
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

    // Return token stream
    quote! {
        #struct_ast
        #create_payload_tokens
        #update_payload_tokens
    }
}
