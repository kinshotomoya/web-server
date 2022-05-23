use std::sync::Arc;
use axum::extract::Extension;
use crate::{IntoResponse, Modules, StatusCode};
use axum::Json;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use crate::error_handling::ErrorResponse;


// ここでimpl IntoResponseとすることで、実際はResult型をmatchでextractするべき箇所をIntoResponse内でResult型にIntoResponseを実装しているので
// 手間が省けている
pub async fn create_project(extension: Extension<Arc<Modules>>, Json(payload): Json<CreateProject>) -> impl IntoResponse {
    let res: Result<(), ErrorResponse> = extension.project_usecase.create_project(payload.project_name).await.map_err(|e| e.into());
    // ↓ このように一つ一つのhandlerにmatchケースを書く必要があるが、impl IntoResponseなら不要
    // match res {
    //     Ok(r) => r.into_response(),
    //     Err(e) => e.into_response()
    // }
    res
}

#[derive(Deserialize)]
pub struct CreateProject {
    project_name: String,
}
