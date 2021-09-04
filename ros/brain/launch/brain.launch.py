from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start a led21on_node in the brain namespace
    led21on_node = Node(
            package='brain',
            namespace='brain',
            executable='led21on_action_server.py',
            name='led21on_action_server'
        )

    # start a test_brain_node in the brain namespace
    test_brain_node = Node(
            package='brain',
            namespace='brain',
            executable='test_brain.py',
            name='test_brain'
        )

    return LaunchDescription([
        led21on_node,
        test_brain_node,
    ])
