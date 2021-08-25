import rclpy
from rclpy.node import Node

from rclpy.action import ActionClient
from pi_gpio_interface.action import GPIO


class MinimalTest(Node):

    def __init__(self):
        super().__init__('minimal_test')
        self._action_client = ActionClient(self, GPIO, 'GPIO')
        timer_period = 0.5  # seconds
        self.timer = self.create_timer(timer_period, self.timer_callback)
        self.i = 0

    def timer_callback(self):
        msg_data = 'Hello World: %d' % self.i
        self.get_logger().info('Publishing: "%s"' % msg_data)
        self.i += 1

    def send_goal(self, order):
        goal_msg = GPIO.Goal()

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)


def main(args=None):
    rclpy.init(args=args)

    minimal_test = MinimalTest()

    future = minimal_test.send_goal({'gpio: "21,high"'})

    rclpy.spin_until_future_complete(minimal_test, future)

    # Destroy the node explicitly
    # (optional - otherwise it will be done automatically
    # when the garbage collector destroys the node object)
    minimal_test.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()

