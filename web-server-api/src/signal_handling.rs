use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::iterator::SignalsInfo;
use signal_hook::consts::{SIGINT, SIGTERM};
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;


pub fn signal_handling(tx: Sender<Command>) {
    let mut signals = SignalsInfo::<WithOrigin>::new(&[SIGINT, SIGTERM]).expect("fail signal");
    let handle = signals.handle();
    // ↓for loopにライフタイムを
    // breakは内側のloopに対して実行されるので、loopが入れ子になっている場合はloopにラベリングできる
    // ex
    // 'a: loop {
    //         break 'a;
    // }
    for signal in &mut signals {
        match signal.signal {
            SIGINT | SIGTERM => {
                // oneshot channelでは一つのメッセージしか送らなくて待ち時間は発生しないので、asyncではない
                tx.send(Command::Kill(String::from("kill"))).unwrap();
                // ↓ここでbreakしないと、txのmove問題でコンパイルエラー起きる
                // sendメソッドはtxの所有権を奪うのでloopで複数かtxは利用できないから
                break;
            },
            _ => unreachable!()
        }
    }
    handle.close();
}

#[derive(Debug)]
pub enum Command {
    Kill(String)
}
