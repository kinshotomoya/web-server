use axum::body;
use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use web_server_domain::error::Error;

#[derive(Error, Debug)]
pub enum ErrorResponse {
    #[error("not found {0}")]
    NotFount(String),
    #[error("mysql connection timeout {0}")]
    MysqlConnectionTimeOut(String),
    #[error("mysql database execution error {0}")]
    MysqlDatabaseExecutionError(String),
    #[error("supervisor actor mailbox error {0}")]
    SupervisorActorMailBoxError(String),
    #[error("initialize supervisor actor error {0}")]
    InitializedSupervisorActorError(String),
    #[error("search actor mailbox error {0}")]
    SearchActorMailBoxError(String),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        match self {
            ErrorResponse::NotFount(message) => create_response(message, StatusCode::NOT_FOUND),
            ErrorResponse::MysqlConnectionTimeOut(message) => {
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorResponse::MysqlDatabaseExecutionError(message) => {
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorResponse::SupervisorActorMailBoxError(message) => {
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorResponse::InitializedSupervisorActorError(message) => {
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorResponse::SearchActorMailBoxError(message) => {
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

fn create_response(error_message: String, status_code: StatusCode) -> Response {
    let body = body::boxed(body::Full::from(error_message));
    Response::builder().status(status_code).body(body).unwrap()
}

// ドメイン層のErrorをapi層のErrorに型変換する
impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        match e {
            Error::NotFount(message) => ErrorResponse::NotFount(message),
            Error::MysqlConnectionTimeOut(message) => {
                ErrorResponse::MysqlConnectionTimeOut(message)
            }
            Error::MysqlDatabaseExecutionError(message) => {
                ErrorResponse::MysqlDatabaseExecutionError(message)
            }
            Error::SupervisorActorMailBoxError(message) => {
                ErrorResponse::SupervisorActorMailBoxError(message)
            }
            Error::InitializedSupervisorActorError(message) => {
                ErrorResponse::InitializedSupervisorActorError(message)
            }
            Error::SearchActorMailBoxError(message) => {
                ErrorResponse::SearchActorMailBoxError(message)
            }
        }
    }
}
