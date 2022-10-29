use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use std::{
    net::SocketAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .merge(SpaRouter::new("/assets", "assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    let mut app = rustle::compile_file_to_string(Path::new("./static/app.svelte")).unwrap();
    app = app.replace(
        "let counter = 0;",
        &format!(
            "let counter = {};",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ),
    );

    std::fs::write("./assets/app.js", app.as_bytes()).unwrap();

    Html(std::fs::read_to_string("./assets/index.html").unwrap())
}
