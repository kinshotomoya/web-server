use std::sync::Arc;
use web_server_domain::repository::project::{ProjectRepository, Test};
use crate::database_repository_impl::DatabaseRepositoryImpl;
use crate::modules::Repositories;
use crate::persistence::mysql_client::MysqlClient;

pub mod project_repository_impl;

pub struct RepositoryImpl {
    project_repository: DatabaseRepositoryImpl,
    test_repository: DatabaseRepositoryImpl
}

impl Repositories for RepositoryImpl {
    // TODO: DatabaseRepositoryImplを実装型として定義しているが、Repositories内ではそれぞれtraitを関連型として定義しているので
    //  それぞれの型が呼ばれるようになっている？？
    type ProjectRepo = DatabaseRepositoryImpl;
    type TestRepo = DatabaseRepositoryImpl;

    fn project_repository(&self) -> &Self::ProjectRepo {
        &self.project_repository
    }

    fn test_repository(&self) -> &Self::TestRepo {
        &self.test_repository
    }
}

// TODO: apiのmodulesから呼び出す
impl RepositoryImpl {
    pub fn new(mysql_client: Arc<MysqlClient>) -> RepositoryImpl {
        RepositoryImpl {
            project_repository: DatabaseRepositoryImpl::new(mysql_client.clone()),
            test_repository: DatabaseRepositoryImpl::new(mysql_client.clone()),
        }
    }
}
