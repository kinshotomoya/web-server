use crate::actor::search_actor;
use crate::actor::search_actor::SearchActor;
use actix::dev::MessageResponse;
use actix::prelude::*;
use actix::Actor;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;
use tracing::info;
use web_server_domain::error::Error;

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
    StartSearch {
        project_id: u64,
    },
    CompletedSearch,
    TerminatedChildActor {
        project_id: u64,
    },
    CheckSearchActor, // searchActorの生存確認を行うメッセージ。timerActorから投げられる
    LoopExecute {
        child_actors: Arc<Mutex<HashMap<u64, Addr<SearchActor>>>>,
    },
}

pub enum State {
    Active,
    Idle,
}

// 構成 SuperVisorActor → childActor
//                     ↓ → childActor2
pub struct SuperVisorActor {
    // 状態を持っておけば、finite state machineにできる
    state: State,
    child_actors: Arc<Mutex<HashMap<u64, Addr<SearchActor>>>>, // NOTE: Arcで包んだ参照を可変参照にするには、Mutexで包んであげる必要がある。lockして他のスレッドから参照されないようにする必要がある
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
            state: State::Idle,
            child_actors: Arc::new(Mutex::new(HashMap::new())),
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

    async fn execute_message(
        msg: Message,
        reply_to: Arc<Addr<SuperVisorActor>>,
        child_actors: Arc<Mutex<HashMap<u64, Addr<SearchActor>>>>,
    ) -> Result<(), Error> {
        match msg {
            Message::StartSearch { project_id } => {
                // 子アクターをすでに保持していなかったら作成するロジック
                //   作成したらHashMapに格納する　＝> ここではできない（Arc<HashMap>になっているので）
                //   なのでこの関数の戻り値を作成したsearchActorのアドレスとidにする
                let mut locked_map = child_actors.lock().unwrap();
                match locked_map.get(&project_id) {
                    Some(search_actor) => {
                        // TODO: リファクタリング。Noneの場合と合わせてエラーハンドリング等をちゃんと書く
                        search_actor
                            .send(search_actor::Message::Execute { project_id })
                            .await;
                        Ok(())
                    }
                    None => {
                        let mut search_actor = SearchActor::new(project_id, reply_to);
                        let message = search_actor.initializing();
                        let search_actor = Supervisor::start(|_| search_actor);
                        let res: Result<(), Error> = search_actor
                            .send(message)
                            .await
                            .map_err(|e| Error::SearchActorMailBoxError(e.to_string()))?;
                        search_actor
                            .send(search_actor::Message::Execute { project_id })
                            .await;
                        locked_map.insert(project_id, search_actor);
                        res
                    }
                }
            }
            Message::CompletedSearch => {
                debug!("complete!");
                Ok(())
            }
            Message::TerminatedChildActor { project_id } => {
                // TODO: search actorが停止した時の実装
                //   有効期限切れで停止した時にこのメッセージがsearch actorから投げられる（未実装）
                Ok(())
            }
            Message::CheckSearchActor => {
                // 保持しているsearchActorに生存確認messageを投げる
                // スレッドを占有しないようにfor文で投げるんじゃなくて、再帰で投げる
                // 自分自身にmessageを投げる
                let res: Result<(), Error> = reply_to
                    .send(Message::LoopExecute { child_actors })
                    .await
                    .map_err(|e| Error::SupervisorActorMailBoxError(e.to_string()))?;
                res
            }
            Message::LoopExecute { child_actors } => {
                let child_actors_clone = Arc::clone(&child_actors);
                let mut locked_map = child_actors_clone.lock().unwrap();
                if locked_map.len() != 0usize {
                    debug!("manage child actors: {:?}", locked_map);
                    let key = locked_map.keys().next().copied().unwrap_or(0); // copied()・・・Option<&A> -> Option<A>
                    let search_actor_address = locked_map.get(&key);
                    if let Some(search_actor) = search_actor_address {
                        search_actor.send(search_actor::Message::CheckRunning).await;
                        locked_map.remove(&key);
                    }
                    drop(locked_map);
                    // NOTE: ↓でさらにLoopExecute messageを投げることで、locked_mapがdead lockを起こしていたので、スレッドが止まってしまっていた。
                    // 明示的にlockを解消するためにdropさせている。
                    reply_to.send(Message::LoopExecute { child_actors }).await;
                } else {
                    debug!("child actor count is zero");
                }
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
                Err(Error::InitializedSupervisorActorError(e))
            }
        }
    }
}

impl Handler<Message> for SuperVisorActor {
    // acto・コンテキスト内にアクセスしたい場合には、ResponseFutureではなくResponseActFutureを戻り方として定義する
    type Result = ResponseActFuture<Self, Result<(), Error>>;

    // SuperVisorActorが受け取ったメッセージ毎にこのhandleメソッドが呼ばれる
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        let ctx_address = Arc::new(ctx.address());
        let child_actors = Arc::clone(&self.child_actors);
        // actorの状態によって処理を変える
        match self.state {
            State::Idle => {
                self.state = State::Active;
                Box::pin(
                    async move { Ok(()) }
                        .into_actor(self)
                        .map(|res, _act, _ctx| {
                            // 自分自身に同じメッセージをもう一回投げている
                            // ResponseActFutureを使うことで、↓のようにcontextを利用できる
                            _ctx.notify(msg);
                            res
                        }),
                )
            }
            State::Active => {
                // TODO: ここら辺何を表しているか理解する
                // https://users.rust-lang.org/t/actix-await-for-send-in-handle/47844
                // https://github.com/actix/actix/issues/438
                Box::pin(
                    async move {
                        // ここで何かしらの非同期処理を行う
                        SuperVisorActor::execute_message(msg, ctx_address, child_actors).await
                    }
                    .into_actor(self)
                    .map(|res, act, _ctx| res),
                )
            }
        }
    }
}
