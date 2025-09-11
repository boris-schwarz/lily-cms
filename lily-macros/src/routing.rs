use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

// MARK: 🔖 Routers
pub fn get_create_one_router(path: &String) -> TokenStream {
    quote! {
            let create_one_router = Router::new().route(#path, post(create_one_handler));
            let router = router.merge(create_one_router);
    }
}

pub fn get_read_one_router(path: &String) -> TokenStream {
    quote! {
            let read_one_router = Router::new().route(#path, get(read_one_handler));
            let router = router.merge(read_one_router);
    }
}

pub fn get_read_many_router(path: &String) -> TokenStream {
    quote! {
            let read_many_router = Router::new().route(#path, get(read_many_handler));
            let router = router.merge(read_many_router);
    }
}

pub fn get_update_one_router(path: &String) -> TokenStream {
    quote! {
            let update_one_router = Router::new().route(#path, patch(update_one_handler));
            let router = router.merge(update_one_router);
    }
}

pub fn get_replace_one_router(path: &String) -> TokenStream {
    quote! {
            let replace_one_router = Router::new().route(#path, put(replace_one_handler));
            let router = router.merge(replace_one_router);
    }
}

pub fn get_delete_one_router(path: &String) -> TokenStream {
    quote! {
            let delete_one_router = Router::new().route(#path, delete(delete_one_handler));
            let router = router.merge(delete_one_router);
    }
}

// MARK: 🔖 Handlers

// handler for "create_one"
pub fn get_create_one_handler(
    original_struct_name: &syn::Ident,
    full_payload_name: &syn::Ident,
    snake_name: &String,
) -> TokenStream {
    quote! {
        async fn create_one_handler(
            Json(payload): Json<#full_payload_name>,
        ) -> ApiResponse<#original_struct_name> {
            let result: Result<#original_struct_name, Error> = #original_struct_name::create_one(&payload);

            match result {
                Ok(data) => ApiResponse::Created(data),
                Err(error) => match error {
                    _ => {
                        eprintln!(concat!("Error creating one [", #snake_name, "]: {}"), error);
                        ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                    }
                }
            }
        }
    }
}

// handler for "read_one"
pub fn get_read_one_handler(original_struct_name: &syn::Ident, snake_name: &String) -> TokenStream {
    quote! {
        async fn read_one_handler(Path(id): Path<String>) -> ApiResponse<#original_struct_name> {
                let result: Result<Option<#original_struct_name>, Error> = #original_struct_name::read_one(&id);

                match result {
                        Ok(option) => match option {
                                Some(data) => ApiResponse::Ok(data),
                                None => ApiResponse::NotFound(Problem::ResourceNotFound {
                                                                                resource: #snake_name.to_string(),
                                                                                id: id,
                                }),
                        },
                        Err(error) => {
                                eprintln!(concat!("Error fetching one [", #snake_name, "]: {}"), error);
                                ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                        }
                }
        }
    }
}

// handler for "read_many"
pub fn get_read_many_handler(
    original_struct_name: &syn::Ident,
    snake_name: &String,
) -> TokenStream {
    quote! {
        async fn read_many_handler() -> ApiResponse<Vec<#original_struct_name>> {
            let result: Result<Vec<#original_struct_name>, Error> = #original_struct_name::read_all();

            match result {
                Ok(data) => ApiResponse::Ok(data),
                Err(error_msg) => {
                    eprintln!(concat!("Error fetching all [", #snake_name, "]: {}"), error_msg);
                    ApiResponse::Erroneous::<Vec<#original_struct_name>>(Problem::InternalError)
                }
            }
        }
    }
}

// handler for "update_one"
pub fn get_update_one_handler(
    original_struct_name: &syn::Ident,
    full_payload_name: &syn::Ident,
    snake_name: &String,
) -> TokenStream {
    quote! {
        async fn update_one_handler(
            Path(id): Path<String>,
            Json(payload): Json<#full_payload_name>,
        ) -> ApiResponse<#original_struct_name> {
            let result: Result<#original_struct_name, Error> = #original_struct_name::update_one(&id, &payload);

            match result {
                Ok(data) => ApiResponse::Ok(data),
                Err(error_msg) => {
                    eprintln!(concat!("Error updating one [", #snake_name, "]: {}"), error_msg);
                    ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                }
            }
        }
    }
}

// handler for "replace_one"
pub fn get_replace_one_handler(
    original_struct_name: &syn::Ident,
    full_payload_name: &syn::Ident,
    snake_name: &String,
) -> TokenStream {
    quote! {
        async fn replace_one_handler(
            Path(id): Path<String>,
            Json(payload): Json<#full_payload_name>,
        ) -> ApiResponse<#original_struct_name> {
            // TODO: Create partial_payload_name with Options
            // TODO: Create replace_one()
            let result: Result<#original_struct_name, Error> = #original_struct_name::update_one(&id, &payload);
            match result {
                Ok(data) => ApiResponse::Ok(data),
                Err(error_msg) => {
                    eprintln!(concat!("Error replacing one [", #snake_name, "]: {}"), error_msg);
                    ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                }
            }
        }
    }
}

// handler for "delete_one"
pub fn get_delete_one_handler(
    original_struct_name: &syn::Ident,
    snake_name: &String,
) -> TokenStream {
    quote! {
        async fn delete_one_handler(Path(id): Path<String>) -> ApiResponse<#original_struct_name> {
            let result: Result<(), Error> = #original_struct_name::delete_one(&id);

            match result {
                Ok(_) => ApiResponse::NoContent,
                Err(error_msg) => {
                    eprintln!(concat!("Error deleting one [", #snake_name, "]: {}"), error_msg);
                    ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                }
            }
        }
    }
}
