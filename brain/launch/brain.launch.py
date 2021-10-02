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

    # mock the presence of our Arduino when testing locally
    mock_arduino = Node(
            package='brain',
            namespace='brain',
            executable='mock_arduino.py',
            name='mock_arduino'
        )

    # start a motors_manager in the brain namespace
    main_brain = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            name='main_brain'
        )

    return LaunchDescription([
        right_motor_worker_node,
        left_motor_worker_node,
        mock_arduino,
        main_brain,
    ])
