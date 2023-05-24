use std::sync::Arc;

use deadpool_postgres::Pool;
use rspc::{ErrorCode, Router};
use tokio::sync::RwLock;
use tracing::info;

use crate::config::Config;

mod config;
mod scan;

type RouterBuilder = rspc::RouterBuilder<RouterContext>;

pub(crate) struct RouterContext {
    pub config: Arc<RwLock<Config>>,
    pub pool: Pool,
    pub token: Option<String>,
    pub scan_status: Arc<RwLock<ScanStatus>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScanStatus {
    pub is_scanning: bool,
}

pub(crate) fn mount() -> Arc<Router<RouterContext>> {
    let config = rspc::Config::new()
        .set_ts_bindings_header("/* eslint-disable */")
        .export_ts_bindings(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../client/bindings/bindings.ts"),
        );

    rspc::Router::<RouterContext>::new()
        .config(config)
        .query("ping", |t| t(|_, _: ()| "pong"))
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                match &mw.ctx.token {
                    Some(token) => {
                        if token == &mw.ctx.config.read().await.password {
                            info!("Authorized");
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
        .query("ping_auth", |t| t(|_, _: ()| "authed!"))
        .merge("config.", config::mount())
        .merge("scan.", scan::mount())
        .build()
        .arced()
}
