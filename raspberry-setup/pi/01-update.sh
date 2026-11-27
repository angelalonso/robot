#!/usr/bin/env bash

set -Eeuo pipefail

if [[ "${EUID}" -eq 0 ]]; then
  SUDO=()
else
  SUDO=(sudo)
fi

echo "Updating package metadata..."
"${SUDO[@]}" apt-get update

echo "Applying available system updates..."
"${SUDO[@]}" DEBIAN_FRONTEND=noninteractive apt-get \
  -o DPkg::Lock::Timeout=300 full-upgrade -y

echo "Installing baseline administration packages..."
"${SUDO[@]}" DEBIAN_FRONTEND=noninteractive apt-get \
  -o DPkg::Lock::Timeout=300 install -y \
  ca-certificates unattended-upgrades

echo "Enabling automatic security updates..."
"${SUDO[@]}" systemctl enable --now unattended-upgrades

echo "Update stage completed. Reboot the Pi before the security stage if the"
echo "package manager reported that a reboot is required."
