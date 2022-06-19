use crate::actor::search_actor::InitializeMessage::Initialized;
use crate::actor::supervisor_actor::{State, SuperVisorActor};
use crate::actor::{search_actor, supervisor_actor};
use actix::prelude::*;
use actix::{Actor, Addr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use actix::dev::AsyncContextParts;
use tracing::{debug, Instrument};
use web_server_domain::error::Error;

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub enum InitializeMessage {
    Initialized,
    InitializedFailed(String),
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub enum Message {
    Execute { project_id: u64 },
    CheckRunning,
}

// project_idに紐ずく複数クエリで検索エンジンに検索をかけにいくactor
pub struct SearchActor {
    project_id: u64, // プロジェクト毎にこのアクターを生成する
    state: State,
    reply_to: Arc<Addr<SuperVisorActor>>, // 処理が終わったらSuperVisorActorに終了メッセージを投げる必要がある
    last_started_at: Instant, // ここで時間をもっていても何をトリガーに生存期間を過ぎたかどうかを判断するべきか？？
}

impl SearchActor {
    pub fn new(project_id: u64, reply_to: Arc<Addr<SuperVisorActor>>) -> Self {
        let when = std::time::Instant::now() + Duration::from_secs(600);
        Self {
            project_id,
            state: State::Idle,
            reply_to,
            last_started_at: when,
        }
    }

    // 初期化で必要な処理をする
    fn initialize(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn initializing(&mut self) -> InitializeMessage {
        match self.initialize() {
            Ok(()) => InitializeMessage::Initialized,
            Err(e) => InitializeMessage::InitializedFailed(e.to_string()),
        }
    }
}

impl Supervised for SearchActor {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        debug!("restarting")
    }
}
impl Actor for SearchActor {
    type Context = Context<Self>;
}

impl Handler<InitializeMessage> for SearchActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: InitializeMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InitializeMessage::Initialized => {
                self.state = State::Active;
                Ok(())
            }
            InitializeMessage::InitializedFailed(e) => {
                Err(Error::InitializedSupervisorActorError(e))
            }
        }
    }
}

impl Handler<Message> for SearchActor {
    type Result = ResponseActFuture<Self, Result<(), Error>>;

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            State::Idle => {
                self.state = State::Active;
                Box::pin(async { Ok(()) }.into_actor(self).map(|res, actor, ctx| {
                    ctx.notify(msg);
                    res
                }))
            }
            State::Active => {
                match msg {
                    Message::Execute { project_id } => {
                        let reply_to = Arc::clone(&self.reply_to);
                        Box::pin(
                            async move {
                                // 処理が終わった後に以下コマンドを打ち込む
                                debug!("search actor execute!");
                                reply_to
                                    .send(supervisor_actor::Message::CompletedSearch)
                                    .await;
                                Ok(())
                            }
                            .into_actor(self)
                            .map(|res, actor, ctx| res),
                        )
                    }
                    Message::CheckRunning => {
                        let reply_to = Arc::clone(&self.reply_to);
                        let project_id = self.project_id;
                        debug!("search-actor: {:?}-{:?}", std::thread::current().name(), std::thread::current().id());
                        let message_count = ctx.parts().curr_handle().into_usize();
                        let search_actor_address = Arc::new(ctx.address());
                        Box::pin(
                            async move {
                                debug!("message_count: {}", message_count);
                                if message_count == 0usize {
                                    reply_to.send(supervisor_actor::Message::TerminatedChildActor {project_id, reply_to: search_actor_address}).await;
                                }
                            }
                            .into_actor(self)
                            .map(|res, actor, ctx| Ok(res)),
                        )
                    }
                }
            }
        }
    }
}
