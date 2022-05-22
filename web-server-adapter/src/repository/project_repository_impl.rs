use std::ops::Deref;
use crate::database_repository_impl::DatabaseRepositoryImpl;
use async_trait::async_trait;
use web_server_domain::error::Error;
use web_server_domain::model::project::Project;
use web_server_domain::repository::project::{ProjectRepository, Test};
extern crate diesel;
use diesel::prelude::*;
use crate::schema::projects;



// DatabaseRepositoryImplにProjectRepositoryを実装する
// もしバックエンドのDBを別DBに差し替える場合にはDatabaseRepositoryImplを差し替えるだけで良い
#[async_trait]
impl ProjectRepository for DatabaseRepositoryImpl {
    async fn list(&self) -> Result<Vec<Project>, Error> {
        todo!()
    }


    // 参考: https://github.com/diesel-rs/diesel/tree/v1.4.4/examples/mysql/getting_started_step_2
    async fn create(&self, project_name: String) -> Result<usize, Error> {
        let new_project = crate::models::NewProject { name: project_name };
        let conn = self.mysql_client.pool.get().map_err(|e| {Error::MysqlConnectionTimeOut(e.to_string())})?;
        let result = diesel::insert_into(projects::table).values(&new_project).execute(conn.deref()).map_err(|e| {Error::MysqlDatabaseExecutionError(e.to_string())});
        result
    }

    async fn update(&self, _id: u64, _project_name: String) -> Result<Project, Error> {
        todo!()
    }

    async fn delete(&self, _id: u64) -> Result<bool, Error> {
        todo!()
    }
}

#[async_trait]
impl Test for DatabaseRepositoryImpl {
    async fn lists(&self) -> Result<Vec<Project>, Error> {
        todo!()
    }
}
