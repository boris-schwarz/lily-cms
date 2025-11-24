use crate::StructNames;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use strum::EnumIter;

pub fn get_route_builder(
    struct_names: &StructNames,
    enabled_actions: &HashSet<String>,
) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;

    // MARK: Create Single
    let add_create_single_route_code: TokenStream = if enabled_actions.contains("create_single") {
        quote! {
            async fn create_single_handler(Json(payload): Json<<#original_struct_name as Endpoint>::PostPayload>) -> ApiResponse<#original_struct_name> {
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
    let add_read_single_route_code: TokenStream = if enabled_actions.contains("read_single") {
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

    // MARK: RouteBuilder
    quote! {
        impl RouteBuilder for #original_struct_name {
            fn add_create_single_route(router: Router) -> Router {
                #add_create_single_route_code
            }
            fn add_read_single_route(router: Router) -> Router {
                #add_read_single_route_code
            }
        }
    }
}

fn return_router_code() -> TokenStream {
    quote! {
        router
    }
}

#[derive(Debug, EnumIter)]
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
    pub fn get_path(&self) -> &'static str {
        match self {
            Routes::CreateSingle => "create_single",
            Routes::CreateMultiple => "create_multiple",
            Routes::ReadSingle => "read_single",
            Routes::ReadMultiple => "read_multiple",
            Routes::ReplaceSingle => "replace_single",
            Routes::ReplaceMultiple => "replace_multiple",
            Routes::UpdateSingle => "update_single",
            Routes::UpdateMultiple => "update_multiple",
            Routes::DeleteSingle => "delete_single",
            Routes::DeleteMultiple => "delete_multiple",
        }
    }
}
