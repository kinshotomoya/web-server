use crate::error_handling::ErrorResponse;
use crate::{IntoResponse, Modules};
use axum::extract::Extension;
use std::sync::Arc;

pub async fn execute_actor(extension: Extension<Arc<Modules>>, axum::extract::Path(project_id): axum::extract::Path<u64>) -> impl IntoResponse {
    let res: Result<(), ErrorResponse> = extension
        .actor_usecase
        .execute_actor(project_id)
        .await
        .map_err(|e| e.into())
        .map(|_| ());
    res
}
