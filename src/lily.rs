pub mod core;
pub mod types;

use dotenv::dotenv;

/// Initialize should only be invoked once
pub fn init() {
    // load /.env into environment variables
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();
}

pub struct Cms {
    host: String,
    port: i32,
}

impl Cms {
    pub fn new() -> core::cms_builder::CmsBuilder {
        core::cms_builder::CmsBuilder::new()
    }

    pub fn get_address(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
