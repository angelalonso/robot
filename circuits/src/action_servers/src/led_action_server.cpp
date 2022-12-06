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
        int fExp = open("/sys/class/gpio/export", O_WRONLY);
        std::string strFExp = std::to_string(fExp);
        char* tmpFExp = new char[strFExp.length() + 1];
        strcpy(tmpFExp, strFExp.c_str());
        RCLCPP_ERROR(this->get_logger(), "-------------- export open -----------------");
        RCLCPP_ERROR(this->get_logger(), tmpFExp); 
        if (fExp != 17 ) {
          if (fExp != -1 ) {
            RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/export");
            exit(1);
          }
        }

        //usleep(500);
        sleep(1);
        int fExpOut = write(fExp, "21", 2);
        std::string strFExpOut = std::to_string(fExpOut);
        char* tmpFExpOut = new char[strFExpOut.length() + 1];
        strcpy(tmpFExpOut, strFExpOut.c_str());
        RCLCPP_ERROR(this->get_logger(), "-------------- export written -----------------");
        RCLCPP_ERROR(this->get_logger(), tmpFExpOut); 
        if (fExpOut != -1 ) {
          if (fExpOut != 2 ) {
             RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/export");
              exit(1);
          }
        }

        close(fExp);

        int fDir = open("/sys/class/gpio/gpio21/direction", O_WRONLY);
        std::string strFDir = std::to_string(fDir);
        char* tmpFDir = new char[strFDir.length() + 1];
        strcpy(tmpFDir, strFDir.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- direction open ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpFDir); 
        if (fDir != 17 ) {
          if (fDir != -1 ) {
             RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio21/direction");
              exit(1);
          }
        }

        sleep(1);
        int fDirOut = write(fDir, "out", 3);
        std::string strFDirOut = std::to_string(fDirOut);
        char* tmpFDirOut = new char[strFDirOut.length() + 1];
        strcpy(tmpFDirOut, strFDirOut.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- direction written ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpFDirOut); 
        //if (write(fddout, "out", 3) != 3 ) {
        //    RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/gpio21/direction");
        //    exit(1);
        //}

        close(fDir);
        return 0;
      }

    private:

      int do_led(string status) {
        int fVal = open("/sys/class/gpio/gpio21/value", O_WRONLY);
        std::string strFVal = std::to_string(fVal);
        char* tmpFVal = new char[strFVal.length() + 1];
        strcpy(tmpFVal, strFVal.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- value open ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpFVal); 
        //if (fVal == -1) {
        //    RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/gpio21/value");
        //    exit(1);
        //}

        sleep(1);
        string cmd(status);
        int cmdonoff;
        if(cmd=="on"){
          cmdonoff = write(fVal, "1", 1);
        } else if (cmd=="off") {
          cmdonoff = write(fVal, "0", 1);
        } else {
          cmdonoff = write(fVal, "0", 1);
        }
        std::string strCmdout = std::to_string(cmdonoff);
        char* tmpCmdout = new char[strCmdout.length() + 1];
        strcpy(tmpCmdout, strCmdout.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- value written ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpCmdout); 

        close(fVal);

        // TODO: do this on closing the class
        // Unexport the pin by writing to /sys/class/gpio/unexport

        int fUnx = open("/sys/class/gpio/unexport", O_WRONLY);
        std::string strFUnx = std::to_string(fUnx);
        char* tmpFUnx = new char[strFUnx.length() + 1];
        strcpy(tmpFUnx, strFUnx.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- unexport open ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpFUnx); 
        //int fd = open("/sys/class/gpio/unexport", O_WRONLY);
        //if (fd == -1) {
        //    RCLCPP_ERROR(this->get_logger(), "Unable to open /sys/class/gpio/unexport");
        //    exit(1);
        //}

        sleep(1);
        int fUnxOut = write(fUnx, "21", 2);
        std::string strFUnxOut = std::to_string(fUnxOut);
        char* tmpFUnxOut = new char[strFUnxOut.length() + 1];
        strcpy(tmpFUnxOut, strFUnxOut.c_str());
        RCLCPP_ERROR(this->get_logger(), "------------- unexport written ---------------");
        RCLCPP_ERROR(this->get_logger(), tmpFUnxOut); 
        ////write(fd, "24", 2);
        //if (write(fd, "21", 2) != -1 ) {
        //    RCLCPP_ERROR(this->get_logger(), "Error writing to /sys/class/gpio/unexport");
        //    exit(1);
        //}

        close(fUnx);

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
