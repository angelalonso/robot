#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"

#include "action_interfaces/action/led.hpp"
using Led = action_interfaces::action::Led;

#include <iostream>
using namespace std;

class ActionClient {
  public:
    std::shared_ptr<rclcpp_action::Client<action_interfaces::action::Led>> led_ac;
    std::shared_ptr<rclcpp::Node> nd;
    string id;
  // TODO: https://github.com/ros2/rclcpp/blob/rolling/rclcpp_action/include/rclcpp_action/client.hpp
  // TODO: several constructors for each action
    ActionClient(std::shared_ptr<rclcpp_action::Client<action_interfaces::action::Led>> led_action, 
        std::shared_ptr<rclcpp::Node> node, 
        string serverid) {
      led_ac = led_action;
      nd = node;
      id = serverid;
    }
    ~ActionClient();

    string getid() {
      return id;
    }
    int send_goal(action_interfaces::action::Led_Goal_<std::allocator<void>>& actint) {
      // Ask server to achieve some goal and wait until it's accepted
      auto goal_handle_future = led_ac->async_send_goal(actint);
      if (rclcpp::spin_until_future_complete(nd, goal_handle_future) !=
        rclcpp::FutureReturnCode::SUCCESS)
      {
        RCLCPP_ERROR(nd->get_logger(), "send goal call failed :(");
        return 1;
      }
    }
};

