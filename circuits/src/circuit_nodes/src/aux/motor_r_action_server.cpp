#include <functional>
#include <memory>
#include <thread>

#include "action_interfaces/action/motor_r.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp_components/register_node_macro.hpp"

#include "circuit_nodes/visibility_control.h"

namespace circuit_nodes {
  class MotorRActionServer : public rclcpp::Node {
    public:
      using MotorR = action_interfaces::action::MotorR;
      using GoalHandleMotorR = rclcpp_action::ServerGoalHandle<MotorR>;

      CIRCUIT_NODES_PUBLIC
      explicit MotorRActionServer(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
      : Node("motor_r_action_server", options) {
        using namespace std::placeholders;

        this->action_server_ = rclcpp_action::create_server<MotorR>(
          this,
          "motor_r",
          std::bind(&MotorRActionServer::handle_goal, this, _1, _2),
          std::bind(&MotorRActionServer::handle_cancel, this, _1),
          std::bind(&MotorRActionServer::handle_accepted, this, _1));
      }

    private:
      rclcpp_action::Server<MotorR>::SharedPtr action_server_;

      rclcpp_action::GoalResponse handle_goal(
        const rclcpp_action::GoalUUID & uuid,
        std::shared_ptr<const MotorR::Goal> goal) {
          RCLCPP_INFO(this->get_logger(), "Received goal request with speed %f", goal->speed);
          (void)uuid;
          return rclcpp_action::GoalResponse::ACCEPT_AND_EXECUTE;
        }

      rclcpp_action::CancelResponse handle_cancel(
        const std::shared_ptr<GoalHandleMotorR> goal_handle) {
          RCLCPP_INFO(this->get_logger(), "Received request to cancel goal");
          (void)goal_handle;
          return rclcpp_action::CancelResponse::ACCEPT;
        }

      void handle_accepted(const std::shared_ptr<GoalHandleMotorR> goal_handle) {
        using namespace std::placeholders;
        // this needs to return quickly to avoid blocking the executor, so spin up a new thread
        std::thread{std::bind(&MotorRActionServer::execute, this, _1), goal_handle}.detach();
      }

      void execute(const std::shared_ptr<GoalHandleMotorR> goal_handle) {
        RCLCPP_INFO(this->get_logger(), "Executing goal");
        rclcpp::Rate loop_rate(1);
        const auto goal = goal_handle->get_goal();
        auto feedback = std::make_shared<MotorR::Feedback>();
        auto & confirmed = feedback->process_feed;
        auto result = std::make_shared<MotorR::Result>();

        for (int i = 1; (i < goal->speed) && rclcpp::ok(); ++i) {
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
  };  // class MotorRActionServer

}  // namespace circuit_nodes

RCLCPP_COMPONENTS_REGISTER_NODE(circuit_nodes::MotorRActionServer)
