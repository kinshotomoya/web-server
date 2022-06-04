mod controllers;

use crate::modules::Modules;
use crate::route::controllers::execute_actor::execute_actor;
use crate::route::controllers::{create_project, healthz};
use axum::error_handling::HandleError;
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::Router;
use controllers::async_sync;
use controllers::feature;
use std::sync::Arc;
use tracing_subscriber::fmt::layer;

pub fn route(modules: Arc<Modules>) -> Router {
    Router::new()
        .route("/healthz", get(healthz::healthz))
        .route("/create_project", post(create_project::create_project))
        .route("/async_sync", get(async_sync::async_sync))
        .route("/future", get(feature::future))
        .route("/actor/:project_id", get(execute_actor))
        // 参考：https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
        .layer(Extension(modules))
    // TODO: 各ハンドラーからのエラーはどうハンドリングするべき？？
}
