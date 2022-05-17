use crate::error::Error;
use crate::model::project::Project;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository {
    async fn list(&self) -> Result<Vec<Project>, Error>;
    async fn create(&self, name: String) -> Result<Project, Error>;
    async fn update(&self, id: u64, name: String) -> Result<Project, Error>;
    async fn delete(&self, id: u64) -> Result<bool, Error>;
}
