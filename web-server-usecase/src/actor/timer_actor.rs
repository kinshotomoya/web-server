use std::sync::Arc;
use std::time::Duration;
use actix::{Actor, Handler, Supervised};
use tracing::debug;
use web_server_domain::error::Error;
use actix::prelude::*;
use crate::actor::supervisor_actor;
use crate::actor::supervisor_actor::SuperVisorActor;


#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub enum Message {
    CheckSearchActor { supervisor_actor_address: Arc<Addr<SuperVisorActor>> }
}

 pub struct TimerActor {
    interval: Duration
}

impl TimerActor {
    pub fn new() -> Self {
        Self {
            interval: Duration::from_secs(10)
        }
    }
}

impl Supervised for TimerActor {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        debug!("restarting timer actor");
    }
}

impl Actor for TimerActor {
    type Context = Context<Self>;
}

impl Handler<Message> for TimerActor{
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Message::CheckSearchActor { supervisor_actor_address } => {
                // run_intervalでasync functionを実行したいならarbiterスレッドを作成して、そいつにasync taskを投げるようにする
                ctx.run_interval(self.interval, move |timer_actor, context| {
                    // 参考：https://www.reddit.com/r/rust/comments/srfho0/help_with_actix_arbiter/
                    let supervisor_actor_address_clone = Arc::clone(&supervisor_actor_address);
                    context.spawn(async move {
                        supervisor_actor_address_clone.send(supervisor_actor::Message::CheckSearchActor).await;
                    }.into_actor(timer_actor)); // futureをactorにしている
                });
            }
        }
        Ok(())
    }
}
