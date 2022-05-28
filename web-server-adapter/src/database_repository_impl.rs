use crate::persistence::mysql_client::MysqlClient;
use std::sync::Arc;

// ここで参照を持つと↓のようにライフタイム地獄になってしまうので（その他ファイル全てにライフタイムを付与しないといけなくなる）、
//　Arcで囲ってあげる
// pub struct DatabaseRepositoryImpl<'a> {
//     pub mysql_client: &'a MysqlClient, //
// }
//
// impl<'a> DatabaseRepositoryImpl<'a> {
//     pub fn new(mysql_client: &MysqlClient) -> DatabaseRepositoryImpl {
//         DatabaseRepositoryImpl{ mysql_client }
//     }
// }

// tokioのランタイムを使っているので、Rcではなくて
// スレッドセーフなArcを使う
pub struct DatabaseRepositoryImpl {
    pub mysql_client: Arc<MysqlClient>, //
}

impl DatabaseRepositoryImpl {
    pub fn new(mysql_client: Arc<MysqlClient>) -> DatabaseRepositoryImpl {
        DatabaseRepositoryImpl { mysql_client }
    }
}
