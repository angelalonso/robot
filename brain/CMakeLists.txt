cmake_minimum_required(VERSION 3.8)
project(brain)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(rclcpp_action REQUIRED)
find_package(rclcpp_components REQUIRED)

find_package(ament_cmake_python REQUIRED)
find_package(rclpy REQUIRED)

if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  # the following line skips the linter which checks for copyrights
  # uncomment the line when a copyright and license is not present in all source files
  #set(ament_cmake_copyright_FOUND TRUE)
  # the following line skips cpplint (only works in a git repo)
  # uncomment the line when this package is not in a git repo
  #set(ament_cmake_cpplint_FOUND TRUE)
  ament_lint_auto_find_test_dependencies()
endif()

add_executable(brain_boot
  src/brain_boot.cpp
)
ament_target_dependencies(brain_boot
  "rclcpp"
)

# Install cpp executables
install(TARGETS
  brain_boot
DESTINATION lib/${PROJECT_NAME}
)


# Install python scripts - MODULES et al.
# Auxiliar libraries
install(PROGRAMS
  scripts/lib_fake_rpi.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/action_clients.py
  DESTINATION lib/${PROJECT_NAME}
)
# Nodes
install(PROGRAMS
  scripts/node_status.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_api.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_led.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_motor_right.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_motor_left.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_servo_laser.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_arduino.py
  DESTINATION lib/${PROJECT_NAME}
)
# The main brain
#install(PROGRAMS
#  scripts/main_brain.py
#  DESTINATION lib/${PROJECT_NAME}
#)

find_package(rosidl_default_generators REQUIRED)

set(action_files
  "action/Led.action"
  "action/Motor.action"
  "action/Servo.action"
  "action/Getstatus.action"
  "action/Whatstatus.action"
  )

rosidl_generate_interfaces(${PROJECT_NAME}
  ${action_files}
)

# Install launch files.
install(DIRECTORY
  launch
  DESTINATION share/${PROJECT_NAME}/
)

ament_package()
