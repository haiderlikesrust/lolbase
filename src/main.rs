use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use lolbase::{
    db::Database,
    web::{self, State},
};

#[tokio::main]
async fn main() {
    let database = Database::new("postgres://haider@localhost:5432/lolbase").await;
    let state = Arc::new(State { database });
    let router = Router::new()
        .route("/api/get/records", get(web::routes::get_all_records))
        .route("/api/new/record", post(web::routes::create_record))
        .route("/api/new/column", post(web::routes::create_column))
        .route("/api/new/row", post(web::routes::create_row))
        .layer(Extension(state));
    Server::bind(&"127.0.0.1:7000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
