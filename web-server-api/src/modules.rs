use web_server_adapter::persistence::mysql_client::MysqlClient;
use web_server_adapter::repository::RepositoryImpl;
use web_server_domain::setting::Settings;
use web_server_usecase::usecase::project_usecase::ProjectUsecase;
use web_server_usecase::usecase::actor_usecase::ActorUsecase;
use std::sync::Arc;
use actix::{Actor, Supervisor};
use web_server_usecase::actor::supervisor_actor::{Idle, SuperVisorActor};

// プロセス内で共有するモジュールを格納する
pub struct Modules {
    // 各ユースケースインスタンス
    pub project_usecase: ProjectUsecase<RepositoryImpl>,
    pub actor_usecase: ActorUsecase
}

impl Modules {
    pub fn new(settings: &Settings) -> Modules {
        let mysql_client: Arc<MysqlClient> = Arc::new(MysqlClient::new(settings));
        let repository_modules: RepositoryImpl = RepositoryImpl::new(mysql_client);
        let project_usecase: ProjectUsecase<RepositoryImpl> = ProjectUsecase::new(repository_modules);
        let supervisor_actor = Supervisor::start(|_| SuperVisorActor::new());
        let actor_usecase: ActorUsecase = ActorUsecase::new(supervisor_actor);
        Self {
            project_usecase,
            actor_usecase
        }
    }
}
