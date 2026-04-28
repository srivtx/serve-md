use axum::{routing::get, Router};
use std::sync::Arc;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

use crate::handlers::{handle_request, handle_root};
use crate::scan::FileIndex;

pub struct AppState {
    pub index: FileIndex,
}

pub fn create_app(index: FileIndex) -> Router {
    let state = Arc::new(AppState { index });

    Router::new()
        .route("/", get(handle_root))
        .route("/*path", get(handle_request))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}
