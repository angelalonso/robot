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
- Connect it to your Wifi
sudo apt install wireless-tools wpasupplicant
iwconfig # Check your ESSID
wpa_passphrase your-ESSID your-wifi-passphrase | sudo tee /etc/wpa_supplicant.conf # Check and correct contents of the file
sudo wpa_supplicant -c /etc/wpa_supplicant.conf -i 
sudo dhclient wlan0
sudo cp /lib/systemd/system/wpa_supplicant.service /etc/systemd/system/wpa_supplicant.service
sudo vim /etc/systemd/system/wpa_supplicant.service
- Change
  - ExecStart=/sbin/wpa_supplicant -u -s -O /run/wpa_supplicant
- to:
  - ExecStart=/sbin/wpa_supplicant -u -s -c /etc/wpa_supplicant.conf -i wlan0
  - Restart=always
- Comment out this line:
  - Alias=dbus-fi.w1.wpa_supplicant1.service
sudo systemctl daemon-reload
sudo systemctl enable wpa_supplicant.service
sudo vim /etc/systemd/system/dhclient.service
- Add:
```[Unit]
Description= DHCP Client
Before=network.target
After=wpa_supplicant.service

[Service]
Type=forking
ExecStart=/sbin/dhclient wlan0 -v
ExecStop=/sbin/dhclient wlan0 -r
Restart=always
 
[Install]
WantedBy=multi-user.target
```
sudo vim /etc/default/crda
```
REGDOMAIN=DE
```
sudo init 6

## Install ROS2 on Ubuntu
sudo apt update && sudo apt install curl gnupg2 lsb-release build-essential
curl -s https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | sudo apt-key add -
sudo sh -c 'echo "deb [arch=$(dpkg --print-architecture)] http://packages.ros.org/ros2/ubuntu $(lsb_release -cs) main" > /etc/apt/sources.list.d/ros2-latest.list'
sudo apt update
sudo apt install ros-foxy-ros-base python3-colcon-common-extensions python3-argcomplete python3-rosdep2 unzip

# Creating new project
source /opt/ros/rolling/setup.zsh
ros2 pkg create --build-type ament_python py_pubsub
-- Add code
rosdep install -i --from-path . --rosdistro rolling -y
colcon build --packages-select brain
. install/setup.zsh

# Raspberry GPIO
wget https://github.com/joan2937/pigpio/archive/master.zip
unzip master.zip
cd pigpio-master
make
sudo make install
