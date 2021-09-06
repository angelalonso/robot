from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start a led_node in the brain namespace
    led_node = Node(
            package='brain',
            namespace='brain',
            executable='led_action_server.py',
            name='led_action_server'
        )

    # start a test_brain_node in the brain namespace
    test_brain_node = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            name='test_brain'
        )

    return LaunchDescription([
        led_node,
        test_brain_node,
    ])
