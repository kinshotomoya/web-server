use crate::persistence::mysql_client::MysqlClient;
use std::sync::Arc;

// mysql clientをフィールドにもつstruct
// TODO: DIできるようにする
pub struct DatabaseRepositoryImpl {
    pub mysql_client: MysqlClient,
}
