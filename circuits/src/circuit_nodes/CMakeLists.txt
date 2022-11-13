cmake_minimum_required(VERSION 3.8)
project(circuit_nodes)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(action_interfaces REQUIRED)
find_package(rclcpp REQUIRED)
find_package(rclcpp_action REQUIRED)
find_package(rclcpp_components REQUIRED)

install(DIRECTORY
  launch
  DESTINATION share/${PROJECT_NAME}
)

add_executable(node_master src/node_master.cpp)
ament_target_dependencies(node_master
  "action_interfaces"
  "rclcpp"
  "rclcpp_action"
  "rclcpp_components")
install(TARGETS
  node_master
  DESTINATION lib/circuit_nodes)

#
#
#add_library(led_action_srvr SHARED
#  src/led_action_server.cpp)
#target_include_directories(led_action_srvr PRIVATE
#  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
#  $<INSTALL_INTERFACE:include>)
#target_compile_definitions(led_action_srvr
#  PRIVATE "CIRCUIT_NODES_BUILDING_DLL")
#ament_target_dependencies(led_action_srvr
#  "action_interfaces"
#  "rclcpp"
#  "rclcpp_action"
#  "rclcpp_components")
#rclcpp_components_register_node(led_action_srvr PLUGIN "circuit_nodes::LedActionServer" EXECUTABLE led_action_server)
#install(TARGETS
#  led_action_srvr
#  ARCHIVE DESTINATION lib
#  LIBRARY DESTINATION lib
#  RUNTIME DESTINATION bin)
#
#
#add_library(motor_l_action_srvr SHARED
#  src/motor_l_action_server.cpp)
#target_include_directories(motor_l_action_srvr PRIVATE
#  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
#  $<INSTALL_INTERFACE:include>)
#target_compile_definitions(motor_l_action_srvr
#  PRIVATE "CIRCUIT_NODES_BUILDING_DLL")
#ament_target_dependencies(motor_l_action_srvr
#  "action_interfaces"
#  "rclcpp"
#  "rclcpp_action"
#  "rclcpp_components")
#rclcpp_components_register_node(motor_l_action_srvr PLUGIN "circuit_nodes::MotorLActionServer" EXECUTABLE motor_l_action_server)
#install(TARGETS
#  motor_l_action_srvr
#  ARCHIVE DESTINATION lib
#  LIBRARY DESTINATION lib
#  RUNTIME DESTINATION bin)
#
#
#add_library(motor_r_action_srvr SHARED
#  src/motor_r_action_server.cpp)
#target_include_directories(motor_r_action_srvr PRIVATE
#  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
#  $<INSTALL_INTERFACE:include>)
#target_compile_definitions(motor_r_action_srvr
#  PRIVATE "CIRCUIT_NODES_BUILDING_DLL")
#ament_target_dependencies(motor_r_action_srvr
#  "action_interfaces"
#  "rclcpp"
#  "rclcpp_action"
#  "rclcpp_components")
#rclcpp_components_register_node(motor_r_action_srvr PLUGIN "circuit_nodes::MotorRActionServer" EXECUTABLE motor_r_action_server)
#install(TARGETS
#  motor_r_action_srvr
#  ARCHIVE DESTINATION lib
#  LIBRARY DESTINATION lib
#  RUNTIME DESTINATION bin)


if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  # the following line skips the linter which checks for copyrights
  # comment the line when a copyright and license is added to all source files
  set(ament_cmake_copyright_FOUND TRUE)
  # the following line skips cpplint (only works in a git repo)
  # comment the line when this package is in a git repo and when
  # a copyright and license is added to all source files
  set(ament_cmake_cpplint_FOUND TRUE)
  ament_lint_auto_find_test_dependencies()
endif()

ament_package()
