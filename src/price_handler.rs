use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
struct Price {
    value: u32,
}

type PriceStore = Mutex<Option<Price>>;

pub async fn price_handler(State(store): State<PriceStore>) -> impl IntoResponse {
    match store.lock().unwrap().as_ref() {
        Some(price) => {
            let response = Json(price);
            (StatusCode::OK, response).into_response()
        }
        None => (StatusCode::NOT_FOUND, "").into_response(),
    }
}

pub async fn set_price(Json(price): Json<Price>, State(store): State<PriceStore>) -> impl IntoResponse {
    let mut store = store.lock().unwrap();
    *store = Some(price);
    (StatusCode::OK, "").into_response()
}

pub async fn delete_price(State(store): State<PriceStore>) -> impl IntoResponse {
    let mut store = store.lock().unwrap();
    *store = None;
    (StatusCode::OK, "").into_response()
}

