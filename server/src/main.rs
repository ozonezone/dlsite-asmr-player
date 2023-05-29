use anyhow::Result;
use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    middleware::from_fn_with_state,
    response::{IntoResponse, Response},
    routing::get,
};
use rspc::integrations::httpz::Request;
use rust_embed::RustEmbed;
use std::{ops::DerefMut, sync::Arc};
use tokio::{signal, sync::RwLock};
use tracing::info;

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

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[derive(RustEmbed)]
#[folder = "../client/dist/"]
struct Assets;

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

    let mut client = pool.get().await?;

    info!("Running database migrations");
    embedded::migrations::runner()
        .run_async(client.deref_mut().deref_mut())
        .await?;
    info!("Database migrations completed");

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

    // #[cfg(not(debug_assertions))]
    let app = app.fallback(static_handler);

    let addr = "[::]:14567".parse::<std::net::SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("listening on http://{}/rspc", addr);
    let axum_task = axum::Server::bind(&addr).serve(app.into_make_service());

    tokio::select! {
        _ = signal::ctrl_c() => {
                info!("Ctrl-C received, shutting down");
        },
        _ = axum_task => {},
    }

    Ok(())
}

async fn static_handler(uri: Uri) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path().trim_start_matches('/');

    if let Some(content) = Assets::get(path) {
        let body = boxed(Full::from(content.data));
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        Ok(Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(body)
            .unwrap())
    } else {
        index_html().await
    }
}

async fn index_html() -> Result<Response, StatusCode> {
    if let Some(content) = Assets::get("index.html") {
        let body = boxed(Full::from(content.data));

        Ok(Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(body)
            .unwrap())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
