use axum::body;
use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use web_server_domain::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorResponse {
    #[error("not found {0}")]
    NotFount(String),
    #[error("mysql connection timeout {0}")]
    MysqlConnectionTimeOut(String),
    #[error("mysql database execution error {0}")]
    MysqlDatabaseExecutionError(String),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        match self {
            ErrorResponse::NotFount(message) =>
                create_response(message, StatusCode::NOT_FOUND),
            ErrorResponse::MysqlConnectionTimeOut(message) =>
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR),
            ErrorResponse::MysqlDatabaseExecutionError(message) =>
                create_response(message, StatusCode::INTERNAL_SERVER_ERROR)
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
            Error::MysqlConnectionTimeOut(message) => ErrorResponse::MysqlConnectionTimeOut(message),
            Error::MysqlDatabaseExecutionError(message) => ErrorResponse::MysqlDatabaseExecutionError(message)
        }
    }
}
