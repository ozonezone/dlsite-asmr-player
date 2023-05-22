use axum::{
    http::StatusCode,
    routing::{get, get_service},
};
use rspc::Config;
use tower_http::services::{ServeDir, ServeFile};

struct Ctx {}

#[tokio::main]
async fn main() {
    let router = rspc::Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings("../client/bindings/bindings.ts"))
        .query("hello", |t| {
            t(|_, _: ()| {
                dbg!("hello");
                "Hello"
            })
        })
        .build()
        .arced();

    let app = axum::Router::new().nest("/rspc", router.endpoint(move || Ctx {}).axum());

    #[cfg(not(debug_assertions))]
    let app = app.fallback(
        get_service(
            ServeDir::new("assets")
                // respond with `not_found.html` for missing files
                .fallback(ServeFile::new("assets/404.html")),
        )
        .handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }),
    );

    let addr = "[::]:4000".parse::<std::net::SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("listening on http://{}/rspc/version", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
