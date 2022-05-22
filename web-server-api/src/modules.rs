use web_server_adapter::persistence::mysql_client::MysqlClient;
use web_server_adapter::repository::RepositoryImpl;
use web_server_domain::setting::Settings;
use web_server_usecase::usecase::project_usecase::ProjectUsecase;
use std::sync::Arc;

// プロセス内で共有するモジュールを格納する
pub struct Modules {
    // 各ユースケースインスタンス
    project_usecase: ProjectUsecase<RepositoryImpl>
}

impl Modules {
    pub fn new(settings: &Settings) -> Modules {
        let mysql_client: Arc<MysqlClient> = Arc::new(MysqlClient::new(settings));
        let repository_modules: RepositoryImpl = RepositoryImpl::new(mysql_client);
        let project_usecase: ProjectUsecase<RepositoryImpl> = ProjectUsecase::new(repository_modules);
        Self {
            project_usecase
        }
    }
}
