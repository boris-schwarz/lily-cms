extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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

#[proc_macro_derive(GeneratePayload, attributes(metadata))]
pub fn generate_payload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("📦 input parsed...");

    // get the name of the struct
    let name: syn::Ident = input.ident;
    let snake_name: String = to_snake_case(&name.to_string());

    println!("📦 name parsed");

    // generate payload struct name
    let payload_name: syn::Ident = format_ident!("{}Payload", name);

    // generate url paths
    let base_path: String = format!("/{}", snake_name);
    let base_path_with_id: String = format!("/{}/{{id}}", snake_name);

    println!("📦 payload and path path parsed");

    // get struct fields
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    println!("📦 fields parsed");

    // filter out metadata annotated fields
    let payload_fields = fields.iter().filter(|field| {
        !field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("metadata"))
    });
    let field_names = payload_fields.clone().map(|f| {
        f.ident
            .as_ref()
            .expect("Named fields should have identifiers")
    });

    // ❓❓ debug output
    let output = quote! {
        // generate the type's payload
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #payload_name {
            #(#payload_fields,)*
        }
    };
    return output.into();

    // generate output
    /* TODO
    id: String,
    created_at: String,
    created_by: String,
    updated_at: String,
    updated_by: String,
    deleted_at: String,
    deleted_by: String,
     */
    let output = quote! {
        // generate the type's payload
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct #payload_name {
            #(#payload_fields,)*
        }

        // generate the From implementation for the type and it's generated payload
        impl From<#payload_name> for #name {
            fn from(payload: #payload_name) -> Self {
                let mut entity = #name {
                    //id: uuid::Uuid::new_v4().to_string(),
                    id: String::from("uuid-from-the-macro"), // TODO
                    #(#field_names: payload.#field_names,)*
                };

                entity
            }
        }

        // generate routes
        pub fn get_routes() -> Router {
            Router::new()
                .route(#base_path, post(derived_create_one))
                .route(#base_path_with_id, get(derived_read_one))
                .route(#base_path, get(derived_read_all))
                .route(#base_path_with_id, put(derived_update_one))
                .route(#base_path_with_id, delete(derived_delete_one))
        }

        // MARK: CREATE

        // generate "create one"
        pub async fn derived_create_one(
            Json(payload): Json<#payload_name>,
        ) -> (StatusCode, Json<#name>) {
            let r: Result<#name, Error> = #name::create_one(payload);

            match r {
                Ok(data) => (StatusCode::CREATED, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error creating one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#name::invalid()))
                }
            }
        }

        // MARK: READ

        // generate "read one"
        pub async fn derived_read_one(Path(id): Path<String>) -> (StatusCode, Json<#name>) {
            let r: Result<#name, Error> = #name::read_one(id);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error fetching one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#name::invalid()))
                }
            }
        }

        // generate "read all"
        pub async fn derived_read_all() -> (StatusCode, Json<Vec<#name>>) {
            let r: Result<Vec<#name>, Error> = #name::read_all();

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error fetching all [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(Vec::new()))
                }
            }
        }

        // MARK: UPDATE

        // generate "update one"
        pub async fn derived_update_one(
            Path(id): Path<String>,
            Json(payload): Json<#payload_name>,
        ) -> (StatusCode, Json<#name>) {
            let r: Result<#name, Error> = #name::update_one(id, payload);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error updating one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#name::invalid()))
                }
            }
        }

        // MARK: DELETE

        // generate "delete one"
        pub async fn derived_delete_one(Path(id): Path<String>) -> (StatusCode, Json<#name>) {
            let r: Result<#name, Error> = #name::delete_one(id);

            match r {
                Ok(data) => (StatusCode::OK, Json(data)),
                Err(error_msg) => {
                    eprintln!(concat!("Error deleting one [", #snake_name, "]: {}"), error_msg);
                    (StatusCode::NOT_FOUND, Json(#name::invalid()))
                }
            }
        }

    };

    // print the generated code during compilation
    let info: String = format!("📦📦📦 generated code for [{}] 📦📦📦", snake_name);
    println!("\n{}\n\n{}\n\n", info, output);

    // return the output
    let token_stream = output.into();

    println!("📦📦📦 token stream 📦📦📦\n{}", token_stream);

    token_stream
}
