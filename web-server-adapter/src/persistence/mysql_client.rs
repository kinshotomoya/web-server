use std::time::Duration;
use diesel::{Connection, MysqlConnection, r2d2};
use diesel::r2d2::{ConnectionManager, Pool};
use web_server_api::setting::Settings;

// r2d2クレートを使ってコネクションプールを管理できるそう
// マルチスレッド環境をイメージしているので、コネクションプール作って
// Arcでラップしてスレッドスレッド間で共有できるようにしたい
// -> r2d2でのPoolは内部でArcでラップされているので明示的にArcでラップする必要はない
// https://github.com/sfackler/r2d2
pub struct MysqlClient {
    // mysql driverとしてdieselが一番startが多そう
    // 参考：https://github.com/diesel-rs/diesel
    pool: Pool<ConnectionManager<MysqlConnection>>
}

impl MysqlClient {
    // TODO: Settingからmysql urlを取得するようにする
    pub fn new(settings: &Settings) -> MysqlClient {
        // settingsの参照を受け取っているので、settings.database.urlとsettingsの内部フィールドの所有権だけmoveすることはできない
        let manager = ConnectionManager::<MysqlConnection>::new(&settings.database.url);
        let pool = r2d2::Pool::builder().max_size(16).connection_timeout(Duration::from_millis(500)).build(manager).expect("fail to create connection pool to mysql");
        MysqlClient { pool }
    }
}
