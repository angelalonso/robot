#include <functional>
#include <future>
#include <memory>
#include <string>
#include <sstream>

#include "brain_interfaces/action/what_status.hpp"

#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp_components/register_node_macro.hpp"


namespace brain
{
class StatusActionServer : public rclcpp::Node
{
public:
  using Status = brain_interfaces::action::WhatStatus;
  using GoalHandleStatus = rclcpp_action::ServerGoalHandle<Status>;

//  ACTION_TUTORIALS_CPP_PUBLIC
  explicit StatusActionServer(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
  : Node("whatstatus_action_server", options)
  {
    using namespace std::placeholders;

    this->action_server_ = rclcpp_action::create_server<Status>(
      this,
      "whatstatus",
      std::bind(&StatusActionServer::handle_goal, this, _1, _2),
      std::bind(&StatusActionServer::handle_cancel, this, _1),
      std::bind(&StatusActionServer::handle_accepted, this, _1));
  }


private:
  rclcpp_action::Server<Status>::SharedPtr action_server_;

  rclcpp_action::GoalResponse handle_goal(
    const rclcpp_action::GoalUUID & uuid,
    std::shared_ptr<const Status::Goal> goal)
  {
    RCLCPP_INFO(this->get_logger(), "Received goal request with key %s", goal->key.c_str());
    (void)uuid;
    return rclcpp_action::GoalResponse::ACCEPT_AND_EXECUTE;
  }

  rclcpp_action::CancelResponse handle_cancel(
    const std::shared_ptr<GoalHandleStatus> goal_handle)
  {
    RCLCPP_INFO(this->get_logger(), "Received request to cancel goal");
    (void)goal_handle;
    return rclcpp_action::CancelResponse::ACCEPT;
  }

  void handle_accepted(const std::shared_ptr<GoalHandleStatus> goal_handle)
  {
    using namespace std::placeholders;
    // this needs to return quickly to avoid blocking the executor, so spin up a new thread
    std::thread{std::bind(&StatusActionServer::execute, this, _1), goal_handle}.detach();
  }

  void execute(const std::shared_ptr<GoalHandleStatus> goal_handle)
  {
    RCLCPP_INFO(this->get_logger(), "Executing goal");
    rclcpp::Rate loop_rate(1);
    const auto goal = goal_handle->get_goal();
    auto feedback = std::make_shared<Status::Feedback>();
    auto & value = feedback->process_feed;
    value.push_back('\0');
    auto result = std::make_shared<Status::Result>();

    //\//for (int i = 1; (i < goal->key) && rclcpp::ok(); ++i) {
    // Check if there is a cancel request
    if (goal_handle->is_canceling()) {
      result->value = value;
      goal_handle->canceled(result);
      RCLCPP_INFO(this->get_logger(), "Goal canceled");
      return;
    }
    // Update value
    //value.push_back(goal->key);
    // Publish feedback
    goal_handle->publish_feedback(feedback);
    RCLCPP_INFO(this->get_logger(), "Publish feedback");

    //\//  loop_rate.sleep();
    //\//}

    // Check if goal is done
    if (rclcpp::ok()) {
      result->value = value;
      goal_handle->succeed(result);
      RCLCPP_INFO(this->get_logger(), "Goal succeeded");
    }
  }
};  // class StatusActionServer

}  // namespace action_tutorials_cpp

RCLCPP_COMPONENTS_REGISTER_NODE(brain::StatusActionServer)
