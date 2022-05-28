use crate::error_handling::ErrorResponse;
use crate::{IntoResponse, Modules};
use axum::extract::Extension;
use std::sync::Arc;

pub async fn execute_actor(extension: Extension<Arc<Modules>>) -> impl IntoResponse {
    let res: Result<(), ErrorResponse> = extension
        .actor_usecase
        .execute_actor()
        .await
        .map_err(|e| e.into())
        .map(|_| ());
    res
}
