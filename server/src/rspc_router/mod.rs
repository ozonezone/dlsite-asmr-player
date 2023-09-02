use std::sync::Arc;

use rspc::integrations::httpz::Request;
use rspc::{ErrorCode, Router};
use tokio::sync::RwLock;

use crate::config::Config;
use crate::prisma::user;
use crate::{AxumRouterState, Db};

use self::utils::ToRspcInternalError;

mod config;
mod product;
mod remote;
mod scan;
mod utils;

type RouterBuilder = rspc::RouterBuilder<RouterContext>;

pub(crate) struct UnauthenticatedRouterContext {
    pub config: Arc<RwLock<Config>>,
    pub db: Db,
    pub token: Option<String>,
    pub scan_status: Arc<RwLock<ScanStatus>>,
}

pub(crate) struct RouterContext {
    pub config: Arc<RwLock<Config>>,
    pub db: Db,
    pub user_id: i32,
    pub scan_status: Arc<RwLock<ScanStatus>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScanStatus {
    pub is_scanning: bool,
}

pub(crate) fn mount(config: Arc<RwLock<Config>>, db: Db) -> axum::Router<AxumRouterState> {
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

                UnauthenticatedRouterContext {
                    config: config.clone(),
                    db,
                    token,
                    scan_status,
                }
            })
            .axum(),
    )
}

fn rspc_mount() -> Arc<Router<UnauthenticatedRouterContext>> {
    let config = rspc::Config::new()
        .set_ts_bindings_header("/* eslint-disable */")
        .export_ts_bindings(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../client/src/bindings/bindings.ts"),
        );

    rspc::Router::<UnauthenticatedRouterContext>::new()
        .config(config)
        .query("ping", |t| t(|_, _: ()| "pong"))
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                let user = &mw
                    .ctx
                    .db
                    .user()
                    .find_unique(user::id::equals(1))
                    .exec()
                    .await
                    .to_rspc_internal_error("Failed to get user data")?
                    .ok_or_else(|| {
                        rspc::Error::new(
                            ErrorCode::InternalServerError,
                            "No admin user".to_string(),
                        )
                    })?;

                match &mw.ctx.token {
                    Some(token) => {
                        if *token == user.password {
                            let new_ctx = RouterContext {
                                config: mw.ctx.config.clone(),
                                db: mw.ctx.db.clone(),
                                user_id: user.id,
                                scan_status: mw.ctx.scan_status.clone(),
                            };
                            Ok(mw.with_ctx(new_ctx))
                        } else {
                            Err(rspc::Error::new(
                                ErrorCode::Unauthorized,
                                "Unauthorized".into(),
                            ))
                        }
                    }
                    None => Err(rspc::Error::new(
                        ErrorCode::BadRequest,
                        "Toke is not specified".into(),
                    )),
                }
            })
        })
        .query("ping_auth", |t| t(|_, _: ()| "authed!"))
        .merge("config.", config::mount())
        .merge("scan.", scan::mount())
        .merge("product.", product::mount())
        .merge("remote.", remote::mount())
        .build()
        .arced()
}
