from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start a led21_node in the brain namespace
    led21_node = Node(
            package='brain',
            namespace='brain',
            executable='led21_action_server.py',
            name='led21_action_server'
        )

    # start a test_brain_node in the brain namespace
    test_brain_node = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            name='test_brain'
        )

    return LaunchDescription([
        led21_node,
        test_brain_node,
    ])
