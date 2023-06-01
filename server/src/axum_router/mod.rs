use axum::{middleware::from_fn_with_state, routing::get, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{config::Config, AxumRouterState};

// #[cfg(not(debug_assertions))]
mod frontend;

mod middleware;
mod stream;

pub(crate) fn mount(
    config: Arc<RwLock<Config>>,
    db: DatabaseConnection,
) -> Router<AxumRouterState> {
    let router = axum::Router::new().nest(
        "/stream",
        axum::Router::new()
            .fallback(get(stream::stream))
            .layer(from_fn_with_state(
                AxumRouterState { config, pool: db },
                middleware::auth_middleware,
            )),
    );

    // #[cfg(not(debug_assertions))]
    let router = router.fallback(frontend::static_handler);

    router
}
