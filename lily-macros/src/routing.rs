use proc_macro2::TokenStream;
use quote::quote;
use strum::EnumIter;

use crate::StructNames;

pub fn get_endpoint_handler_code(
    struct_names: &StructNames,
    route: &Routes,
    is_active: bool,
) -> TokenStream {
    let fn_signature = route.get_fn_signature(struct_names);
    let fn_body = if is_active {
        route.get_fn_body(struct_names)
    } else {
        get_endpoint_not_found_fn_body()
    };
    quote! {
        #fn_signature {
            #fn_body
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
    fn get_fn_signature(&self, struct_names: &StructNames) -> TokenStream {
        match self {
            Routes::CreateOne => get_create_one_fn_signature(struct_names),
            Routes::CreateMany => get_create_many_fn_signature(struct_names),
            Routes::ReadOne => get_read_one_fn_signature(struct_names),
            Routes::ReadMany => get_read_many_fn_signature(struct_names),
            Routes::ReplaceOne => get_replace_one_fn_signature(struct_names),
            Routes::ReplaceMany => get_replace_many_fn_signature(struct_names),
            Routes::UpdateOne => get_update_one_fn_signature(struct_names),
            Routes::UpdateMany => get_update_many_fn_signature(struct_names),
            Routes::DeleteOne => get_delete_one_fn_signature(struct_names),
            Routes::DeleteMany => get_delete_many_fn_signature(struct_names),
        }
    }
    fn get_fn_body(&self, struct_names: &StructNames) -> TokenStream {
        match self {
            Routes::CreateOne => get_create_one_fn_body(struct_names),
            Routes::CreateMany => get_endpoint_not_found_fn_body(),
            Routes::ReadOne => get_read_one_fn_body(struct_names),
            Routes::ReadMany => get_read_many_fn_body(struct_names),
            Routes::ReplaceOne => get_replace_one_fn_body(struct_names),
            Routes::ReplaceMany => get_endpoint_not_found_fn_body(),
            Routes::UpdateOne => get_update_one_fn_body(struct_names),
            Routes::UpdateMany => get_endpoint_not_found_fn_body(),
            Routes::DeleteOne => get_delete_one_fn_body(struct_names),
            Routes::DeleteMany => get_endpoint_not_found_fn_body(),
        }
    }
}

fn get_endpoint_not_found_fn_body() -> TokenStream {
    quote! {
        ApiResponse::NotFound(Problem::EndpointNotFound)
    }
}

// MARK: Create One
fn get_create_one_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn create_one_handler(
            Json(payload): Json<Self::PostPayload>
        ) -> ApiResponse<#original_struct_name>
    }
}
fn get_create_one_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
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

// MARK: Create Many
fn get_create_many_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn create_many_handler(
            Json(payload): Json<Self::PostPayload>
        ) -> ApiResponse<#original_struct_name>
    }
}

// MARK: Read One
fn get_read_one_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn read_one_handler(
            Path(id): Path<String>
        ) -> ApiResponse<#original_struct_name>
    }
}
fn get_read_one_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
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

// MARK: Read Many
fn get_read_many_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn read_many_handler() -> ApiResponse<Vec<#original_struct_name>>
    }
}
fn get_read_many_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
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

// MARK: Update One
fn get_update_one_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let patch_payload_name = &struct_names.patch_payload_name;
    quote! {
        async fn update_one_handler(
            Path(id): Path<String>,
            Json(payload): Json<#patch_payload_name>,
        ) -> ApiResponse<#original_struct_name>
    }
}
fn get_update_one_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
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

// MARK: Update Many
fn get_update_many_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn update_many_handler(
            Json(payload): Json<Self::PatchPayload>
        ) -> ApiResponse<#original_struct_name>
    }
}

// MARK: Replace One
fn get_replace_one_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let post_payload_name = &struct_names.post_payload_name;
    quote! {
        async fn replace_one_handler(
            Path(id): Path<String>,
            Json(payload): Json<#post_payload_name>,
        ) -> ApiResponse<#original_struct_name>
    }
}
fn get_replace_one_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
        let result: Result<#original_struct_name, Error> = #original_struct_name::replace_one(&id, &payload);

        match result {
            Ok(data) => ApiResponse::Ok(data),
            Err(error_msg) => {
                eprintln!(concat!("Error replacing one [", #snake_name, "]: {}"), error_msg);
                ApiResponse::Erroneous::<#original_struct_name>(Problem::InternalError)
            }
        }
    }
}

// MARK: Replace Many
fn get_replace_many_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn replace_many_handler(
            Json(payload): Json<Self::PostPayload>
        ) -> ApiResponse<#original_struct_name>
    }
}

// MARK: Delete One
fn get_delete_one_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn delete_one_handler(
            Path(id): Path<String>
        ) -> ApiResponse<#original_struct_name>
    }
}
fn get_delete_one_fn_body(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    let snake_name = &struct_names.snake_case;
    quote! {
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

// MARK: Delete Many
fn get_delete_many_fn_signature(struct_names: &StructNames) -> TokenStream {
    let original_struct_name = &struct_names.original;
    quote! {
        async fn delete_many_handler(
            Path(id): Path<String>
        ) -> ApiResponse<#original_struct_name>
    }
}
