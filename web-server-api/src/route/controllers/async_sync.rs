use crate::hasher;

pub async fn async_sync() -> String {
    print_thread_info("async");
    // CPU高負荷なブロッキング処理は、spawn_blockingで別スレッドで処理させる
    let blocking_res = tokio::task::spawn_blocking(|| {
        print_thread_info("blocking");
        let hash = hasher::create_hash_from(String::from("hello world"));
        hash
    })
    .await;

    // ノンブロッキング処理はspawnで非同期所利用スレッドプールにタスク投げる
    // TODO: ノンブロッキングなhttp clientを使ってみようか
    // 各crateの比較：https://blog.logrocket.com/the-state-of-rust-http-clients/
    // hyper公式：https://github.com/hyperium/hyper
    tokio::task::spawn(async {});

    match blocking_res {
        Ok(hash) => hash,
        Err(_) => String::from("error"),
    }
}

fn print_thread_info(prefix: &str) {
    let thread_info = std::thread::current();
    println!(
        "{}-{}-{:?}",
        prefix,
        thread_info.name().unwrap_or(""),
        thread_info.id()
    );
}
