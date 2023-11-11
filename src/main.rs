mod templates;

use axum::{routing::get, Router};
use templates::IndexTemplate;

async fn index() -> IndexTemplate {
    return IndexTemplate {};
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
