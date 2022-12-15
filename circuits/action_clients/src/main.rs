use std::sync::Arc;
use std::{thread::sleep, time::Duration};

use anyhow::Result;
use rclrust::{qos::QoSProfile, rclrust_info};
use rclrust_msg::std_msgs::msg::String as String_;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = rclrust::init()?;
    let mut node = ctx.create_node("led_client")?;
    let logger = node.logger();
    let publisher = node.create_publisher::<String_>("led_act_in", &QoSProfile::default())?;

    //sleep(Duration::from_millis(100)); // delay for the clients to come up
    let _subscription = node.create_subscription(
        "led_act_out",
        move |msg: Arc<String_>| {
            rclrust_info!(logger, "GOT {}", msg.data);
        },
        &QoSProfile::default(),
    )?;

    for count in 0..4 {
        publisher.publish(&String_ {
            data: format!("hello {}", count),
        })?;
        sleep(Duration::from_millis(1000));
    }

    node.wait();
    Ok(())
}
