
use axum::{response::Html, routing::get, Router};

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {}

async fn handler() -> Html<String> {
    return Html(MyTemplate {}.render().unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
