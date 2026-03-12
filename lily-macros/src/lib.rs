use crate::util::StructNames;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

mod endpoint;
mod util;

#[proc_macro_attribute]
pub fn endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse struct
    let struct_ast: ItemStruct = parse_macro_input!(item as ItemStruct);

    // Generate struct names
    let struct_names = StructNames::from(&struct_ast);

    // Generate endpoint tokens
    let endpoint_tokens = endpoint::generate_endpoint_tokens(struct_ast, &struct_names, attr);

    // Generate persistence tokens
    let persistence_tokens = proc_macro2::TokenStream::new();

    // Return token stream
    let output = quote! {
        #endpoint_tokens
        #persistence_tokens
    };

    output.into()
}
