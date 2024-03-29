use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{AxumRouterState, Db};

#[cfg(not(debug_assertions))]
mod frontend;

mod middleware;
mod stream;

#[allow(clippy::let_and_return)]
pub(crate) fn mount(db: Db) -> Router<AxumRouterState> {
    let router = axum::Router::new().nest(
        "/stream",
        axum::Router::new()
            .fallback(get(stream::stream))
            .layer(from_fn_with_state(
                AxumRouterState { db },
                middleware::auth_middleware,
            )),
    );

    #[cfg(not(debug_assertions))]
    let router = router.fallback(frontend::static_handler);

    router
}
