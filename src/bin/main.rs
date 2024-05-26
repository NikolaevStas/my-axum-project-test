use axum::{routing::post, Router};
use price_handler::*;

#[tokio::main]
async fn main() {
    let store = Mutex::new(None);
    let app = Router::new()
        .route("/price", routing::get(price_handler).post(set_price))
        .route("/price", routing::delete(delete_price));

    let addr = spawn_app().await.expect("не удалось запустить сервер");

    tracing::info!("Запущен на http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("сервер не может быть запущен");
}

