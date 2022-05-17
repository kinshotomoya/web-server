use std::sync::Arc;
use web_server_domain::repository::project::{ProjectRepository, Test};
use crate::database_repository_impl::DatabaseRepositoryImpl;
use crate::persistence::mysql_client::MysqlClient;

// 複数のrepositoryをまとめるモジュール
pub trait Repositories {
    // TODO: 関連型を調べる。よくわかっていない。。
    type ProjectRepo: ProjectRepository;
    type TestRepo: Test;
    fn project_repository(&self) -> &Self::ProjectRepo;
    fn test_repository(&self) -> &Self::TestRepo;
}
