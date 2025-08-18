use super::env;
use super::welcome;
use crate::lily::Cms;
use axum::{routing::get, Router};

pub struct CmsBuilder {
    host: Option<String>,
    port: Option<i32>,
}

impl CmsBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
        }
    }

    #[allow(dead_code)]
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    #[allow(dead_code)]
    pub fn port(mut self, port: i32) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn serve(
        self,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>
    {
        let host: String = self.host.unwrap_or(env::get_lily_host());
        let port: i32 = self.port.unwrap_or(env::get_lily_port());
        let addr: String = format!("{}:{}", host, port);
        let cms: Cms = Cms { port, host };

        let app: Router = Router::new().route("/", get(root));
        let app = app.merge(crate::lily::types::content::get_routes());
        welcome::cli_startup_message(&cms);

        async move {
            let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(&addr).await
            {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to bind to {}: {}", addr, e);
                    return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
                }
            };
            axum::serve(listener, app)
                .await
                .map_err(|e| Box::new(e) as _)
        }
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Lily!"
}
