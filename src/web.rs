mod middleware;

use std::sync::Arc;

use poem::listener::TcpListener;
use poem::middleware::CatchPanic;
use poem::web::Json;
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};

use crate::prelude::*;
use crate::web::middleware::*;
use crate::Database;

pub async fn run(endpoint: String, db: Arc<Database>) -> Result {
    let app = Route::new()
        .at("/health", get(health))
        .data(db)
        .with(CatchPanic::new())
        .with(ErrorMiddleware);
    Server::new(TcpListener::bind(endpoint)).run(app).await?;
    Ok(())
}

#[handler]
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
