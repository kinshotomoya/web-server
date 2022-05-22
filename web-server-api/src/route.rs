mod controllers;

use std::sync::Arc;
use crate::route::controllers::{create_project, healthz};
use axum::routing::{get, post};
use axum::Router;
use controllers::async_sync;
use controllers::feature;
use axum::extract::Extension;
use crate::modules::Modules;


pub fn route(modules: Arc<Modules>) -> Router {
    Router::new()
        .route("/healthz", get(healthz::healthz))
        .route("/create_project", post(create_project::create_project))
        .route("/async_sync", get(async_sync::async_sync))
        .route("/future", get(feature::future))
        // 参考：https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
        .layer(Extension(modules))
        // TODO: 各ハンドラーからのエラーはどうハンドリングするべき？？
}
