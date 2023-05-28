use std::{collections::VecDeque, path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use sanitize_filename::sanitize;
use tokio::sync::RwLock;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{config::Config, cornucopia::queries::product::get_product_path};

#[derive(Clone)]
pub(crate) struct AxumRouterState {
    pub config: Arc<RwLock<Config>>,
    pub pool: Pool,
}

pub(super) async fn stream(
    State(state): State<AxumRouterState>,
    request: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut paths = request.uri().path().split('/').collect::<VecDeque<_>>();
    if paths.len() < 3 {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid request".to_string(),
        ));
    }
    paths.pop_front();
    let product_id = paths.pop_front().unwrap();
    let client = state.pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get client from pool: {:?}", e),
        )
    })?;
    let product_root_path = get_product_path()
        .bind(&client, &product_id)
        .one()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get product path: {:?}", e),
            )
        })?;

    let mut file_path = PathBuf::from(&product_root_path);
    paths.into_iter().for_each(|path| {
        let path = urlencoding::decode(path);
        if let Ok(path) = path {
            file_path.push(sanitize(path));
        }
    });

    Ok(ServeFile::new(file_path).oneshot(request).await)
}
