use crate::StructNames;
use crate::util::to_kebab_case;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use strum::{Display, EnumIter};

pub fn get_route_builder(
    struct_names: &StructNames,
    enabled_actions: &HashSet<String>,
) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;

    // MARK: Create Single
    let add_create_single_route_tokens: TokenStream = if enabled_actions.contains("create_single") {
        quote! {
            async fn create_single_handler(Json(payload): Json<<#original_struct_name as Endpoint>::CreatePayload>) -> ApiResponse<#original_struct_name> {
                let result = #original_struct_name::create_single(&payload).await;

                match result {
                    Ok(data) => ApiResponse::Ok(data),
                    Err(error_msg) => {
                        eprintln!(concat!("Error creating single [", #snake_name, "]: {}"), error_msg);
                        ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                    }
                }
            }

            router.route(&#original_struct_name::get_path(), post(create_single_handler))
        }
    } else {
        return_router_code()
    };

    // MARK: Read Single
    let add_read_single_route_tokens: TokenStream = if enabled_actions.contains("read_single") {
        quote! {
            async fn read_single_handler(Path(id): Path<<#original_struct_name as Endpoint>::Id>) -> ApiResponse<#original_struct_name> {
                let result = #original_struct_name::read_single(&id).await;

                match result {
                    Ok(option) => match option {
                        Some(data) => ApiResponse::Ok(data),
                        None => ApiResponse::NotFound(Problem::ResourceNotFound {
                            resource: #snake_name.to_string(),
                            id: id,
                        }),
                    },
                    Err(error_msg) => {
                        eprintln!(concat!("Error reading single [", #snake_name, "]: {}"), error_msg);
                        ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                    }
                }
            }

            router.route(&#original_struct_name::get_path_with_id(), get(read_single_handler))
        }
    } else {
        return_router_code()
    };

    // MARK: Update Single
    let add_update_single_route_tokens: TokenStream = if enabled_actions.contains("update_single") {
        quote! {
            async fn update_single_handler(Path(id): Path<<#original_struct_name as Endpoint>::Id>, Json(payload): Json<<#original_struct_name as Endpoint>::UpdatePayload>) -> ApiResponse<#original_struct_name> {
                let result = #original_struct_name::update_single(&id, &payload).await;

                match result {
                    Ok(data) => ApiResponse::Ok(data),
                    Err(error_msg) => {
                        eprintln!(concat!("Error updating single [", #snake_name, "]: {}"), error_msg);
                        ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                    }
                }
            }

            router.route(&#original_struct_name::get_path_with_id(), patch(update_single_handler))
        }
    } else {
        return_router_code()
    };

    // MARK: Replace Single
    // let add_replace_single_route_tokens: TokenStream = if enabled_actions.contains("replace_single") {
    //     quote! {
    //         async fn replace_single_handler(Path(id): Path<<#original_struct_name as Endpoint>::Id>, Json(payload): Json<<#original_struct_name as Endpoint>::ReplacePayload>) -> ApiResponse<#original_struct_name> {
    //             let result = #original_struct_name::replace_single(&id, &payload).await;

    //             match result {
    //                 Ok(data) => ApiResponse::Ok(data),
    //                 Err(error_msg) => {
    //                     eprintln!(concat!("Error replacing single [", #snake_name, "]: {}"), error_msg);
    //                     ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
    //                 }
    //             }
    //         }

    //         router.route(&#original_struct_name::get_path_with_id(), put(replace_single_handler))
    //     }
    // } else {
    //     return_router_code()
    // };

    // // MARK: Delete Single
    let add_delete_single_route_tokens: TokenStream = if enabled_actions.contains("delete_single") {
        quote! {
            async fn delete_single_handler(Path(id): Path<<#original_struct_name as Endpoint>::Id>) -> ApiResponse<#original_struct_name> {
                let result = #original_struct_name::delete_single(&id).await;

                match result {
                    Ok(option) => match option {
                        Some(data) => ApiResponse::Ok(data),
                        None => ApiResponse::NotFound(Problem::ResourceNotFound {
                            resource: #snake_name.to_string(),
                            id: id,
                        }),
                    },
                    Err(error_msg) => {
                        eprintln!(concat!("Error deleting single [", #snake_name, "]: {}"), error_msg);
                        ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                    }
                }
            }

            router.route(&#original_struct_name::get_path_with_id(), delete(delete_single_handler))
        }
    } else {
        return_router_code()
    };

    // MARK: RouteBuilder
    quote! {
        impl RouteBuilder for #original_struct_name {
            fn add_create_single_route(router: Router) -> Router {
                #add_create_single_route_tokens
            }
            fn add_read_single_route(router: Router) -> Router {
                #add_read_single_route_tokens
            }
            fn add_update_single_route(router: Router) -> Router {
                #add_update_single_route_tokens
            }
            // fn add_replace_single_route(router: Router) -> Router {
            //     #add_replace_single_route_tokens
            // }
            fn add_delete_single_route(router: Router) -> Router {
                #add_delete_single_route_tokens
            }
        }
    }
}

fn return_router_code() -> TokenStream {
    quote! {
        router
    }
}

#[derive(Debug, Display, EnumIter)]
pub enum Routes {
    CreateSingle,
    CreateMultiple,
    ReadSingle,
    ReadMultiple,
    ReplaceSingle,
    ReplaceMultiple,
    UpdateSingle,
    UpdateMultiple,
    DeleteSingle,
    DeleteMultiple,
}

impl Routes {
    pub fn get_path(&self) -> String {
        let variant = self.to_string();
        to_kebab_case(&variant)
    }
}
