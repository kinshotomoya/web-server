// TODO: actorモジュールを呼び出すusecase
// actixがgithubスター多そう
// https://github.com/actix/actix

use crate::actor::supervisor_actor::{Message, SuperVisorActor};
use actix::{Actor, Addr, MailboxError, Supervisor};
use std::sync::Arc;
use tracing::debug;
use web_server_domain::error::Error;

pub struct ActorUsecase {
    supervisor_actor: Arc<Addr<SuperVisorActor>>,
}

impl ActorUsecase {
    // TODO: Resultが二重になっているのでflatにする
    pub async fn execute_actor(&self, project_id: u64) -> Result<Result<(), Error>, Error> {
        let res: Result<Result<(), Error>, Error> = self
            .supervisor_actor
            .send(Message::StartSearch { project_id })
            .await
            .map_err(|e| Error::SupervisorActorMailBoxError(e.to_string()));
        debug!("{:?}", res);
        res
    }

    pub fn new(supervisor_actor: Arc<Addr<SuperVisorActor>>) -> Self {
        Self { supervisor_actor }
    }
}
