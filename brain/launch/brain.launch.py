from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start motor_workers in the brain namespace
    right_motor_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_right_motor_worker.py',
            name='node_right_motor_worker'
        )

    left_motor_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_left_motor_worker.py',
            name='node_left_motor_worker'
        )

    # start a motors_manager in the brain namespace
    motor_manager_node = Node(
            package='brain',
            namespace='brain',
            executable='node_motors_manager.py',
            name='node_motors_manager'
        )

    return LaunchDescription([
        right_motor_worker_node,
        left_motor_worker_node,
        motor_manager_node,
    ])
