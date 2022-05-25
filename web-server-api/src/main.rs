use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use web_server_domain::setting;
use web_server_usecase::actor::supervisor_actor::SuperVisorActor;
use crate::modules::Modules;
use actix::prelude::*;

use crate::signal_handling::Command;

// このmodを定義することでmainのmodule treeに登録している感じ
mod hasher;
mod route;
mod server;
mod signal_handling;
mod trace;
mod modules;
mod error_handling;
// tokioを使ってweb serverを実装
// 参考：https://github.com/tokio-rs/tokio/blob/master/examples/echo.rs
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;
//
//     loop {
//         let (mut tcp_stream, _) = listener.accept().await?;
//         // 別スレッドをたててread writeの処理をしている
//         // こうしないと, あるリクエストの処理が終わるまで別リクエストの処理ができない
//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024];
//             loop {
//                 let n = tcp_stream.read(&mut buf).await.expect("faile to read data from socket");
//                 println!("{:?}", buf);
//
//                 if n == 0 {
//                     return ;
//                 }
//
//                 tcp_stream.write_all(&buf[0..n]).await.expect("");
//             }
//         });
//
//     }
// }

// axum sample
// 参考： https://github.com/tokio-rs/axum/blob/main/examples/readme/src/main.rs
// actorを動かすためにactix::mainでactix runtimeを利用
// tokio::mainではなくて
#[actix::main]
async fn main() {
    let socket = SocketAddr::from(([127, 0, 0, 1], 8080));
    // 参考：https://rust-cli.github.io/book/in-depth/signals.html
    // 方法1
    // ctrlc crateを使うとCTRL + Cのシグナルと受け取ることができる
    // ただこれだとctrl cのシグナルしかハンドリングできない
    // ctrlc::set_handler(|| {
    //     println!("receive!!!");
    //     // ↓こんな感じでプロセス殺せる
    //     std::process::exit(1)
    // }).expect("fail");

    // 方法2
    // let mut signals: SignalsInfo = Signals::new(&[SIGINT]).expect("");
    // mainスレッドで↓このようにシグナル待ちをしてしまうと、後続のweb serverの立ち上げができなくなるので
    // シグナル処理は別スレッドで行う必要がある
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("sss");
    //         std::process::exit(1);
    //     }
    // });

    // 設定ファイルの読み込み
    let env = std::env::var("RUN_ENV").expect("fail to load env");
    let settings = setting::Settings::new(env).expect("fail to load config file");

    // module（usecaseやrepositoryをまとめたもの）の作成
    let modules = Arc::new(Modules::new(&settings));
    // tracingの設定
    trace::setting_trace(&settings);

    // 方法3
    // channelを使って処理する
    let (tx, rx) = tokio::sync::oneshot::channel::<Command>();

    // tokio::spawnは別スレッドを作成しているわけではない
    // 非同期タスクを作って、同一スレッドで渡した処理をさせている

    // actix::mainのruntimeを使うようになったので、シグナルハンドリングは別スレッド（OS thread）で行うように修正
    let signal_handle_thread = std::thread::spawn( || signal_handling::signal_handling(tx) );

    // awaitしないとserver起動しない
    // run_serverメソッドはasyncになっていてmainスレッドで待ってあげないと、下の処理に進んでしまう
    server::run_server(socket, rx, modules).await;
    // signal handling threadがちゃんと終わってからmain threadを終わらせるために必要
    // thread::spawnでいう thread.join()と同じ

    // スレッドが終わるのを待つ
    signal_handle_thread.join();

    // TODO:
    //  4. actorを実装!!
    //  5. 別apiとhttpで通信できるように
    //  5. 別スレッドでredisサーバを叩くように
}
