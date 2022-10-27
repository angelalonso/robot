// Based on Jacob Beck's works

#include <chrono>
#include <rclcpp/rclcpp.hpp>

using namespace std::chrono_literals;

class BrainBoot : public rclcpp::Node {
 public:
  BrainBoot() : Node("brainboot"), count_(0) {
    timer_ = this->create_wall_timer(1s, std::bind(&BrainBoot::timer_callback_, this));
  }

 private:
  void timer_callback_() {
    RCLCPP_INFO(this->get_logger(), "Hello! %lu", count_);
    count_++;
  }

  rclcpp::TimerBase::SharedPtr timer_;
  size_t count_;
};

int main(int argc, char* argv[]) {
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<BrainBoot>());
  rclcpp::shutdown();
  return 0;
}

