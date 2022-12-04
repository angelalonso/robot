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
      }

    private:
      void removeTrigger(){
        // remove the trigger from the LED
        std::fstream fs;
        fs.open( LED_PATH "/trigger", std::fstream::out);
        fs << "none";
        fs.close();
      }

      int new_do_led(string status) {
        return 0;
        // Export the desired pin by writing to /sys/class/gpio/export

        int fd = open("/sys/class/gpio/export", O_WRONLY);
        if (fd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/export");
            exit(1);
        }
        // TODO: use LEDMAIN_PIN from .env
        if (write(fd, "21", 2) != 2) {
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/export");
            exit(1);
        }
        RCLCPP_INFO(this->get_logger(), "--------------------- WROTE TO /sys/class/gpio/export");

        close(fd);

        // Set the pin to be an output by writing "out" to /sys/class/gpio/gpio24/direction

        fd = open("/sys/class/gpio/gpio21/direction", O_WRONLY);
        RCLCPP_INFO(this->get_logger(), "--------------------- OPEN /sys/class/gpio/gpio21/direction");
        if (fd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/gpio21/direction");
            exit(1);
        }

        RCLCPP_INFO(this->get_logger(), "--------------------- wrote OUT /sys/class/gpio21/direction");
        if (write(fd, "out", 3) != 3) {
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/direction");
            exit(1);
        }

        close(fd);


        fd = open("/sys/class/gpio/gpio21/value", O_WRONLY);
        RCLCPP_INFO(this->get_logger(), "--------------------- OPEN /sys/class/gpio/gpio21/value");
        if (fd == -1) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/gpio21/value");
            exit(1);
        }
  
        string cmd(status);
        if(cmd=="on"){
          RCLCPP_INFO(this->get_logger(), " ----------------------------------- ON");
          RCLCPP_INFO(this->get_logger(), "--------------------- WRITE 1 /sys/class/gpio/gpio21/value");
          if (write(fd, "1", 1) != 1) {
              RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/value");
              exit(1);
          }
        } else if (cmd=="off"){
          RCLCPP_INFO(this->get_logger(), " ----------------------------------- OFF");
          RCLCPP_INFO(this->get_logger(), "--------------------- WRITE 0 /sys/class/gpio/gpio21/value");
          if (write(fd, "0", 1) != 1) {
              RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/value");
              exit(1);
          }
        }

        close(fd);

        // Unexport the pin by writing to /sys/class/gpio/unexport

        fd = open("/sys/class/gpio/unexport", O_WRONLY);
        if (fd == -1) {
            RCLCPP_INFO(this->get_logger(), "--------------------- OPEN /sys/class/gpio/unexport");
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/unexport");
            exit(1);
        }

        if (write(fd, "24", 2) != 2) {
            RCLCPP_INFO(this->get_logger(), "--------------------- WROTE TO /sys/class/gpio/unexport");
            RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/unexport");
            exit(1);
        }

        close(fd);

        // And exit
        return 0;
      }

      int do_led(string status) {
        string cmd(status);
        std::fstream fs;
        cout << "Starting the LED flash program" << endl;
        cout << "The LED Path is: " << LED_PATH << endl;

        // select whether it is on, off or flash
        if(cmd=="on"){
          RCLCPP_INFO(this->get_logger(), " ----------------------------------- ON");
          removeTrigger();
          fs.open (LED_PATH "/brightness", std::fstream::out);
          fs << "1";
          fs.close();
        }
        else if (cmd=="off"){
          RCLCPP_INFO(this->get_logger(), " ----------------------------------- OFF");
          removeTrigger();
          fs.open (LED_PATH "/brightness", std::fstream::out);
          fs << "0";
          fs.close();
        }
        else if (cmd=="flash"){
          fs.open (LED_PATH "/trigger", std::fstream::out);
          fs << "timer";
          fs.close();
          fs.open (LED_PATH "/delay_on", std::fstream::out);
          fs << "50";
          fs.close();
          fs.open (LED_PATH "/delay_off", std::fstream::out);
          fs << "50";
          fs.close();
        }
        else if (cmd=="status"){
          // display the current trigger details
          fs.open( LED_PATH "/trigger", std::fstream::in);
          string line;
          while(getline(fs,line)) cout << line;
          fs.close();
        }
        else{
          cout << "Invalid command" << endl;
        }
        cout << "Finished the LED flash program" << endl;
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
          new_do_led("on");
         } else {
          new_do_led("off");
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
