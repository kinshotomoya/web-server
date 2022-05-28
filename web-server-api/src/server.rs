use crate::{Command, Modules};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::Receiver;
use tracing::debug;

pub async fn run_server(socket: SocketAddr, rx: Receiver<Command>, modules: Arc<Modules>) {
    let app = crate::route::route(modules);
    let server = axum::Server::bind(&socket).serve(app.into_make_service());
    debug!("binding axum server at port 8080");
    let graceful = server.with_graceful_shutdown(async {
        // 引数に渡したfutureのクロージャが完了したらgraceful shutdownされる様な設定
        // txからのコマンドを待っている
        // 今回はシグナルを待っていたがそれ以外にも終了待ちすべき処理をここに追加する
        match rx.await.ok() {
            Some(Command::Kill(s)) => debug!("get the command: {}", s),
            None => debug!("nothing to do"),
        }
    });

    // graceful shutdownの完了を待っている
    match graceful.await {
        Ok(_) => debug!("graceful shutdown correctly"),
        Err(e) => debug!("{}", e),
    }
}
