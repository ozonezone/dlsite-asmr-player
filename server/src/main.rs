use anyhow::Result;
use rspc::integrations::httpz::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[cfg(not(debug_assertions))]
use axum::{
    http::StatusCode,
    routing::{get, get_service},
};
#[cfg(not(debug_assertions))]
use tower_http::services::{ServeDir, ServeFile};

use crate::{config::Config, router::RouterContext};

mod config;
#[allow(warnings, unused)]
mod prisma;
mod router;
mod scan;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("dlsite=debug,server=debug")
        .init();

    let config = match Config::from_file().await {
        Ok(config) => config,
        Err(e) => {
            info!("Failed to load config file: {:?}", e);
            info!("Creating default config file");
            let config = Config::default();
            let config_path = config.write_to_file().await?;
            let config_path = config_path.to_str().unwrap_or("");
            info!("Config file created: {}", config_path);
            config
        }
    };

    let config = Arc::new(RwLock::new(config));

    let client = prisma::new_client().await.unwrap();

    #[cfg(debug_assertions)]
    {
        info!("Migrating database accepting data loss (dev mode)");
        client._db_push().accept_data_loss().await?;
    }

    #[cfg(not(debug_assertions))]
    {
        info!("Migrating database");
        client._migrate_deploy().await?;
    }

    let db = Arc::new(client);
    let router = router::mount();
    let scan_status = Arc::new(RwLock::new(router::ScanStatus {
        is_scanning: false,
        current: None,
        total: None,
    }));

    let app = axum::Router::new().nest(
        "/rspc",
        router
            .endpoint(move |req: Request| {
                let token = req.query_pairs().and_then(|pairs| {
                    pairs
                        .into_iter()
                        .find(|(key, _)| key == "token")
                        .map(|(_, value)| value.to_string())
                });

                RouterContext {
                    config: config.clone(),
                    db: db.clone(),
                    token,
                    scan_status,
                }
            })
            .axum(),
    );

    #[cfg(not(debug_assertions))]
    let app = app.fallback(
        get_service(
            ServeDir::new("assets")
                // respond with `not_found.html` for missing files
                .fallback(ServeFile::new("assets/404.html")),
        )
        .handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }),
    );

    let addr = "[::]:4000".parse::<std::net::SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("listening on http://{}/rspc", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
