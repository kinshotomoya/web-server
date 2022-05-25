use std::any::Any;
use actix::Actor;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
pub enum Message {
    // ここにSuperVisorActorへのメッセージを追加していく
    Ping { count: usize }
}


// 構成 SuperVisorActor → childActor
//                     ↓ → childActor2

// TODO: state machineにするにはどうする？？
pub struct SuperVisorActor {
    count: usize
}

impl SuperVisorActor {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }
}

// TODO: actix::Supervisedを実装する
// => そうすると、Supervisor actor特有のメソッド等を実装できる！
impl Actor for SuperVisorActor {
    type Context = Context<Self>;

    // このアクターが初めて呼ばれたときにこのstartedメソッドが一回呼ばれる
    fn started(&mut self, ctx: &mut Self::Context) {

    }
}

impl Handler<Message> for SuperVisorActor {
    type Result = usize;

    // SuperVisorActorが受け取ったメッセージ毎にこのhandlerメソッドが呼ばれる
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        // 受け取ったメッセージによって処理を変える
        match msg {
            Message::Ping {count} => {
                // TODO: こんな感じで、子アクターを作成するメッセージを受け取ったら子アクターを作成する
                // let rr = SuperVisorActor::new().start();
                // println!("{:p}", &rr);
                self.count += count;
                self.count
            }
        }
    }
}
