use std::sync::Arc;

use entity::entities::user;
use rspc::{ErrorCode, Router};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::sync::RwLock;

use crate::config::Config;

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
    pub pool: DatabaseConnection,
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
                let password = user::Entity::find_by_id(1)
                    .one(&mw.ctx.pool)
                    .await
                    .to_rspc_internal_error("Failed to get user data")?
                    .ok_or_else(|| {
                        rspc::Error::new(
                            ErrorCode::InternalServerError,
                            "No admin user".to_string(),
                        )
                    })?
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
