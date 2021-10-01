cmake_minimum_required(VERSION 3.5)

project(brain)

# Default to C++14
if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 14)
endif()

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)

find_package(ament_cmake_python REQUIRED)
find_package(rclpy REQUIRED)

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

# Install python scripts
install(PROGRAMS
  scripts/fake_rpi.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_right_motor_worker.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_left_motor_worker.py
  DESTINATION lib/${PROJECT_NAME}
)
install(PROGRAMS
  scripts/node_motors_manager.py
  DESTINATION lib/${PROJECT_NAME}
)

find_package(rosidl_default_generators REQUIRED)

set(action_files
  "action/Led.action"
  "action/Motor.action"
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
