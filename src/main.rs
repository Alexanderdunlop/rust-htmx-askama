
use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;

// `Deserialize` need be implemented to use with `Query`
#[derive(Deserialize)]
struct  RangeParameter {
    start: usize,
    end: usize,
}

async fn handler(Query(range): Query<RangeParameter>) -> Html<String> {
    // Generate a random rumber in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);
    
    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
