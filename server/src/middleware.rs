use crate::{cornucopia::queries::user::get_user, stream::AxumRouterState};
use axum::{
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AuthQuery {
    token: Option<String>,
}
pub(crate) async fn auth_middleware<B>(
    State(state): State<AxumRouterState>,
    Query(query): Query<AuthQuery>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(token) = query.token {
        let client = state
            .pool
            .get()
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
        if token
            == get_user()
                .bind(&client, &1)
                .one()
                .await
                .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?
                .password
        {
            let response = next.run(request).await;
            return Ok(response);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
