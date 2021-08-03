https://docs.ros.org/en/foxy/Tutorials/Workspace/Creating-A-Workspace.html#new-directory
source /opt/ros/foxy/setup.bash
mkdir -p ~/dev_ws/src

https://docs.ros.org/en/foxy/Tutorials/Creating-Your-First-ROS2-Package.html
ros2 pkg create --build-type ament_cmake --node-name my_node example1
cd ~/dev_ws
colcon build --packages-select example1
. install/setup.bash
ros2 run example1 my_node

https://docs.ros.org/en/foxy/Tutorials/Writing-A-Simple-Py-Publisher-And-Subscriber.html
