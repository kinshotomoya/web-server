use thiserror::Error;

// thiserrorを使うことでカスタムエラーをstd::error::Errorの実装をしなくてもよくなる
// 参考：https://docs.rs/thiserror/latest/thiserror/
#[derive(Error, Debug)]
pub enum Error {
    #[error("not found {0}")]
    NotFount(String),
    #[error("mysql connection timeout {0}")]
    MysqlConnectionTimeOut(String),
    #[error("mysql database execution error {0}")]
    MysqlDatabaseExecutionError(String),
    #[error("supervisor actor mailbox error {0}")]
    SupervisorActorMailBoxError(String),
    #[error("initialize supervisor actor error {0}")]
    InitializedSupervisorActorError(String)
}


