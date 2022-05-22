use std::sync::Arc;
use crate::database_repository_impl::DatabaseRepositoryImpl;
use crate::modules::Repositories;
use crate::persistence::mysql_client::MysqlClient;

pub mod project_repository_impl;

pub struct RepositoryImpl {
    project_repository: DatabaseRepositoryImpl,
    test_repository: DatabaseRepositoryImpl
}

impl Repositories for RepositoryImpl {
    // traitでは、実装すべき型を定義しておいて
    // 実際の実装部分（↓ここ）では、↑実装すべき型の実装を定義してあげる
    // こうすることでtraitをより汎用的に定義できる
    type ProjectRepo = DatabaseRepositoryImpl;
    type TestRepo = DatabaseRepositoryImpl;

    fn project_repository(&self) -> &Self::ProjectRepo {
        &self.project_repository
    }

    fn test_repository(&self) -> &Self::TestRepo {
        &self.test_repository
    }
}

impl RepositoryImpl {
    pub fn new(mysql_client: Arc<MysqlClient>) -> RepositoryImpl {
        RepositoryImpl {
            project_repository: DatabaseRepositoryImpl::new(mysql_client.clone()),
            test_repository: DatabaseRepositoryImpl::new(mysql_client),
        }
    }
}
