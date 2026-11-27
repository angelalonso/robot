#!/usr/bin/env bash

set -Eeuo pipefail

ALLOW_SSH_FROM=""
while (($#)); do
  case "$1" in
    --allow-ssh-from)
      [[ $# -ge 2 ]] || { echo "--allow-ssh-from needs an address" >&2; exit 2; }
      ALLOW_SSH_FROM="$2"
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 [--allow-ssh-from ADDRESS]"
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [[ "${EUID}" -eq 0 ]]; then
  SUDO=()
else
  SUDO=(sudo)
fi

SSH_USER="${SUDO_USER:-${USER}}"
AUTHORIZED_KEYS="${HOME}/.ssh/authorized_keys"
if [[ -n "${SUDO_USER:-}" ]]; then
  USER_HOME="$(getent passwd "${SUDO_USER}" | cut -d: -f6)"
  AUTHORIZED_KEYS="${USER_HOME}/.ssh/authorized_keys"
fi

[[ "${SSH_USER}" != root ]] || {
  echo "Run this script as the normal SSH user, not root." >&2
  exit 1
}
[[ -s "${AUTHORIZED_KEYS}" ]] || {
  echo "No authorized SSH key found at ${AUTHORIZED_KEYS}; refusing to disable password SSH." >&2
  exit 1
}

if [[ -z "${ALLOW_SSH_FROM}" && -n "${SSH_CONNECTION:-}" ]]; then
  ALLOW_SSH_FROM="${SSH_CONNECTION%% *}"
fi
[[ -n "${ALLOW_SSH_FROM}" ]] || {
  echo "Could not determine the laptop address. Re-run with --allow-ssh-from ADDRESS." >&2
  exit 1
}

case "${ALLOW_SSH_FROM}" in
  *[!0-9a-fA-F:./]*)
    echo "SSH source must be an IPv4/IPv6 address or CIDR range: ${ALLOW_SSH_FROM}" >&2
    exit 1
    ;;
esac

echo "Installing firewall and SSH rate-limiter..."
"${SUDO[@]}" DEBIAN_FRONTEND=noninteractive apt-get update
"${SUDO[@]}" DEBIAN_FRONTEND=noninteractive apt-get \
  -o DPkg::Lock::Timeout=300 install -y ufw fail2ban

DROP_IN="/etc/ssh/sshd_config.d/robot-hardening.conf"
TEMP_DROP_IN="$(mktemp)"
trap 'rm -f -- "${TEMP_DROP_IN}"' EXIT
cat >"${TEMP_DROP_IN}" <<EOF
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin no
PubkeyAuthentication yes
X11Forwarding no
AllowUsers ${SSH_USER}
EOF

"${SUDO[@]}" install -o root -g root -m 0644 "${TEMP_DROP_IN}" "${DROP_IN}"
"${SUDO[@]}" sshd -t

"${SUDO[@]}" ufw default deny incoming
"${SUDO[@]}" ufw default allow outgoing
"${SUDO[@]}" ufw allow from "${ALLOW_SSH_FROM}" to any port 22 proto tcp
"${SUDO[@]}" ufw --force enable

"${SUDO[@]}" systemctl enable --now fail2ban
"${SUDO[@]}" systemctl reload ssh

echo "Security stage completed."
echo "SSH is allowed from: ${ALLOW_SSH_FROM}"
echo "Keep this session open and verify a second key-based SSH session before closing it."
"${SUDO[@]}" ufw status verbose
"${SUDO[@]}" fail2ban-client status sshd
