use quote::format_ident;
use syn::ItemStruct;

pub struct StructNames {
    pub original: syn::Ident,
    pub snake_case: String,
    pub create_payload_name: syn::Ident,
    pub update_payload_name: syn::Ident,
}
impl From<&ItemStruct> for StructNames {
    fn from(ast: &ItemStruct) -> Self {
        let struct_name: syn::Ident = ast.ident.clone();
        let snake_case = to_snake_case(&struct_name.to_string());
        StructNames {
            create_payload_name: format_ident!("Create{}", &struct_name),
            update_payload_name: format_ident!("Update{}", &struct_name),
            original: struct_name,
            snake_case: snake_case,
        }
    }
}

/// Converts a string from lowerCamelCase to snake_case
///
/// # Examples
/// ```
/// let snake_case: String = to_snake_case("lowerCamelCase");
/// ```
///
/// # Note
/// This function was created by Claude Code
/// TODO: Verify that it's doing what it should do
pub fn to_snake_case(input: &str) -> String {
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

/// Converts a string from lowerCamelCase to kebab case
///
/// # Examples
/// ```
/// let kebab_case: String = to_kebab_case("lowerCamelCase");
/// ```
pub fn to_kebab_case(input: &str) -> String {
    to_snake_case(input).replace("_", "-")
}

/// Checks wether a value of type syn::data::Field::Type is of type Option
///
/// # Examples
/// ```
/// if is_option(&field.ty) {}
/// ```
pub fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}
