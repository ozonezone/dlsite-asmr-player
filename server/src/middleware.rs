use crate::stream::AxumRouterState;
use axum::{
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use entity::entities::user;
use sea_orm::EntityTrait;
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
        if token
            == user::Entity::find_by_id(1)
                .one(&state.pool)
                .await
                .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?
                .password
        {
            let response = next.run(request).await;
            return Ok(response);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
