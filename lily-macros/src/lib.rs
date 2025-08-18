use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{Fields, ItemStruct, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn lily_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Get the struct
    let mut struct_ast: ItemStruct = parse_macro_input!(item as ItemStruct);

    // Create the code for the payload struct
    let original_struct_name: &syn::Ident = &struct_ast.ident;
    let payload_name: syn::Ident = format_ident!("{}Payload", original_struct_name);
    let original_fields = if let Fields::Named(fields) = &struct_ast.fields {
        &fields.named
    } else {
        panic!("This macro only works on structs with named fields");
    };
    let payload_struct_code = quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #payload_name {
            #original_fields
        }
    };

    // Create the code for the from/into implementation
    let field_names = original_fields.iter().map(|f| &f.ident);
    let payload_from_into_code = quote! {
        impl From<#payload_name> for #original_struct_name {
            fn from(payload: #payload_name) -> Self {
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

    // Add derives to the original struct
    let derives: syn::Attribute = parse_quote! {
        #[derive(Clone, Debug, serde::Serialize)]
    };
    struct_ast.attrs.push(derives);

    // Create code for the module's get_routes()
    let snake_name: String = to_snake_case(&original_struct_name.to_string());
    let base_path: String = format!("/{}", snake_name);
    let base_path_with_id: String = format!("/{}/{{id}}", snake_name);
    let get_routes_code = quote! {
        pub fn get_routes() -> Router {
            Router::new()
                .route(#base_path, post(derived_create_one))
                .route(#base_path_with_id, get(derived_read_one))
                .route(#base_path, get(derived_read_all))
                .route(#base_path_with_id, put(derived_update_one))
                .route(#base_path_with_id, delete(derived_delete_one))
        }

        // generate "create one"
        pub async fn derived_create_one(
            Json(payload): Json<#payload_name>,
        ) -> (StatusCode, Json<#original_struct_name>) {
            let r: Result<#original_struct_name, Error> = #original_struct_name::create_one(payload);

            match r {
                Ok(data) => (StatusCode::CREATED, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error creating one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#original_struct_name::invalid()))
                }
            }
        }

        // generate "read one"
        pub async fn derived_read_one(Path(id): Path<String>) -> (StatusCode, Json<#original_struct_name>) {
            let r: Result<#original_struct_name, Error> = #original_struct_name::read_one(id);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error fetching one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#original_struct_name::invalid()))
                }
            }
        }

        // generate "read all"
        pub async fn derived_read_all() -> (StatusCode, Json<Vec<#original_struct_name>>) {
            let r: Result<Vec<#original_struct_name>, Error> = #original_struct_name::read_all();

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error fetching all [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(Vec::new()))
                }
            }
        }

        // generate "update one"
        pub async fn derived_update_one(
            Path(id): Path<String>,
            Json(payload): Json<#payload_name>,
        ) -> (StatusCode, Json<#original_struct_name>) {
            let r: Result<#original_struct_name, Error> = #original_struct_name::update_one(id, payload);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error updating one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#original_struct_name::invalid()))
                }
            }
        }

        // generate "delete one"
        pub async fn derived_delete_one(Path(id): Path<String>) -> (StatusCode, Json<#original_struct_name>) {
            let r: Result<#original_struct_name, Error> = #original_struct_name::delete_one(id);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error deleting one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#original_struct_name::invalid()))
                }
            }
        }
    };

    // Generate code
    let output = quote! {
        #struct_ast
        #payload_struct_code
        #payload_from_into_code
        #get_routes_code
    };

    // Print the generated code during compilation
    let info: String = format!("📦📦📦 generated code for [{}] 📦📦📦", snake_name);
    println!("\n{}\n\n{}\n\n", info, output);

    output.into()
}

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
