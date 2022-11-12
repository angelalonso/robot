#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"

#include "action_interfaces/action/led.hpp"
#include "action_interfaces/action/motor_l.hpp"
#include "action_interfaces/action/motor_r.hpp"
using Led = action_interfaces::action::Led;
using MotorL = action_interfaces::action::MotorL;
using MotorR = action_interfaces::action::MotorR;

#include <iostream>
using namespace std;

class ActionClient {
  public:
    std::shared_ptr<rclcpp_action::Client<action_interfaces::action::Led>> led_ac;
    std::shared_ptr<rclcpp_action::Client<action_interfaces::action::MotorL>> motor_l_ac;
    std::shared_ptr<rclcpp_action::Client<action_interfaces::action::MotorR>> motor_r_ac;

    std::shared_ptr<rclcpp::Node> nd;
    string id;

    ActionClient(std::shared_ptr<rclcpp_action::Client<action_interfaces::action::Led>> led_action, 
        std::shared_ptr<rclcpp::Node> node, 
        string serverid) {
      led_ac = led_action;
      nd = node;
      id = serverid;
    }
    ActionClient(std::shared_ptr<rclcpp_action::Client<action_interfaces::action::MotorL>> motor_l_action, 
        std::shared_ptr<rclcpp::Node> node, 
        string serverid) {
      motor_l_ac = motor_l_action;
      nd = node;
      id = serverid;
    }
    ActionClient(std::shared_ptr<rclcpp_action::Client<action_interfaces::action::MotorR>> motor_r_action, 
        std::shared_ptr<rclcpp::Node> node, 
        string serverid) {
      motor_r_ac = motor_r_action;
      nd = node;
      id = serverid;
    }

    ~ActionClient();

    string getid() {
      return id;
    }

    int send_goal(action_interfaces::action::Led_Goal_<std::allocator<void>>& actint) {
      RCLCPP_INFO(nd->get_logger(), "Sending goal");
      // Ask server to achieve some goal and wait until it's accepted
      auto goal_handle_future = led_ac->async_send_goal(actint);
      if (rclcpp::spin_until_future_complete(nd, goal_handle_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "send goal call failed :(");
        return 1;
      }
      rclcpp_action::ClientGoalHandle<Led>::SharedPtr goal_handle = goal_handle_future.get();
      if (!goal_handle) {
        RCLCPP_ERROR(nd->get_logger(), "Goal was rejected by server");
        return 1;
      }

      // Wait for the server to be done with the goal
      // TODO: use ac everywhere??
      auto result_future = led_ac->async_get_result(goal_handle);

      RCLCPP_INFO(nd->get_logger(), "Waiting for result");
      if (rclcpp::spin_until_future_complete(nd, result_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "get result call failed :(");
        return 1;
      }

      rclcpp_action::ClientGoalHandle<Led>::WrappedResult wrapped_result = result_future.get();

      switch (wrapped_result.code) {
        case rclcpp_action::ResultCode::SUCCEEDED:
          break;
        case rclcpp_action::ResultCode::ABORTED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was aborted");
          return 1;
        case rclcpp_action::ResultCode::CANCELED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was canceled");
          return 1;
        default:
          RCLCPP_ERROR(nd->get_logger(), "Unknown result code");
          return 1;
      }

      RCLCPP_INFO(nd->get_logger(), "result received");
      for (auto number : wrapped_result.result->confirmed) {
        RCLCPP_INFO(nd->get_logger(), "%" PRId32, number);
      }
      return 0;
    }
    int send_goal(action_interfaces::action::MotorL_Goal_<std::allocator<void>>& actint) {
      RCLCPP_INFO(nd->get_logger(), "Sending goal");
      // Ask server to achieve some goal and wait until it's accepted
      auto goal_handle_future = motor_l_ac->async_send_goal(actint);
      if (rclcpp::spin_until_future_complete(nd, goal_handle_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "send goal call failed :(");
        return 1;
      }
      rclcpp_action::ClientGoalHandle<MotorL>::SharedPtr goal_handle = goal_handle_future.get();
      if (!goal_handle) {
        RCLCPP_ERROR(nd->get_logger(), "Goal was rejected by server");
        return 1;
      }

      // Wait for the server to be done with the goal
      // TODO: use ac everywhere??
      auto result_future = motor_l_ac->async_get_result(goal_handle);

      RCLCPP_INFO(nd->get_logger(), "Waiting for result");
      if (rclcpp::spin_until_future_complete(nd, result_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "get result call failed :(");
        return 1;
      }

      rclcpp_action::ClientGoalHandle<MotorL>::WrappedResult wrapped_result = result_future.get();

      switch (wrapped_result.code) {
        case rclcpp_action::ResultCode::SUCCEEDED:
          break;
        case rclcpp_action::ResultCode::ABORTED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was aborted");
          return 1;
        case rclcpp_action::ResultCode::CANCELED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was canceled");
          return 1;
        default:
          RCLCPP_ERROR(nd->get_logger(), "Unknown result code");
          return 1;
      }

      RCLCPP_INFO(nd->get_logger(), "result received");
      for (auto number : wrapped_result.result->confirmed) {
        RCLCPP_INFO(nd->get_logger(), "%" PRId32, number);
      }
      return 0;
    }
    int send_goal(action_interfaces::action::MotorR_Goal_<std::allocator<void>>& actint) {
      RCLCPP_INFO(nd->get_logger(), "Sending goal");
      // Ask server to achieve some goal and wait until it's accepted
      auto goal_handle_future = motor_r_ac->async_send_goal(actint);
      if (rclcpp::spin_until_future_complete(nd, goal_handle_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "send goal call failed :(");
        return 1;
      }
      rclcpp_action::ClientGoalHandle<MotorR>::SharedPtr goal_handle = goal_handle_future.get();
      if (!goal_handle) {
        RCLCPP_ERROR(nd->get_logger(), "Goal was rejected by server");
        return 1;
      }

      // Wait for the server to be done with the goal
      // TODO: use ac everywhere??
      auto result_future = motor_r_ac->async_get_result(goal_handle);

      RCLCPP_INFO(nd->get_logger(), "Waiting for result");
      if (rclcpp::spin_until_future_complete(nd, result_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "get result call failed :(");
        return 1;
      }

      rclcpp_action::ClientGoalHandle<MotorR>::WrappedResult wrapped_result = result_future.get();

      switch (wrapped_result.code) {
        case rclcpp_action::ResultCode::SUCCEEDED:
          break;
        case rclcpp_action::ResultCode::ABORTED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was aborted");
          return 1;
        case rclcpp_action::ResultCode::CANCELED:
          RCLCPP_ERROR(nd->get_logger(), "Goal was canceled");
          return 1;
        default:
          RCLCPP_ERROR(nd->get_logger(), "Unknown result code");
          return 1;
      }

      RCLCPP_INFO(nd->get_logger(), "result received");
      for (auto number : wrapped_result.result->confirmed) {
        RCLCPP_INFO(nd->get_logger(), "%" PRId32, number);
      }
      return 0;
    }
};

