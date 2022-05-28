use std::collections::HashMap;
use std::sync::Arc;
use actix::dev::MessageResponse;
use actix::prelude::*;
use actix::Actor;
use tracing::log::info;
use tracing::log::debug;
use web_server_domain::error::Error;
use crate::actor::search_actor::SearchActor;


#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub enum InitializeMessage {
    Initialized,
    InitializedFailed(String),
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), Error>")]
pub enum Message {
    // ここにSuperVisorActorへのメッセージを追加していく
    StartSearch { project_id: u64 }
}

pub enum State {
    Active,
    Idle,
}

// 構成 SuperVisorActor → childActor
//                     ↓ → childActor2
pub struct SuperVisorActor {
    count: usize,
    // 状態を持っておけば、finite state machineにできる
    state: State,
    child_actors: Arc<HashMap<u64, String>>
}

impl Supervised for SuperVisorActor {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        // SuperVisorActorがstopしていたら、このメソッドがよばれ
        // 再度同じアドレスのSuperVisorActorを再起動する
        debug!("restart!!")
    }
}

impl SuperVisorActor {
    pub fn new() -> Self {
        Self {
            count: 0,
            state: State::Idle,
            child_actors: Arc::new(HashMap::new())
        }
    }

    fn initialize(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // actor初期化に必要な処理を行う
    pub fn initializing(&mut self) -> InitializeMessage {
        match self.initialize() {
            Ok(()) => InitializeMessage::Initialized,
            Err(e) => InitializeMessage::InitializedFailed(e.to_string()),
        }
    }

    async fn execute_message(msg: Message, reply_to: Arc<Addr<SuperVisorActor>>, child_actors: Arc<HashMap<u64, String>>) -> Result<(), Error> {
        match msg {
            Message::StartSearch {project_id} => {
                // TODO: 子アクターをすでに保持していなかったら作成するロジック
                //   作成したらHashMapに格納する　＝> ここではできない（Arc<HashMap>になっているので）
                //   なのでこの関数の戻り値を作成したsearchActorのアドレスとidにする
                let mut search_actor = SearchActor::new(project_id, reply_to);
                let message = search_actor.initializing();
                let search_actor = Supervisor::start(|_| search_actor);
                // TODO: searchActorが作成できなかったらエラーを返す.作成できたらOkを返す
                let a = search_actor.send(message).await;
                Ok(())
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

// fsmぽくするなら投げられるメッセージ毎にHandlerを実装するしかなさそう
impl Handler<InitializeMessage> for SuperVisorActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: InitializeMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InitializeMessage::Initialized => {
                self.state = State::Active;
                Ok(())
            }
            InitializeMessage::InitializedFailed(e) => {
                ctx.stop();
                Err(Error::SupervisorActorMailBoxError(e)) // TODO: エラー定義する
            }
        }
    }
}

impl Handler<Message> for SuperVisorActor {
    type Result = ResponseActFuture<Self, Result<(), Error>>;

    // SuperVisorActorが受け取ったメッセージ毎にこのhandleメソッドが呼ばれる
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        let ctx_address = Arc::new(ctx.address());
        let actors = self.child_actors.clone();
        // actorの状態によって処理を変える
        match self.state {
            State::Idle => {
                self.state = State::Active;
                Box::pin(
                    async move {
                        Ok(())
                    }.into_actor(self).map(|res, _act, _ctx| {
                        // 自分自身に同じメッセージをもう一回投げている
                        _ctx.notify(msg);
                        res
                    })
                )
            }
            State::Active => {
                // TODO: ここら辺何を表しているか理解する
                // https://users.rust-lang.org/t/actix-await-for-send-in-handle/47844
                // https://github.com/actix/actix/issues/438
                Box::pin(
                    async move {
                        SuperVisorActor::execute_message(msg, ctx_address, actors).await;
                        Ok(())
                    }.into_actor(self).map(|res, _act, _ctx| {
                        // TODO: 作成したsearchActorをHashMapに格納する
                        res
                    })
                )
            },
        }
    }
}
