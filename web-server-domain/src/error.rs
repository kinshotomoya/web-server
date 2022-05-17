use thiserror::Error;

// thiserrorを使うことでカスタムエラーをstd::error::Errorの実装をしなくてもよくなる
// 参考：https://docs.rs/thiserror/latest/thiserror/
#[derive(Error, Debug)]
pub enum Error {
    #[error("not found {0}")]
    NotFount(String),
}
