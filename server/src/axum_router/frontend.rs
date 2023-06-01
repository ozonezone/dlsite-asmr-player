use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../client/dist/"]
struct Assets;

pub(super) async fn static_handler(uri: Uri) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path().trim_start_matches('/');

    if let Some(content) = Assets::get(path) {
        let body = boxed(Full::from(content.data));
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        Ok(Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(body)
            .unwrap())
    } else {
        index_html().await
    }
}

async fn index_html() -> Result<Response, StatusCode> {
    if let Some(content) = Assets::get("index.html") {
        let body = boxed(Full::from(content.data));

        Ok(Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(body)
            .unwrap())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
