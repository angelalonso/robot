use crate::node_action_server_led::*;
use tiny_tokio_actor::*;

// COMMON?

#[derive(Clone, Debug)]
pub struct EventMessage(String);

impl SystemEvent for EventMessage {}

#[derive(Clone, Debug)]
pub struct StartMessage {
    pub destination: ActorPath,
    pub limit: i8,
}

// COMMON  - END

#[derive(Clone)]
pub struct ClientLedActor {
    pub counter: i8,
}

impl Actor<EventMessage> for ClientLedActor {}

#[derive(Clone, Debug)]
pub enum ClientLedMessage {
    Start(StartMessage),
    Ping(i8),
}

impl Message for ClientLedMessage {
    type Response = ServerLedMessage;
}

#[async_trait]
impl Handler<EventMessage, ClientLedMessage> for ClientLedActor {
    async fn handle(
        &mut self,
        msg: ClientLedMessage,
        ctx: &mut ActorContext<EventMessage>,
    ) -> ServerLedMessage {
        if let ClientLedMessage::Start(message) = msg {
            let limit = message.limit;
            if let Some(mut destination) = ctx
                .system
                .get_actor::<ServerLedActor>(&message.destination)
                .await
            {
                while self.counter > -1 && self.counter < limit {
                    let ping = ClientLedMessage::Ping(self.counter);
                    let result = destination.ask(ping).await.unwrap();
                    self.counter = result.0;
                    ctx.system
                        .publish(EventMessage(format!("Counter is now {}", self.counter)));
                }
            }
        }

        ServerLedMessage(self.counter)
    }
}
