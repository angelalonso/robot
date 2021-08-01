# On the laptop

- Install ROS on Ubuntu
sudo sh -c 'echo "deb http://packages.ros.org/ros/ubuntu $(lsb_release -sc) main" > /etc/apt/sources.list.d/ros-latest.list'
curl -s https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | sudo apt-key add -
sudo apt update
sudo apt-get install ros-noetic-desktop-full python3-rosdep2 # rosdep was missing
sudo rm /etc/ros/rosdep/sources.list.d/20-default.list
sudo rosdep init
rosdep update
source /opt/ros/noetic/setup.zsh # because I use...zsh
- Navigate to the folder for your robot files and prepare it
mkdir src
catkin_make
source devel/setup.zsh

# On the Raspberry
- https://roboticsbackend.com/install-ubuntu-on-raspberry-pi-without-monitor/
- Get image from here https://ubuntu.com/download/raspberry-pi
- Install it to the Raspberry MicroSD
- Unmount Partitions. Put the MicroSD on the Raspberry and connect it with a cable to the router. Boot it
- Find it
nmap -sP 192.168.1.0/24 
ssh -o PreferredAuthentications=password -o PubkeyAuthentication=no ubuntu@192.168.xx.yyy
- Change the password
--- Edit /etc/locale.gen and uncomment the line with en_US.UTF-8
--locale-gen en_US.UTF-8
--update-locale en_US.UTF-8
- Install ROS on Ubuntu
  - https://varhowto.com/install-ros-noetic-raspberry-pi-4/
sudo sh -c 'echo "deb http://packages.ros.org/ros/ubuntu focal main" > /etc/apt/sources.list.d/ros-noetic.list'
sudo apt-key adv --keyserver 'hkp://keyserver.ubuntu.com:80' --recv-key C1CF6E31E6BADE8868B172B4F42ED6FBAB17C654
sudo apt update
sudo apt-get install -y python3-rosdep python3-rosinstall-generator python3-wstool python3-rosinstall build-essential cmake
sudo rosdep init
rosdep update
mkdir ~/ros_catkin_ws
cd ~/ros_catkin_ws
rosinstall_generator ros_comm --rosdistro noetic --deps --wet-only --tar > noetic-ros_comm-wet.rosinstall
wstool init src noetic-ros_comm-wet.rosinstall
- If it fails, rm src/.rosinstall and re-run it
rosdep install -y --from-paths src --ignore-src --rosdistro noetic -r --os=ubuntu:focal
sudo src/catkin/bin/catkin_make_isolated --install -DCMAKE_BUILD_TYPE=Release --install-space /opt/ros/noetic -j1 -DPYTHON_EXECUTABLE=/usr/bin/python3
source /opt/ros/noetic/setup.bash
roscd
sudo apt install python3-roslaunch
- test everything is good
roscore
