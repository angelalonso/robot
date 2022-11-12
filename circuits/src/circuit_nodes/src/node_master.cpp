#include <chrono>
#include <cinttypes>

#include "action_client.hpp"
#include "action_interfaces/action/led.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"

using Led = action_interfaces::action::Led;

#include <iostream>
using namespace std;

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  auto led_ac_node = rclcpp::Node::make_shared("led_action_client");
  auto led_ac = rclcpp_action::create_client<Led>(led_ac_node, "led");

  if (!led_ac->wait_for_action_server(std::chrono::seconds(20))) {
    RCLCPP_ERROR(led_ac_node->get_logger(), "Action server not available after waiting");
    return 1;
  }

  string led_ac_name = "led";
  ActionClient* led_ac_obj = new ActionClient(led_ac, led_ac_node, led_ac_name);
  std::string msg = "####################" + led_ac_obj->getid();
  RCLCPP_INFO(led_ac_node->get_logger(), msg.c_str());

  // Populate a goal
  auto led_goal_msg = Led::Goal();
  led_goal_msg.turn_on = 1;
  led_ac_obj->send_goal(led_goal_msg);
  //delete ac;

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
