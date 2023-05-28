use anyhow::Result;
use axum::{middleware::from_fn_with_state, routing::get};
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

use crate::{
    config::Config,
    cornucopia::queries::user::{exist_user, insert_user},
    middleware::auth_middleware,
    router::RouterContext,
    stream::AxumRouterState,
};

mod config;
mod cornucopia;
mod db;
mod middleware;
mod pool;
mod router;
mod scan;
mod stream;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let config = match Config::from_file().await {
        Ok(config) => config,
        Err(e) => {
            info!("Failed to load config file: {:?}", e);
            info!("Creating default config file");
            let config = Config::default();
            let config_path = config.write_to_file().await?;
            let config_path = config_path.to_str().unwrap_or("");
            info!("Config file created to: {}", config_path);
            config
        }
    };

    let config = Arc::new(RwLock::new(config));

    let pool = pool::create_pool().await?;

    let client = pool.get().await?;
    if exist_user().bind(&client, &1).one().await? == 0 {
        info!("Creating default admin user with password 'password'");
        insert_user().bind(&client, &1, &"password").await?;
    }

    let router = router::mount();
    let scan_status = Arc::new(RwLock::new(router::ScanStatus { is_scanning: false }));

    let app = axum::Router::new()
        .nest(
            "/stream",
            axum::Router::new()
                .fallback(get(stream::stream))
                .layer(from_fn_with_state(
                    AxumRouterState {
                        config: config.clone(),
                        pool: pool.clone(),
                    },
                    auth_middleware,
                )),
        )
        .with_state(AxumRouterState {
            config: config.clone(),
            pool: pool.clone(),
        })
        .nest(
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
                        pool,
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

    let addr = "[::]:14567".parse::<std::net::SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("listening on http://{}/rspc", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
