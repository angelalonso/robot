from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    ld = LaunchDescription()
    node_master = Node(
        package="circuit_nodes",
        executable="node_master",
    )
    led_action_server = Node(
        package="circuit_nodes",
        executable="led_action_server",
    )
    motor_l_action_server = Node(
        package="circuit_nodes",
        executable="motor_l_action_server",
    )
    motor_r_action_server = Node(
        package="circuit_nodes",
        executable="motor_r_action_server",
    )
    ld.add_action(node_master)
    ld.add_action(led_action_server)
    ld.add_action(motor_l_action_server)
    ld.add_action(motor_r_action_server)
    return ld
