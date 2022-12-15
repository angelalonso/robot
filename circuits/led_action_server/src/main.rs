use std::sync::Arc;

use anyhow::Result;
use rclrust::{qos::QoSProfile, rclrust_info};
use rclrust_msg::std_msgs::msg::String as String_;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = rclrust::init()?;
    let mut node = ctx.create_node("led_server")?;
    let logger = node.logger();

    let publisher = node.create_publisher::<String_>("led_act_out", &QoSProfile::default())?;

    let _subscription = node.create_subscription(
        "led_act_in",
        move |msg: Arc<String_>| {
            rclrust_info!(logger, "DOING {}", msg.data);
            let _pub_out = match publisher.publish(&String_ {
                data: format!("{} DONE", msg.data),
            }) {
                Ok(_x) => (),
                Err(_) => (),
            };
            rclrust_info!(logger, "SENT DONE");
        },
        &QoSProfile::default(),
    )?;

    node.wait();
    Ok(())
}
