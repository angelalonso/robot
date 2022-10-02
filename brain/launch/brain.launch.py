from launch import LaunchDescription
from launch.actions import LogInfo
from launch.actions import Shutdown
from launch_ros.actions import Node


def generate_launch_description():

    emit_shutdown_action = Shutdown(reason='launch is shutting down')
#    # start the main brain manager in the brain namespace
#    main_brain = Node(
#            package='brain',
#            namespace='brain',
#            executable='main_brain.py',
#            name='main_brain'
#        )

    # API for remote control
    node_api = Node(
            package='brain',
            namespace='brain',
            executable='node_api.py',
            name='node_api'
        )

    # Manage Status using a Rust program ## REMEMBER a Service is CPUllshit!
    node_status = Node(
            package='brain',
            namespace='brain',
            executable='node_status.py',
            name='node_status',
            on_exit=[LogInfo(msg=["STATUS Node 2 stopped. Stopping everything..."]),
                     emit_shutdown_action],
        )

    # Read from the Serial Link that connects to the Arduino
    node_arduino = Node(
            package='brain',
            namespace='brain',
            executable='node_arduino.py',
            name='node_arduino'
        )

    # start led in the brain namespace
    node_led = Node(
            package='brain',
            namespace='brain',
            executable='node_led.py',
            name='node_led'
        )

    # start servo_worker for the laser in the brain namespace
    node_servo_laser = Node(
            package='brain',
            namespace='brain',
            executable='node_servo_laser.py',
            name='node_servo_laser'
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

    return LaunchDescription([
#        main_brain,
        node_api,
        node_status,
        node_arduino,
        node_led,
        node_servo_laser,
        motor_right_worker_node,
        motor_left_worker_node,
    ])
