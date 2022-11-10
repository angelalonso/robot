#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp/rclcpp.hpp"

#include <iostream>
using namespace std;

class ActionClient {
public:
    string a, c;
// TODO: https://github.com/ros2/rclcpp/blob/rolling/rclcpp_action/include/rclcpp_action/client.hpp
    ActionClient(rclcpp::Node node, rclcpp_action::Client<typename ActionT> action, string serverid) {
      a = rclcpp_action::create_client<action>(node, serverid);

      if (!a->wait_for_action_server(std::chrono::seconds(20))) {
        RCLCPP_ERROR(node->get_logger(), "Action server not available after waiting");
        return 1;
      }
      return a;
    }
};

