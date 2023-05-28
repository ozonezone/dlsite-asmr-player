use std::sync::Arc;

use deadpool_postgres::Pool;
use rspc::{ErrorCode, Router};
use tokio::sync::RwLock;

use crate::{config::Config, cornucopia::queries::user::get_user};

use self::utils::ToRspcError;

mod circle;
mod common;
mod config;
mod product;
mod scan;
mod utils;

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
                .join("../client/src/bindings/bindings.ts"),
        );

    rspc::Router::<RouterContext>::new()
        .config(config)
        .query("ping", |t| t(|_, _: ()| "pong"))
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                let client = mw
                    .ctx
                    .pool
                    .get()
                    .await
                    .to_rspc_internal_error("Cannot connect db")?;
                let password = get_user()
                    .bind(&client, &1)
                    .one()
                    .await
                    .to_rspc_internal_error("Failed to get user data")?
                    .password;
                match &mw.ctx.token {
                    Some(token) => {
                        if token == &password {
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
        .merge("product.", product::mount())
        .merge("circle.", circle::mount())
        .build()
        .arced()
}
