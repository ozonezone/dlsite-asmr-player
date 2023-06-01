use std::sync::Arc;

use entity::entities::user;
use rspc::integrations::httpz::Request;
use rspc::{ErrorCode, Router};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::sync::RwLock;

use crate::config::Config;
use crate::AxumRouterState;

use self::utils::ToRspcInternalError;

mod common;
mod config;
mod product;
mod scan;
mod utils;

type RouterBuilder = rspc::RouterBuilder<RouterContext>;

pub(crate) struct RouterContext {
    pub config: Arc<RwLock<Config>>,
    pub db: DatabaseConnection,
    pub token: Option<String>,
    pub scan_status: Arc<RwLock<ScanStatus>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScanStatus {
    pub is_scanning: bool,
}

pub(crate) fn mount(
    config: Arc<RwLock<Config>>,
    db: DatabaseConnection,
) -> axum::Router<AxumRouterState> {
    let scan_status = Arc::new(RwLock::new(ScanStatus { is_scanning: false }));

    axum::Router::new().nest(
        "/rspc",
        rspc_mount()
            .endpoint(move |req: Request| {
                let token = req.query_pairs().and_then(|pairs| {
                    pairs
                        .into_iter()
                        .find(|(key, _)| key == "token")
                        .map(|(_, value)| value.to_string())
                });

                RouterContext {
                    config: config.clone(),
                    db,
                    token,
                    scan_status,
                }
            })
            .axum(),
    )
}

fn rspc_mount() -> Arc<Router<RouterContext>> {
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
                    .one(&mw.ctx.db)
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
        .build()
        .arced()
}
