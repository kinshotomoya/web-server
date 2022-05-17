use web_server_domain::error::Error;
use web_server_domain::model::project::Project;
use web_server_domain::repository::project::ProjectRepository;
use derive_new::new;

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
#[derive(new)]
pub struct ProjectUsecase<R> where R: ProjectRepository {
    repository: R
}

impl<R: ProjectRepository> ProjectUsecase<R> {
    async fn list_project(&self, name: String) -> Result<Vec<Project>, Error> {
        self.repository.list().await
    }
}
