mod controllers;
use crate::hasher;
use crate::hasher::create_hash_from;
use crate::route::controllers::create_project::create_project;
use crate::route::controllers::{create_project, healthz};
use crate::{IntoResponse, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use controllers::async_sync;
use controllers::feature;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::task;


pub fn route() -> Router {


    Router::new()
        .route("/healthz", get(healthz::healthz))
        .route("/create_project", post(create_project::create_project))
        .route("/async_sync", get(async_sync::async_sync))
        .route("/future", get(feature::future))
}
