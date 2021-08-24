# Testing your Raspberry's GPIOs

Before getting deep into how ROS2 you'll want to test manually the components you use. This doc is about exactly that.

## Turning a GPIO on and off
https://raspberry-projects.com/pi/command-line/io-pins-command-line/io-pin-control-from-the-command-line

Schematics (proper drawing TBD)

```
                           .___.              
                  +3V3---1-|O O|--2--+5V
          (SDA)  GPIO2---3-|O O|--4--+5V
         (SCL1)  GPIO3---5-|O O|--6--GND--------------------------------------
    (GPIO_GLCK)  GPIO4---7-|O O|--8-----GPIO14 (TXD0)                        |
                    GND--9-|O.O|-10-----GPIO15 (RXD0)                        |
    (GPIO_GEN0) GPIO17--11-|O O|-12-----GPIO18 (GPIO_GEN1)                   |
    (GPIO_GEN2) GPIO27--13-|O O|-14--GND                                     |
    (GPIO_GEN3) GPIO22--15-|O O|-16-----GPIO23 (GPIO_GEN4)                   |
                  +3V3--17-|O O|-18-----GPIO24 (GPIO_GEN5)                   |
     (SPI_MOSI) GPIO10--19-|O.O|-20--GND                                     |
     (SPI_MOSO) GPIO9 --21-|O O|-22-----GPIO25 (GPIO_GEN6)                   |
     (SPI_SCLK) GPIO11--23-|O O|-24-----GPIO8  (SPI_C0_N)                    |
                    GND-25-|O O|-26-----GPIO7  (SPI_C1_N)                    |
       (EEPROM) ID_SD---27-|O O|-28-----ID_SC Reserved for ID EEPROM         |
                GPIO5---29-|O.O|-30--GND                                      >
                GPIO6---31-|O O|-32-----GPIO12                               <
                GPIO13--33-|O O|-34--GND                                      > 330Ohm
                GPIO19--35-|O O|-36-----GPIO16                               <
                GPIO26--37-|O O|-38-----GPIO20                                |
                      _-39-|O O|-40-----GPIO21----------------------------|>|--
                           '---'                                          LED
(_ means Ground)
```
Commands
```
echo "21" > /sys/class/gpio/export
echo "out" > /sys/class/gpio/gpio21/direction
echo "1" > /sys/class/gpio/gpio21/value
echo "0" > /sys/class/gpio/gpio21/value
```

## Turning a GPIO on and off with PIGPIO
Install PIGPIO
```
cd ros/brain
mkdir src && cd src
wget https://github.com/joan2937/pigpio/archive/master.zip
unzip master.zip
cd pigpio-master
make
sudo make install
cd ..
rm master.zip
sudo rm -r pigpio-master
```
Build https://github.com/botamochi6277/ros2_pigpio
```
cd robot/ros/brain/src/
git clone https://github.com/botamochi6277/ros2_pigpio
. /opt/ros/foxy/setup.bash
MAKEFLAGS="-j1 -l1" colcon build
. install/setup.bash
sudo pigpiod
ros2 run ros2_pigpio gpio_writer --ros-args --param pin:=21
```
On a second terminal:
```
cd robot/ros/brain/src/
. install/setup.bash
ros2 topic pub --once gpio_output_21 std_msgs/msg/Bool '{data: false}'
```

## Turning a GPIO on and off with ROS2
TBD

[PREV: Preparing the Raspberry <--](002_Raspberry.md) [--> NEXT: Build a Chassis](004_Chassis.md)
