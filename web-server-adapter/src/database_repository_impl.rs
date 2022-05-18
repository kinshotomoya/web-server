use std::sync::Arc;
use crate::persistence::mysql_client::MysqlClient;


pub struct DatabaseRepositoryImpl {
    pub mysql_client: Arc<MysqlClient>,
}

impl DatabaseRepositoryImpl {
    pub fn new(mysql_client: Arc<MysqlClient>) -> DatabaseRepositoryImpl {
        DatabaseRepositoryImpl{ mysql_client }
    }
}
