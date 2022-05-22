use crate::error::Error;
use crate::model::project::Project;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository {
    async fn list(&self) -> Result<Vec<Project>, Error>;
    async fn create(&self, project_name: String) -> Result<usize, Error>;
    async fn update(&self, id: u64, project_name: String) -> Result<Project, Error>;
    async fn delete(&self, id: u64) -> Result<bool, Error>;
}

#[async_trait]
pub trait Test {
    async fn lists(&self) -> Result<Vec<Project>, Error>;
}
