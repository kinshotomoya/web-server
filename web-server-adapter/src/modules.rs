use web_server_domain::repository::project::{ProjectRepository, Test};

// 複数のrepositoryをまとめるモジュール
pub trait Repositories {
    type ProjectRepo: ProjectRepository;
    type TestRepo: Test;
    fn project_repository(&self) -> &Self::ProjectRepo;
    fn test_repository(&self) -> &Self::TestRepo;
}
