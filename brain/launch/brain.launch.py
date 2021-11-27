from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start motor_workers in the brain namespace
    right_motor_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_right_motor_worker.py',
            arguments=['--ros-args', '--log-level', 'INFO'],
            name='node_right_motor_worker'
        )

    left_motor_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_left_motor_worker.py',
            arguments=['--ros-args', '--log-level', 'INFO'],
            name='node_left_motor_worker'
        )

    # mock the presence of our Arduino when testing locally
    node_arduino = Node(
            package='brain',
            namespace='brain',
            executable='node_arduino.py',
            arguments=['--ros-args', '--log-level', 'INFO'],
            name='node_arduino'
        )

    # start a motors_manager in the brain namespace
    main_brain = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            arguments=['--ros-args', '--log-level', 'INFO'],
            name='main_brain'
        )

    return LaunchDescription([
        right_motor_worker_node,
        left_motor_worker_node,
        node_arduino,
        main_brain,
    ])
