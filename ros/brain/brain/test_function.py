import rclpy
from rclpy.node import Node



class MinimalTest(Node):

    def __init__(self):
        super().__init__('minimal_test')
        timer_period = 0.5  # seconds
        self.timer = self.create_timer(timer_period, self.timer_callback)
        self.i = 0

    def timer_callback(self):
        msg = String()
        msg.data = 'Hello World: %d' % self.i
        self.get_logger().info('Publishing: "%s"' % msg.data)
        self.i += 1



def main(args=None):
    rclpy.init(args=args)

    minimal_test = MinimalTest()

    rclpy.spin(minimal_test)

    # Destroy the node explicitly
    # (optional - otherwise it will be done automatically
    # when the garbage collector destroys the node object)
    minimal_test.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
