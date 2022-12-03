FROM ubuntu:jammy

ENV TERM linux
ENV DEBIAN_FRONTEND noninteractive
ARG NEWUSER

# Add user
RUN useradd -m ${NEWUSER} \
&& echo "${NEWUSER}:${NEWUSER}" | chpasswd \
&& usermod -aG sudo ${NEWUSER} \
&& mkdir -p /home/${NEWUSER}/robot \
&& chown ${NEWUSER}. -R /home/${NEWUSER}

# Install aarch64 gcc and build tools and some other stuff
RUN apt-get update -y \
&& apt-get install -y \
bc \
bison \
build-essential \
cpio \
curl \
device-tree-compiler \
fakeroot \
flex \
gcc-aarch64-linux-gnu \
git \
gnupg \
kmod \
lsb-release \
net-tools \
python3 \
python3-dev \
python3-pip \
python3-venv \
sudo \
swig \
&& rm -rf /var/lib/apt/lists/*

RUN curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key -o /usr/share/keyrings/ros-archive-keyring.gpg \
&& sh -c 'echo "deb [arch=amd64,arm64] http://repo.ros2.org/ubuntu/main `lsb_release -cs` main" > /etc/apt/sources.list.d/ros2-latest.list' \
&& curl -s https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | sudo apt-key add - \
&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu jammy main" | tee /etc/apt/sources.list.d/ros2.list > /dev/null \
&& apt-get update -y \
&& apt-get install -y ros-rolling-ros-base python3-colcon-common-extensions

# Prepare workspace
USER ${NEWUSER}
ENV PATH "$PATH:/home/robotadm/.local/bin"
WORKDIR /home/${NEWUSER}
RUN python3 -m pip install --upgrade pip \
&& pip3 install \
apiflask \
flask \
flatdict \
maturin \
python-dotenv \
ros_cross_compile \
RPi.GPIO

WORKDIR "/home/${NEWUSER}"
#ENTRYPOINT ["/bin/bash", "/script/script.sh"]
