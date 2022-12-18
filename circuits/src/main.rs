use circuits::node::*;
use circuits::node_action_client_led::*;
use circuits::node_action_server_led::*;
use tiny_tokio_actor::*;

use std::thread;

#[tokio::main]
async fn main() {
    //let mut node_port: HashMap<String, String> = HashMap::new();
    // TODO: add infos in a hashmap

    let node_action_server_led = Node::new("action_server_led", "8401", "8501");
    let node_action_client_led = Node::new("action_client_led", "8501", "8401");

    println!("{}", node_action_server_led.get_port());
    //node_action_server_led.run();
    println!("{}", node_action_client_led.get_port());
    //node_action_client_led.run();
    let handle_s = thread::spawn(move || {
        node_action_server_led.run();
    });
    let handle_c = thread::spawn(move || {
        node_action_client_led.run();
    });
    handle_s.join().unwrap();
    handle_c.join().unwrap();
    //    let led_bus = EventBus::<EventMessage>::new(1000);
    //    let system = ActorSystem::new("action_led", led_bus);
    //
    //    let action_client_led = ClientLedActor { counter: 0 };
    //    let mut action_client_led_ref = system
    //        .create_actor("client_led", action_client_led)
    //        .await
    //        .unwrap();
    //
    //    let action_server_led = ServerLedActor {};
    //    let action_server_led_ref = system
    //        .create_actor("server_led", action_server_led)
    //        .await
    //        .unwrap();
    //
    //    let start = StartMessage {
    //        destination: action_server_led_ref.path().clone(),
    //        limit: 15,
    //    };
    //
    //    let mut events = system.events();
    //    tokio::spawn(async move {
    //        loop {
    //            match events.recv().await {
    //                Ok(event) => println!("Received event! {:?}", event),
    //                Err(err) => println!("Error receivng event!!! {:?}", err),
    //            }
    //        }
    //    });
    //
    //    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    //
    //    let result = action_client_led_ref
    //        .ask(ClientLedMessage::Start(start))
    //        .await
    //        .unwrap();
    //    println!("Final result: {:?}", &result);
}
