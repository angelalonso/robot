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

  ActionClient* led_ac_obj = new ActionClient(led_ac_node, led_ac_name);
  //std::string msg = "####################" + led_ac_obj->getid();
  //RCLCPP_INFO(led_ac_node->get_logger(), msg.c_str());
  // Populate a goal
  auto led_goal_msg = Led::Goal();
  led_goal_msg.turn_on = 1;
  led_ac_obj->send_goal(led_goal_msg);
  //delete ac;
  // LED - END //

  // MOTOR LEFT //
  auto motor_l_ac_node = rclcpp::Node::make_shared("motor_l_action_client");
  string motor_l_ac_name = "motor_l";
  ActionClient* motor_l_ac_obj = new ActionClient(motor_l_ac_node, motor_l_ac_name);
  //msg = "####################" + motor_l_ac_obj->getid();
  //RCLCPP_INFO(motor_l_ac_node->get_logger(), msg.c_str());
  // Populate a goal
  auto motor_l_goal_msg = MotorL::Goal();
  motor_l_goal_msg.speed = 1;
  motor_l_ac_obj->send_goal(motor_l_goal_msg);
  // MOTOR LEFT - END//
  
  // MOTOR RIGHT //
  auto motor_r_ac_node = rclcpp::Node::make_shared("motor_r_action_client");
  string motor_r_ac_name = "motor_r";
  ActionClient* motor_r_ac_obj = new ActionClient(motor_r_ac_node, motor_r_ac_name);
  //msg = "####################" + motor_r_ac_obj->getid();
  //RCLCPP_INFO(motor_r_ac_node->get_logger(), msg.c_str());
  // Populate a goal
  auto motor_r_goal_msg = MotorR::Goal();
  motor_r_goal_msg.speed = 0;
  motor_r_ac_obj->send_goal(motor_r_goal_msg);
  // MOTOR RIGHT - END//

  rclcpp::shutdown();
  return 0;
}
