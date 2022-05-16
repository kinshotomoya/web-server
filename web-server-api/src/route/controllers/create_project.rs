use axum::Json;
use crate::{IntoResponse, StatusCode};
use serde::{Deserialize, Serialize};


// TODO: IntoResponse ???
pub async fn create_project(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        user_name: payload.username
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    user_name: String,
}
