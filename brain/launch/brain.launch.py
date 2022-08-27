from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start led in the brain namespace
    main_brain = Node(
            package='brain',
            namespace='brain',
            executable='node_led.py',
            name='node_led'
        )

    # start motor_workers in the brain namespace
    motor_right_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_motor_right.py',
            name='node_motor_right_worker'
        )

    motor_left_worker_node = Node(
            package='brain',
            namespace='brain',
            executable='node_motor_left.py',
            name='node_motor_left_worker'
        )
        
    # start servo_worker for the laser  in the brain namespace
    node_servo_laser = Node(
            package='brain',
            namespace='brain',
            executable='node_servo_laser.py',
            name='node_servo_laser'
        )

    # Read from the Serial Link that connects to the Arduino
    node_arduino = Node(
            package='brain',
            namespace='brain',
            executable='node_arduino.py',
            name='node_arduino'
        )

    # Manage Status using a Topic ## REMEMBER a Service is CPUllshit!
    publisher_status = Node(
            package='brain',
            namespace='brain',
            executable='publisher_status.py',
            name='publisher_status'
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
        publisher_status,
        node_arduino,
        node_servo_laser,
        motor_right_worker_node,
        motor_left_worker_node,
    ])
