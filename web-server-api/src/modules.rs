use web_server_adapter::persistence::mysql_client::MysqlClient;
use web_server_adapter::repository::RepositoryImpl;
use web_server_domain::repository::project::ProjectRepository;
use web_server_domain::setting::Settings;
use web_server_usecase::usecase::project_usecase::ProjectUsecase;
use std::string::String;
use std::sync::Arc;

// プロセス内で共有するモジュールを格納する
pub struct Modules {
    // 複数リポジトリを保持するインスタンス
    // TODO: usecaseのみをもつ、usecaseがrepository_moduleを保持するようにする
    repository_modules: RepositoryImpl
    // 各ユースケースインスタンス
    // project_usecase: ProjectUsecase<>
}

impl Modules {
    pub fn new(settings: &Settings) -> Modules {
        // TODO: この中で必要なオブジェクトを全て作成する
        let mysql_client = Arc::new(MysqlClient::new(settings));
        Self {
            repository_modules: RepositoryImpl::new(mysql_client)
        }
    }
}
