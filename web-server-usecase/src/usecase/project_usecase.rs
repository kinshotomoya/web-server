use web_server_domain::error::Error;
use web_server_domain::model::project::Project;
use web_server_domain::repository::project::ProjectRepository;
use web_server_adapter::modules::Repositories;

// [メモ]
// genericsを用いない場合は、動的ディスパッチ（dyn）でDIできる
// pub struct ProjectUsecase {
//     repository: dyn ProjectRepository
// }
//
// impl ProjectUsecase {
//
// }

// genericsを用いることで、traitよりもスピードが上がる
// rustではコンパイル時にgenericsに対して全ての型を実装するので、traitのように動的ディスパッチは必要ない
// #[derive(new)]
pub struct ProjectUsecase<R> where R: Repositories {
    repository: R
}

impl<R: Repositories> ProjectUsecase<R> {
    pub async fn list_project(&self) -> Result<Vec<Project>, Error> {
        self.repository.project_repository().list().await
    }

    pub async fn create_project(&self, name: String) -> Result<usize, Error> {
        self.repository.project_repository().create(name).await
    }


    pub fn new(repo: R) -> ProjectUsecase<R> {
        Self {
            repository: repo
        }
    }
}
