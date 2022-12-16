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

RUN apt-get update && apt-get install -y curl \
 && apt-get install build-essential sudo -y \
 && rm -rf /var/lib/apt/lists/*
 
RUN  sudo -i -u $NEWUSER bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
RUN curl -s https://packagecloud.io/install/repositories/dirk-thomas/colcon/script.deb.sh | sudo bash

# Prepare workspace
USER ${NEWUSER}
ENV PATH="/home/${NEWUSER}/.cargo/bin:${PATH}"

WORKDIR "/home/${NEWUSER}"
#ENTRYPOINT ["/bin/bash", "/script/script.sh"]
