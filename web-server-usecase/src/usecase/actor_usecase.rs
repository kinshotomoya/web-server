// TODO: actorモジュールを呼び出すusecase
// actixがgithubスター多そう
// https://github.com/actix/actix

use actix::Actor;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
enum Message {
    // ここにSuperVisorActorへのメッセージを追加していく
    Ping { count: usize }
}


// 構成 SuperVisorActor → childActor
//                     ↓ → childActor2

// TODO: state machineにするにはどうする？？
struct SuperVisorActor {
    count: usize
}

impl Actor for SuperVisorActor {
    type Context = Context<Self>;
}

impl Handler<Message> for SuperVisorActor {
    type Result = usize;

    // SuperVisorActorが受け取ったメッセージ毎にこのhandlerメソッドが呼ばれる
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        // 受け取ったメッセージによって処理を変える
        match msg {
            Message::Ping {count} => {
                self.count += count;
                self.count
            }
        }
    }
}
