use proc_macro2::TokenStream;
use quote::quote;
use strum::EnumIter;

use crate::StructNames;
pub fn get_route_builder(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
        impl RouteBuilder for #original_struct_name {
            fn add_get_single_route(router: Router) -> Router {
                // TODO: return router if create_one is NOT generated...

                // TODO: return following if create_one is generated:
                async fn get_single_handler(Path(id): Path<<#original_struct_name as Endpoint>::Id>) -> ApiResponse<#original_struct_name> {
                    let result = #original_struct_name::get_single(&id).await;

                    match result {
                        Ok(option) => match option {
                            Some(data) => ApiResponse::Ok(data),
                            None => ApiResponse::NotFound(Problem::ResourceNotFound {
                                resource: #snake_name.to_string(),
                                id: id,
                            }),
                        },
                        Err(error_msg) => {
                            eprintln!(concat!("Error getting single [", #snake_name, "]: {}"), error_msg);
                            ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
                        }
                    }
                }

                router.route(&#original_struct_name::get_path_with_id(), get(get_single_handler))
            }
            fn add_create_single_route(router: Router) -> Router {
                // TODO: return router if create_one is NOT generated...

                // TODO: return following if create_one is generated:
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
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum Routes {
    CreateOne,
    CreateMany,
    ReadOne,
    ReadMany,
    ReplaceOne,
    ReplaceMany,
    UpdateOne,
    UpdateMany,
    DeleteOne,
    DeleteMany,
}

impl Routes {
    pub fn get_path(&self) -> &'static str {
        match self {
            Routes::CreateOne => "create_one",
            Routes::CreateMany => "create_many",
            Routes::ReadOne => "read_one",
            Routes::ReadMany => "read_many",
            Routes::ReplaceOne => "replace_one",
            Routes::ReplaceMany => "replace_many",
            Routes::UpdateOne => "update_one",
            Routes::UpdateMany => "update_many",
            Routes::DeleteOne => "delete_one",
            Routes::DeleteMany => "delete_many",
        }
    }
}
