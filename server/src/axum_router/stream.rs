use std::{collections::VecDeque, path::PathBuf};

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use sanitize_filename::sanitize;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::prisma::product;

use super::AxumRouterState;

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
    let product_root_path = state
        .db
        .product()
        .find_unique(product::id::equals(product_id.to_string()))
        .exec()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get product path: {:?}", e),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Product not found".to_string(),
            )
        })?
        .path;

    let mut file_path = PathBuf::from(&product_root_path);
    paths.into_iter().for_each(|path| {
        let path = urlencoding::decode(path);
        if let Ok(path) = path {
            file_path.push(sanitize(path));
        }
    });

    Ok(ServeFile::new(file_path).oneshot(request).await)
}
