use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::modules::user::resource;

pub fn api() -> Router {
    Router::new()
        .route("/", get(resource::sys_user::list))
        .route("/login", post(resource::sys_user::login))
}