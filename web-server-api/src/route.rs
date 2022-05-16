mod controllers;
use std::time::Duration;
use axum::{Json, Router};
use axum::routing::{get, post};
use crate::{IntoResponse, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::hasher;
use crate::hasher::create_hash_from;
use controllers::feature;
use controllers::async_sync;
use crate::route::controllers::create_project::create_project;
use crate::route::controllers::{create_project, healthz};

pub fn route() -> Router {
    Router::new()
        .route("/healthz", get(healthz::healthz))
        .route("/create_project", post(create_project::create_project))
        .route("/async_sync", get(async_sync::async_sync))
        .route("/future", get(feature::future))
}
