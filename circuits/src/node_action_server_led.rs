//use crate::actor::*;
use crate::node_action_client_led::*;
use tiny_tokio_actor::*;

#[derive(Clone, Debug)]
pub struct ServerLedMessage(pub i8);

impl Message for ServerLedMessage {
    type Response = ();
}

#[derive(Clone)]
pub struct ServerLedActor;

impl Actor<EventMessage> for ServerLedActor {}

#[async_trait]
impl Handler<EventMessage, ClientLedMessage> for ServerLedActor {
    async fn handle(
        &mut self,
        msg: ClientLedMessage,
        _ctx: &mut ActorContext<EventMessage>,
    ) -> ServerLedMessage {
        if let ClientLedMessage::Ping(counter) = msg {
            ServerLedMessage(counter + 1)
        } else {
            ServerLedMessage(-1)
        }
    }
}
