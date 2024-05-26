use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct Price {
    price: f64,
}

#[tokio::main]
async fn main() {
    let state = Mutex::new(Price { price: 0.0 });

    let app = Router::new()
        .route("/price", get(|| async move {
            let state = state.lock().unwrap();
            state.price.into_response()
        }))
        .route("/price", put(|| async move {
            let state = state.lock().unwrap();
            let new_price: Price = serde_json::from_str("{}").unwrap();
            state.price = new_price.price;
            StatusCode::OK.into_response()
        }))
        .route("/price", delete(|| async move {
            let state = state.lock().unwrap();
            state.price = 0.0;
            StatusCode::OK.into_response()
        }));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

