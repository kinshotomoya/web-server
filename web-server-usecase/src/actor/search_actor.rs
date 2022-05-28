use std::sync::Arc;
use actix::{Actor, Addr};
use crate::actor::supervisor_actor::{State, SuperVisorActor};
use actix::prelude::*;
use tracing::debug;
use web_server_domain::error::Error;
use crate::actor::search_actor::InitializeMessage::Initialized;


#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub enum InitializeMessage {
    Initialized,
    InitializedFailed(String),
}


#[derive(Message)]
#[rtype(result = "()")]
pub enum Message {
    Execute { project_id: u64 }
}

// project_idに紐ずく複数クエリで検索エンジンに検索をかけにいくactor
pub struct SearchActor {
    project_id: u64, // プロジェクト毎にこのアクターを生成する
    state: State,
    reply_to: Arc<Addr<SuperVisorActor>> // 処理が終わったらSuperVisorActorに終了メッセージを投げる必要がある
}

impl SearchActor {
    pub fn new(project_id: u64, reply_to: Arc<Addr<SuperVisorActor>>) -> Self {
        Self {
            project_id,
            state: State::Idle,
            reply_to
        }
    }

    // 初期化で必要な処理をする
    fn initialize(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn initializing(&mut self) -> InitializeMessage {
        match self.initialize() {
            Ok(()) => InitializeMessage::Initialized,
            Err(e) => InitializeMessage::InitializedFailed(e.to_string())
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

impl Handler<InitializeMessage> for SearchActor{
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: InitializeMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InitializeMessage::Initialized => {
                self.state = State::Active;
                Ok(())
            },
            InitializeMessage::InitializedFailed(e) => Err(Error::InitializedSupervisorActorError(e))
        }
    }
}


impl Handler<Message> for SearchActor{
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
