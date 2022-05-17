use crate::database_repository_impl::DatabaseRepositoryImpl;
use async_trait::async_trait;
use web_server_domain::error::Error;
use web_server_domain::model::project::Project;
use web_server_domain::repository::project::{ProjectRepository, Test};

// DatabaseRepositoryImplにProjectRepositoryを実装する
// もしバックエンドのDBを別DBに差し替える場合にはDatabaseRepositoryImplを差し替えるだけで良い
#[async_trait]
impl ProjectRepository for DatabaseRepositoryImpl {
    async fn list(&self) -> Result<Vec<Project>, Error> {
        todo!()
    }

    async fn create(&self, name: String) -> Result<Project, Error> {
        todo!()
    }

    async fn update(&self, id: u64, name: String) -> Result<Project, Error> {
        todo!()
    }

    async fn delete(&self, id: u64) -> Result<bool, Error> {
        todo!()
    }
}

#[async_trait]
impl Test for DatabaseRepositoryImpl {
    async fn lists(&self) -> Result<Vec<Project>, Error> {
        todo!()
    }
}
