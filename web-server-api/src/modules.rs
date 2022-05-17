use web_server_domain::repository::project::ProjectRepository;
use web_server_usecase::usecase::project_usecase::ProjectUsecase;
// プロセス内で共有するモジュールを格納する
struct Modules {
    // 複数リポジトリを保持するインスタンス
    // repositoryes:
    // 各ユースケースインスタンス
    // project_usecase: ProjectUsecase<>
}
