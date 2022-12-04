#include <functional>
#include <memory>
#include <thread>

#include <iostream>
#include <fstream>
//#include <string>
#define LED_PATH "/sys/class/leds/beaglebone:green:usr0"
//
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>


#include "action_interfaces/action/led.hpp"
#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "rclcpp_components/register_node_macro.hpp"

#include "action_servers/visibility_control.h"

using namespace std;

namespace action_servers {
  class LedActionServer : public rclcpp::Node {
    public:
      using Led = action_interfaces::action::Led;
      using GoalHandleLed = rclcpp_action::ServerGoalHandle<Led>;

      ACTION_SERVERS_PUBLIC
      explicit LedActionServer(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
      : Node("led_action_server", options) {
        using namespace std::placeholders;

        this->action_server_ = rclcpp_action::create_server<Led>(
          this,
          "led",
          std::bind(&LedActionServer::handle_goal, this, _1, _2),
          std::bind(&LedActionServer::handle_cancel, this, _1),
          std::bind(&LedActionServer::handle_accepted, this, _1));
        prepare_led();
      }

      int prepare_led() {
        int fd = open("/sys/class/gpio/export", O_WRONLY);
        if (fd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/export");
            exit(1);
        }

        // TODO: investigate why this is a -1 and the one below is a 3
        if (write(fd, "21", 2) != -1 ) {
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/export");
            exit(1);
        }

        close(fd);

        int fdd = open("/sys/class/gpio/gpio21/direction", O_WRONLY);
        if (fdd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/gpio21/direction");
            exit(1);
        }

        if (write(fdd, "out", 3) != 3 ) {
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/direction");
            exit(1);
        }

        close(fdd);
        return 0;
      }

    private:

      int do_led(string status) {
        int fd = open("/sys/class/gpio/gpio21/value", O_WRONLY);
        //RCLCPP_INFO(this->get_logger(), "--------------------- OPEN /sys/class/gpio/gpio21/value");
        if (fd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/gpio21/value");
            exit(1);
        }

        string cmd(status);
        if(cmd=="on"){
          //RCLCPP_INFO(this->get_logger(), " ----------------------------------- ON");
          if (write(fd, "1", 1) != 1) {
              RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/value");
              exit(1);
          }
        } else if (cmd=="off"){
          //RCLCPP_INFO(this->get_logger(), " ----------------------------------- OFF");
          if (write(fd, "0", 1) != 1) {
              RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/value");
              exit(1);
          }
        }

        close(fd);

        // TODO: do this on closing the class
        // Unexport the pin by writing to /sys/class/gpio/unexport

        fd = open("/sys/class/gpio/unexport", O_WRONLY);
        if (fd == -1) {
            //RCLCPP_INFO(this->get_logger(), "--------------------- OPEN /sys/class/gpio/unexport");
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/unexport");
            exit(1);
        }

        //write(fd, "24", 2);
        if (write(fd, "24", 2) != -1 ) {
            //RCLCPP_INFO(this->get_logger(), "--------------------- WROTE TO /sys/class/gpio/unexport");
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/unexport");
            exit(1);
        }

        close(fd);

        // And exit
        return 0;
      }

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

        //Maybe avoid using on and off and use 1 and 0 instead directly
        if (goal->turn_on == 1) {
          do_led("on");
         } else {
          do_led("off");
        };

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

}  // namespace action_servers

RCLCPP_COMPONENTS_REGISTER_NODE(action_servers::LedActionServer)
