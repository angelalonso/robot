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
  scripts/py_test.py
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
