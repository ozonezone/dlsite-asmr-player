use rspc::{integrations::httpz::Request, ErrorCode};
use std::sync::{Arc, Mutex};

#[cfg(not(debug_assertions))]
use axum::{
    http::StatusCode,
    routing::{get, get_service},
};
#[cfg(not(debug_assertions))]
use tower_http::services::{ServeDir, ServeFile};

use crate::config::Config;

mod config;
#[allow(warnings, unused)]
mod prisma;

pub(crate) struct Ctx {
    pub config: Arc<Mutex<Config>>,
    pub db: Arc<prisma::PrismaClient>,
    pub token: Option<String>,
}

#[tokio::main]
async fn main() {
    let config = Arc::new(Mutex::new(Config {
        password: "password".to_string(),
    }));
    let db = Arc::new(prisma::new_client().await.unwrap());

    let router = rspc::Router::<Ctx>::new()
        .config(rspc::Config::new().export_ts_bindings("../client/bindings/bindings.ts"))
        .query("ping", |t| t(|_, _: ()| "ping"))
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                dbg!(&mw.ctx.token);
                match &mw.ctx.token {
                    Some(token) => {
                        if token == &mw.ctx.config.lock().unwrap().password {
                            Ok(mw)
                        } else {
                            Err(rspc::Error::new(
                                ErrorCode::Unauthorized,
                                "Unauthorized".into(),
                            ))
                        }
                    }
                    None => Err(rspc::Error::new(
                        ErrorCode::Unauthorized,
                        "Unauthorized".into(),
                    )),
                }
            })
        })
        .query("ping_auth", |t| t(|_, _: ()| "ping_auth"))
        .build()
        .arced();

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

                Ctx {
                    config: config.clone(),
                    db: db.clone(),
                    token,
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
}
