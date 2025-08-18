// MARK: Global

/// Reads any sized type from the environment variables
pub trait FromEnv: Sized {
    fn from_env(key: &str, fallback: Self) -> Self;
}

/// Implementation for `FromEnv` for the type `String`
impl FromEnv for String {
    fn from_env(key: &str, fallback: Self) -> Self {
        std::env::var(key).unwrap_or(fallback)
    }
}

/// Implementation for `FromEnv` for the type `i32`
impl FromEnv for i32 {
    fn from_env(key: &str, fallback: Self) -> Self {
        std::env::var(key)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(fallback)
    }
}

// MARK: Specific

/// Gets Lily CMS' host from the environment variables
pub fn get_lily_host() -> String {
    String::from_env("LILY_HOST", "0.0.0.0".to_string())
}

/// Gets Lily CMS' port from the environment variables
pub fn get_lily_port() -> i32 {
    i32::from_env("LILY_PORT", 5173)
}

/// Gets Lily CMS' name from the environment variables
pub fn get_lily_name() -> String {
    String::from_env("CARGO_PKG_NAME", "app".to_string())
}

/// Gets Lily CMS' version from the environment variables
pub fn get_lily_version() -> String {
    let version = String::from_env("CARGO_PKG_VERSION", "0.0.0".to_string());
    format!("v{}", version)
}

/// Gets Lily CMS' product_id (name and version) from the environment variables
pub fn get_lily_product_id() -> String {
    let name = get_lily_name();
    let version = get_lily_version();
    format!("{} {}", name, version)
}
