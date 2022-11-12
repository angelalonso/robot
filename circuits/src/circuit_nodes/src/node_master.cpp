#include <chrono>
#include <cinttypes>

#include "action_client.hpp"

#include "action_interfaces/action/led.hpp"
#include "action_interfaces/action/motor_l.hpp"
#include "action_interfaces/action/motor_r.hpp"

#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"

using Led = action_interfaces::action::Led;
using MotorL = action_interfaces::action::MotorL;
using MotorR = action_interfaces::action::MotorR;

#include <iostream>
using namespace std;

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  // LED //
  auto led_ac_node = rclcpp::Node::make_shared("led_action_client");
  string led_ac_name = "led";
  auto led_ac = rclcpp_action::create_client<Led>(led_ac_node, led_ac_name);

  if (!led_ac->wait_for_action_server(std::chrono::seconds(20))) {
    RCLCPP_ERROR(led_ac_node->get_logger(), "Action server not available after waiting");
    return 1;
  }

  ActionClient* led_ac_obj = new ActionClient(led_ac, led_ac_node, led_ac_name);
  std::string msg = "####################" + led_ac_obj->getid();
  RCLCPP_INFO(led_ac_node->get_logger(), msg.c_str());

  // Populate a goal
  auto led_goal_msg = Led::Goal();
  led_goal_msg.turn_on = 1;
  led_ac_obj->send_goal(led_goal_msg);
  //delete ac;
  // LED - END //

  // MOTOR LEFT //
  auto motor_l_ac_node = rclcpp::Node::make_shared("motor_l_action_client");
  auto motor_l_ac = rclcpp_action::create_client<MotorL>(motor_l_ac_node, "motor_l");

  if (!motor_l_ac->wait_for_action_server(std::chrono::seconds(20))) {
    RCLCPP_ERROR(motor_l_ac_node->get_logger(), "Action server not available after waiting");
    return 1;
  }

  string motor_l_ac_name = "motor_l";
  ActionClient* motor_l_ac_obj = new ActionClient(motor_l_ac, motor_l_ac_node, motor_l_ac_name);
  msg = "####################" + motor_l_ac_obj->getid();
  RCLCPP_INFO(motor_l_ac_node->get_logger(), msg.c_str());

  // Populate a goal
  auto motor_l_goal_msg = MotorL::Goal();
  motor_l_goal_msg.speed = 1;
  motor_l_ac_obj->send_goal(motor_l_goal_msg);
  // MOTOR LEFT - END//
  
  // MOTOR RIGHT //
  auto motor_r_ac_node = rclcpp::Node::make_shared("motor_r_action_client");
  auto motor_r_ac = rclcpp_action::create_client<MotorR>(motor_r_ac_node, "motor_r");

  if (!motor_r_ac->wait_for_action_server(std::chrono::seconds(20))) {
    RCLCPP_ERROR(motor_r_ac_node->get_logger(), "Action server not available after waiting");
    return 1;
  }

  string motor_r_ac_name = "motor_r";
  ActionClient* motor_r_ac_obj = new ActionClient(motor_r_ac, motor_r_ac_node, motor_r_ac_name);
  msg = "####################" + motor_r_ac_obj->getid();
  RCLCPP_INFO(motor_r_ac_node->get_logger(), msg.c_str());

  // Populate a goal
  auto motor_r_goal_msg = MotorR::Goal();
  motor_r_goal_msg.speed = 0;
  motor_r_ac_obj->send_goal(motor_r_goal_msg);
  // MOTOR RIGHT - END//

  /*

  RCLCPP_INFO(node->get_logger(), "Sending goal");
  // Ask server to achieve some goal and wait until it's accepted
  auto goal_handle_future = action_client->async_send_goal(goal_msg);
  if (rclcpp::spin_until_future_complete(node, goal_handle_future) !=
    rclcpp::FutureReturnCode::SUCCESS)
  {
    RCLCPP_ERROR(node->get_logger(), "send goal call failed :(");
    return 1;
  }

  rclcpp_action::ClientGoalHandle<Led>::SharedPtr goal_handle = goal_handle_future.get();
  if (!goal_handle) {
    RCLCPP_ERROR(node->get_logger(), "Goal was rejected by server");
    return 1;
  }

  // Wait for the server to be done with the goal
  auto result_future = action_client->async_get_result(goal_handle);

  RCLCPP_INFO(node->get_logger(), "Waiting for result");
  if (rclcpp::spin_until_future_complete(node, result_future) !=
    rclcpp::FutureReturnCode::SUCCESS)
  {
    RCLCPP_ERROR(node->get_logger(), "get result call failed :(");
    return 1;
  }

  rclcpp_action::ClientGoalHandle<Led>::WrappedResult wrapped_result = result_future.get();

  switch (wrapped_result.code) {
    case rclcpp_action::ResultCode::SUCCEEDED:
      break;
    case rclcpp_action::ResultCode::ABORTED:
      RCLCPP_ERROR(node->get_logger(), "Goal was aborted");
      return 1;
    case rclcpp_action::ResultCode::CANCELED:
      RCLCPP_ERROR(node->get_logger(), "Goal was canceled");
      return 1;
    default:
      RCLCPP_ERROR(node->get_logger(), "Unknown result code");
      return 1;
  }

  RCLCPP_INFO(node->get_logger(), "result received");
  for (auto number : wrapped_result.result->confirmed) {
    RCLCPP_INFO(node->get_logger(), "%" PRId32, number);
  }
  */

  rclcpp::shutdown();
  return 0;
}
