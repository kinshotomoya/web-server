use actix::Actor;
use actix::dev::MessageResponse;
use actix::prelude::*;
use tracing::info;

#[derive(Debug, MessageResponse)]
pub struct ActorResponse {
    count: usize
}

#[derive(Message)]
#[rtype(result = "ActorResponse")]
pub enum Message {
    // ここにSuperVisorActorへのメッセージを追加していく
    Ping { count: usize },
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum Idle {
    Initialized
}

pub enum State {
    Active,
    Idle,
}

// 構成 SuperVisorActor → childActor
//                     ↓ → childActor2

// TODO: state machineにするにはどうする？？
//  finite state machineにするのは難しそう。。。
//  Handlerは複数実装できるが、切り替えることはできない（常にhandleメソッドに実装したメッセージを受け取れる状態になってしまう）
pub struct SuperVisorActor {
    count: usize,
    // 状態を持っておけば、finite state machineにできる！？
    // 各handlerのなかで、状態によってメッセージ受け取れるか判断する
    state: State
}

impl Supervised for SuperVisorActor {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        // SuperVisorActorがstopしていたら、このメソッドがよばれ
        // 再度同じアドレスのSuperVisorActorを再起動する
        info!("restart!!")
    }
}

impl SuperVisorActor {
    pub fn new() -> Self {
        Self {
            count: 0,
            state: State::Idle
        }
    }

    fn execute_message(&mut self, msg: Message) -> ActorResponse {
        match msg {
            Message::Ping {count} => {
                // TODO: こんな感じで、子アクターを作成するメッセージを受け取ったら子アクターを作成する
                // let rr = SuperVisorActor::new().start();
                // println!("{:p}", &rr);
                self.count += count;
                ActorResponse { count: self.count}
            }
        }
    }
}

impl Actor for SuperVisorActor {
    type Context = Context<Self>;

    // このアクターが初めて呼ばれたときにこのstartedメソッドが一回呼ばれる
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("started supervisor actor");
    }

}

impl Handler<Message> for SuperVisorActor {
    type Result = ActorResponse;

    // SuperVisorActorが受け取ったメッセージ毎にこのhandleメソッドが呼ばれる
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        // actorの状態によって処理を変える
        match self.state {
            State::Idle => {
                self.state = State::Active;
                // ctx.address().send(msg)
                self.handle(msg, ctx)
            },
            State::Active => {
                self.execute_message(msg)
            }
        }
    }
}
