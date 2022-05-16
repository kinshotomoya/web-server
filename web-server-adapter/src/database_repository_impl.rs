use std::sync::Arc;
use crate::persistence::mysql_client::MysqlClient;

// mysql clientをフィールドにもつstruct
// TODO: DIできるようにする
pub struct DatabaseRepositoryImpl {
    pub mysql_client: MysqlClient
}
