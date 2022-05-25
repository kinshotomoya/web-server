// TODO: actorモジュールを呼び出すusecase
// actixがgithubスター多そう
// https://github.com/actix/actix

use std::sync::Arc;
use actix::{Actor, Addr, MailboxError};
use web_server_domain::error::Error;
use crate::actor::supervisor_actor::{Message, SuperVisorActor};

pub struct ActorUsecase {
    supervisor_actor: Addr<SuperVisorActor>
}

impl ActorUsecase {
    pub async fn execute_actor(&self) -> Result<usize, Error>{
        let res: Result<usize, Error> = self.supervisor_actor.send(Message::Ping {count: 1}).await.map_err(|e| Error::SupervisorActorMailBoxError(e.to_string()));
        println!("{:?}", res);
        res
    }

    pub fn new(supervisor_actor: Addr<SuperVisorActor>) -> Self {
        Self{
            supervisor_actor
        }
    }
}
