use std::sync::Arc;
use axum::extract::Extension;
use crate::{IntoResponse, Modules, StatusCode};
use axum::Json;
use serde::{Deserialize, Serialize};

// TODO: IntoResponse ???
pub async fn create_project(extension: Extension<Arc<Modules>>, Json(payload): Json<CreateProject>) -> impl IntoResponse {
    let res = extension.project_usecase.create_project(payload.project_name).await;

    (StatusCode::CREATED, ())
}

#[derive(Deserialize)]
pub struct CreateProject {
    project_name: String,
}
