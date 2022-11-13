#include <wiringPi.h>
#include <functional>
#include <memory>
#include <thread>

#include "action_interfaces/action/led.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp_components/register_node_macro.hpp"

#include "circuit_nodes/visibility_control.h"

namespace circuit_nodes {
  class LedActionServer : public rclcpp::Node {
    public:
      using Led = action_interfaces::action::Led;
      using GoalHandleLed = rclcpp_action::ServerGoalHandle<Led>;

      CIRCUIT_NODES_PUBLIC
      explicit LedActionServer(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
      : Node("led_action_server", options) {
        using namespace std::placeholders;

        this->action_server_ = rclcpp_action::create_server<Led>(
          this,
          "led",
          std::bind(&LedActionServer::handle_goal, this, _1, _2),
          std::bind(&LedActionServer::handle_cancel, this, _1),
          std::bind(&LedActionServer::handle_accepted, this, _1));
      }

    private:
      rclcpp_action::Server<Led>::SharedPtr action_server_;

      rclcpp_action::GoalResponse handle_goal(
        const rclcpp_action::GoalUUID & uuid,
        std::shared_ptr<const Led::Goal> goal) {
          RCLCPP_INFO(this->get_logger(), "Received goal request with turn_on %d", goal->turn_on);
          (void)uuid;
          return rclcpp_action::GoalResponse::ACCEPT_AND_EXECUTE;
        }

      rclcpp_action::CancelResponse handle_cancel(
        const std::shared_ptr<GoalHandleLed> goal_handle) {
          RCLCPP_INFO(this->get_logger(), "Received request to cancel goal");
          (void)goal_handle;
          return rclcpp_action::CancelResponse::ACCEPT;
        }

      void handle_accepted(const std::shared_ptr<GoalHandleLed> goal_handle) {
        using namespace std::placeholders;
        // this needs to return quickly to avoid blocking the executor, so spin up a new thread
        std::thread{std::bind(&LedActionServer::execute, this, _1), goal_handle}.detach();
      }

      void execute(const std::shared_ptr<GoalHandleLed> goal_handle) {
        RCLCPP_INFO(this->get_logger(), "Executing goal");
        rclcpp::Rate loop_rate(1);
        const auto goal = goal_handle->get_goal();
        auto feedback = std::make_shared<Led::Feedback>();
        auto & confirmed = feedback->process_feed;
        auto result = std::make_shared<Led::Result>();

        for (int i = 1; (i < goal->turn_on) && rclcpp::ok(); ++i) {
          // Check if there is a cancel request
          if (goal_handle->is_canceling()) {
            result->confirmed = confirmed;
            goal_handle->canceled(result);
            RCLCPP_INFO(this->get_logger(), "Goal canceled");
            return;
          }
          // Update confirmed
          // Publish feedback
          goal_handle->publish_feedback(feedback);
          RCLCPP_INFO(this->get_logger(), "Publish feedback");

          loop_rate.sleep();
        }

        // Check if goal is done
        if (rclcpp::ok()) {
          result->confirmed = confirmed;
          goal_handle->succeed(result);
          RCLCPP_INFO(this->get_logger(), "Goal succeeded");
        }
      }
  };  // class LedActionServer

}  // namespace circuit_nodes

RCLCPP_COMPONENTS_REGISTER_NODE(circuit_nodes::LedActionServer)
