mod controllers;

use std::sync::Arc;
use crate::hasher;
use crate::modules;
use crate::hasher::create_hash_from;
use crate::route::controllers::create_project::create_project;
use crate::route::controllers::{create_project, healthz};
use crate::{IntoResponse, StatusCode};
use axum::routing::{get, post};
use axum::{AddExtension, Json, Router};
use controllers::async_sync;
use controllers::feature;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use axum::extract::Extension;
use axum::http::Extensions;
use tokio::task;
use crate::modules::Modules;


pub fn route(modules: Arc<Modules>) -> Router {
    Router::new()
        .route("/healthz", get(healthz::healthz))
        .route("/create_project", post(create_project::create_project))
        .route("/async_sync", get(async_sync::async_sync))
        .route("/future", get(feature::future))
        // 参考：https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
        .layer(Extension(modules))
}
