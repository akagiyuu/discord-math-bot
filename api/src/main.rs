use axum::{extract::Query, routing::get, Router};
use mathematica::OutputFormat;
use std::collections::HashMap;

async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    let expression = params.get("expr").unwrap();
    mathematica::eval(expression.clone(), OutputFormat::Plaintext)
}

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(query))
        .route("/help", get(|| async { "https://reference.wolfram.com" }));

    axum::Server::bind(&ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
