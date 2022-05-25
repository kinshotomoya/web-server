use std::sync::Arc;
use axum::extract::Extension;
use crate::{IntoResponse, Modules};
use crate::error_handling::ErrorResponse;

pub async fn execute_actor(extension: Extension<Arc<Modules>>) -> impl IntoResponse {
    let res: Result<(), ErrorResponse> = extension.actor_usecase.execute_actor().await.map_err(|e| e.into()).map(|_| ());
    res
}
