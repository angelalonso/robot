from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    # start a motors_manager in the brain namespace
    main_brain = Node(
            package='brain',
            namespace='brain',
            executable='main_brain.py',
            name='main_brain'
        )

    return LaunchDescription([
        main_brain,
    ])
