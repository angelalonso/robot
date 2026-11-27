#!/usr/bin/env bash

set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
CONFIG_FILE="${ROOT_DIR}/config"
STAGE="all"
DRY_RUN=0

if [[ -r "${CONFIG_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${CONFIG_FILE}"
else
  echo "Missing ${CONFIG_FILE}. Copy config.example to config and edit it." >&2
  exit 1
fi

: "${PI_HOST:?PI_HOST is required in ${CONFIG_FILE}}"
: "${PI_USER:?PI_USER is required in ${CONFIG_FILE}}"
: "${SSH_KEY:?SSH_KEY is required in ${CONFIG_FILE}}"

usage() {
  printf 'Usage: %s [--stage all|update|secure] [--dry-run]\n' "$0"
}

while (($#)); do
  case "$1" in
    --stage)
      [[ $# -ge 2 ]] || { usage >&2; exit 2; }
      STAGE="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

case "${STAGE}" in
  all|update|secure) ;;
  *) echo "Invalid stage: ${STAGE}" >&2; exit 2 ;;
esac

if ((DRY_RUN == 0)); then
  [[ -f "${SSH_KEY}" ]] || {
    echo "SSH private key not found: ${SSH_KEY}" >&2
    exit 1
  }
  [[ -f "${SSH_KEY}.pub" ]] || {
    echo "SSH public key not found: ${SSH_KEY}.pub" >&2
    exit 1
  }
fi

SSH=(ssh -i "${SSH_KEY}" -o BatchMode=yes -o ConnectTimeout=10)
SCP=(scp -i "${SSH_KEY}" -o BatchMode=yes)
REMOTE="${PI_USER}@${PI_HOST}"
REMOTE_DIR="/tmp/robot-setup-${USER}-$$"

run() {
  if ((DRY_RUN)); then
    printf '+'
    printf ' %q' "$@"
    printf '\n'
  else
    "$@"
  fi
}

if [[ "${STAGE}" == all || "${STAGE}" == update ]]; then
  UPDATE_SCRIPT="${ROOT_DIR}/pi/01-update.sh"
fi
if [[ "${STAGE}" == all || "${STAGE}" == secure ]]; then
  SECURE_SCRIPT="${ROOT_DIR}/pi/02-secure.sh"
fi

echo "Checking key-based SSH access to ${REMOTE}..."
run "${SSH[@]}" "${REMOTE}" true

run "${SSH[@]}" "${REMOTE}" "mkdir -m 700 '${REMOTE_DIR}'"
cleanup() {
  if ((DRY_RUN == 0)); then
    "${SSH[@]}" "${REMOTE}" "rm -rf -- '${REMOTE_DIR}'" || true
  fi
}
trap cleanup EXIT

if [[ -n "${UPDATE_SCRIPT-}" ]]; then
  run "${SCP[@]}" "${UPDATE_SCRIPT}" "${REMOTE}:${REMOTE_DIR}/01-update.sh"
  run "${SSH[@]}" -t "${REMOTE}" "bash '${REMOTE_DIR}/01-update.sh'"
fi

if [[ -n "${SECURE_SCRIPT-}" ]]; then
  run "${SCP[@]}" "${SECURE_SCRIPT}" "${REMOTE}:${REMOTE_DIR}/02-secure.sh"
  run "${SSH[@]}" -t "${REMOTE}" "bash '${REMOTE_DIR}/02-secure.sh'"
fi

echo "Raspberry Pi setup stage '${STAGE}' completed."
