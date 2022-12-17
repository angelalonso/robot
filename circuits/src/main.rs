use circuits::node::Node;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mut nodes: Vec<Node> = vec![];
    let data1 = Arc::new(Mutex::new("".to_string()));
    let msgs: Vec<tokio::sync::MutexGuard<String>>; //add several for each group of nodes
    let node_names = vec!["led_action_client", "led_action_server"];

    let mut idx = 0;
    let mut index;
    for n in node_names {
        println!("Adding node: {}", n);
        idx += 1;
        index = format!("{}", idx);
        let this_node = Node::new(index, n.to_string());
        nodes.push(this_node);
    }
    let mut handles = vec![];
    for mut n in nodes {
        let data2 = Arc::clone(&data1);
        let handle = tokio::spawn(async move {
            let mut lock = data2.lock().await;
            n.run(lock).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
