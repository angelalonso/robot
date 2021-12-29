from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start motor_workers in the brain namespace
    motor_right_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_motor_right_worker.py',
            name='node_motor_right_worker'
        )

    motor_left_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_motor_left_worker.py',
            name='node_motor_left_worker'
        )
        
    # Read from the Serial Link that connects to the Arduino
    node_arduino = Node(
            package='brain',
            namespace='brain',
            executable='node_arduino.py',
            name='node_arduino'
        )

    # Manage Status
    node_status = Node(
            package='brain',
            namespace='brain',
            executable='status.py',
            name='node_status'
        )

    # Manage Status using a Service
    service_status = Node(
            package='brain',
            namespace='brain',
            executable='service_status.py',
            name='service_status'
        )

    # API for remote control
    node_api = Node(
            package='brain',
            namespace='brain',
            executable='node_api.py',
            name='node_api'
        )

    # start the main brain manager in the brain namespace
    main_brain = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            name='main_brain'
        )

    return LaunchDescription([
        main_brain,
        node_api,
        service_status,
        node_status,
        node_arduino,
        motor_right_worker_node,
        motor_left_worker_node,
    ])
