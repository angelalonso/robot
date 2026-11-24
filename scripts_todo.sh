#!/usr/bin/env bash

# IMPLEMENTATION SPECIFICATION
# ============================
#
# This file is a task document, not the implementation. It describes the work
# required to make install_to_raspberry.sh and robot.sh safe, reproducible, and
# compatible with the current Rust circuits/ baseline.
#
# A future implementation agent must:
#   1. Read this entire document before editing.
#   2. Inspect the current files and preserve unrelated user changes.
#   3. Implement the requirements below rather than only silencing ShellCheck.
#   4. Run the validation commands at the end.
#   5. Report any hardware-dependent item that cannot be verified locally.
#
# Scope:
#   - install_to_raspberry.sh
#   - robot.sh
#   - files/env.template, when required by the script contract
#   - a new active deployment/service template, when required by the installer
#   - focused documentation updates required to keep commands accurate
#
# Do not:
#   - modify Rust application behavior unless a script integration requires it;
#   - revive files under docs/archive/ or files/archive/;
#   - reintroduce the removed brain/ tree;
#   - commit changes, push branches, reset the worktree, or discard unrelated
#     user changes;
#   - claim that physical GPIO, motor, battery, or Raspberry hardware behavior
#     was verified on a development machine.

# ---------------------------------------------------------------------------
# 1. CURRENT DEFECTS TO FIX
# ---------------------------------------------------------------------------
#
# install_to_raspberry.sh:
#
# - Line 21 hashes the literal string '${PASS}' because single quotes prevent
#   expansion. Use a safe password-input strategy and never write the plaintext
#   password to logs.
# - show_log "err" prints an error but does not return non-zero or stop the
#   script. Every fatal validation or provisioning error must stop execution.
# - Line 75 uses undefined NEWPORT; the configuration calls the setting
#   SSHPORT.
# - The script only writes userconf.txt and then prints a manual checklist. Its
#   name implies installation, but it does not install the Rust application,
#   runtime configuration, service, or required dependencies.
# - It relies on the caller's current directory and on an implicitly sourced
#   .env file.
# - It does not validate required variables, command dependencies, mount layout,
#   existing userconf.txt, or whether the target image is compatible.
# - It does not configure SSH enablement, Wi-Fi, SSH keys, hostname, firewall,
#   pigpio/runtime dependencies, repository checkout, build, or systemd.
#
# robot.sh:
#
# - CWDMAIN=$(pwd) breaks when the script is invoked outside the repository
#   root. Resolve the script directory instead.
# - load_dotenv executes arbitrary shell code and requires PASS even though the
#   runtime/development wrapper does not need the provisioning password.
# - SSHIP and GIT_BRANCH are used but are absent from files/env.template.
# - kill_switch discovers processes with ps|grep and sends kill -9. It can kill
#   unrelated processes and can leave GPIO/motors unsafe.
# - live_kill force-kills the Rust process instead of requesting a stop and
#   allowing cleanup.
# - trigger_push automatically commits all tracked changes before deployment.
#   Deployment must never silently commit or push work.
# - trigger_deploy hardcodes /home/robotadm/.cargo/bin/cargo instead of using
#   the configured remote user/environment.
# - live_run uses sudo to launch the robot directly instead of a service or
#   explicit, controlled foreground mode.
# - ping is used as the only availability test, has no timeout, and does not
#   prove that SSH is usable.
# - cargo failure checks after set -e are redundant and obscure the intended
#   error handling.
# - do_mode "$@" is unquoted and ShellCheck reports SC2068.

# ---------------------------------------------------------------------------
# 2. SHARED SCRIPT CONTRACT
# ---------------------------------------------------------------------------
#
# Both scripts must:
#
# - start with:
#       #!/usr/bin/env bash
#       set -Eeuo pipefail
# - use a SCRIPT_DIR based on BASH_SOURCE, not pwd:
#       SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
# - quote every path, variable expansion, SSH argument, and command argument;
# - use small functions with local variables and explicit return codes;
# - print errors to stderr and exit non-zero for fatal errors;
# - support a non-interactive invocation suitable for CI or another AI;
# - avoid passwords and private keys in output;
# - use `command -v` to validate required local commands;
# - use `trap` for cleanup and handle INT, TERM, and EXIT where appropriate;
# - never use `eval`, `killall`, `pkill`, broad recursive deletion, or
#   `ps | grep` process discovery;
# - pass ShellCheck without errors. Warnings must either be fixed or justified
#   in the implementation review.
#
# Configuration loading:
#
# - Define one documented configuration path, defaulting to:
#       REPO_DIR/.env
# - Do not require PASS for robot.sh, local tests, or an already-installed
#   robot. PASS belongs only to first-boot provisioning.
# - Prefer a safe KEY=VALUE parser that accepts the existing env.template
#   format without executing commands. If shell-compatible sourcing is retained,
#   document the trust boundary and validate the file permissions.
# - Validate required values explicitly and report the missing variable names.
# - Add only deployment variables that are actually consumed. At minimum,
#   document SSHIP and GIT_BRANCH if the remote workflow still uses them.

# ---------------------------------------------------------------------------
# 3. install_to_raspberry.sh REQUIRED DESIGN
# ---------------------------------------------------------------------------
#
# Decide and document two explicit modes:
#
#   bootstrap:
#     Prepare a mounted Raspberry Pi OS armhf image and create the initial user
#     configuration. This mode runs on the development machine.
#
#   deploy:
#     Connect to an already-booted Raspberry Pi over SSH, install dependencies,
#     copy/build/configure circuits, install the service, and verify readiness.
#
# If implementing only one mode, rename the script or clearly reject the
# unsupported mode. Do not leave a script that appears to install but only
# prints manual instructions.
#
# Bootstrap requirements:
#
# - Require an explicit --config path or default to SCRIPT_DIR/.env.
# - Validate MICROSD_PATH, NEWUSER, and PASS before touching the card.
# - Validate that the configured boot and rootfs mount directories exist.
# - Support the actual Raspberry Pi OS mount layout used by the selected image,
#   and fail with a clear message when only one partition is mounted.
# - Generate the password hash from the value of PASS, never the literal
#   variable name. Use `printf '%s' "$PASS"` rather than echo.
# - Write userconf.txt atomically with mode 0600 where possible.
# - Refuse to overwrite an existing userconf.txt unless an explicit
#   --force/--replace option is supplied.
# - Enable SSH using the image's supported mechanism, or document the manual
#   step if the selected image cannot be configured offline.
# - Do not copy Wi-Fi passwords or private keys into tracked files or logs.
# - Unmount only paths that this script mounted, unless an explicit option
#   requests cleanup. Never unmount unrelated user mounts.
# - Print exact next steps and the expected first SSH command.
#
# Deploy requirements:
#
# - Accept host, user, SSH port, repository path, and branch through validated
#   configuration or explicit command-line options.
# - Use SSH options with connection timeout and batch behavior, for example
#   ConnectTimeout and BatchMode where appropriate.
# - Check SSH connectivity, not only ICMP ping.
# - Install only the dependencies required by circuits:
#     git, cargo/rust toolchain strategy, libudev development/runtime support,
#     pigpio requirements, and any service prerequisites.
# - Decide whether rust-pigpio requires pigpiod for this codebase and document
#   the result. Do not blindly disable pigpiod.
# - Clone or update the repository idempotently without deleting a working tree.
# - Refuse to deploy over uncommitted remote changes unless an explicit force
#   option is supplied.
# - Install the runtime `.env` with restrictive permissions and never commit it.
# - Build the exact `circuits` package using the configured remote user and
#   `cargo build --release --manifest-path ...`.
# - Install an active systemd unit for circuits. The unit must:
#     * run as the intended non-root user where GPIO permissions allow;
#     * use an explicit WorkingDirectory and EnvironmentFile;
#     * start after network readiness;
#     * restart on failure with a bounded policy;
#     * stop motors/release GPIO on controlled shutdown;
#     * expose logs through journalctl;
#     * not use the archived brain paths.
# - Enable/start the service only after the binary, configuration, and unit
#   have been validated.
# - Verify service state and a harmless health/status endpoint if available.
# - Never perform a movement command as part of installation.

# ---------------------------------------------------------------------------
# 4. robot.sh REQUIRED DESIGN
# ---------------------------------------------------------------------------
#
# Preserve useful modes, but make their contracts explicit:
#
#   help
#   test
#   test_arduino
#   deploy
#   run
#   live_run
#   push
#   kill
#   live_kill
#
# Required behavior:
#
# - Resolve REPO_DIR from the script location.
# - Use `case` for command dispatch and reject unknown or extra arguments.
# - Quote "$@" and all variable expansions.
# - `help` must work without .env.
# - `test` must work without PASS, SSHIP, or a reachable robot.
# - `deploy` must never call git commit automatically.
# - Replace automatic commit/push with one of:
#     * deploy the current commit only and require a clean worktree; or
#     * offer an explicit `push` command that reports what it will push; or
#     * require an explicit `--commit` option with a user-supplied message.
#   The default must be non-destructive.
# - Before deployment, verify the local branch/commit and show the target
#   branch/host. Do not deploy a dirty tree silently.
# - Use SSH connectivity checks with bounded retries and timeouts. Ping may be
#   an optional diagnostic, never the only readiness test.
# - Use the configured remote user and `command -v cargo`/a known Cargo path on
#   the remote host. Do not hardcode /home/robotadm.
# - Make remote commands use `set -Eeuo pipefail` and quote paths.
# - Prefer systemd operations:
#       systemctl --user ...   or   sudo systemctl ...
#   depending on the selected service ownership. The choice must be documented.
# - Keep `live_run` only as a deliberate foreground/debug mode.
# - `live_kill` must:
#     1. request a safe motor stop if an API/service endpoint is available;
#     2. send TERM to the exact process owned by this application/service;
#     3. wait for exit;
#     4. escalate to KILL only after a timeout;
#     5. run GPIO cleanup;
#     6. return failure if cleanup cannot be confirmed.
# - Do not identify processes using `ps | grep`. Use a PID file, systemd unit,
#   `pgrep -x` with ownership checks, or an exact command/pid mechanism.
# - Do not assume killing the process makes motors safe. The implementation
#   must explicitly account for motor stop and GPIO state.
# - Install EXIT/INT/TERM traps without recursively invoking cleanup.
# - `cargo check`, `cargo clippy`, and `cargo test` should fail immediately
#   with a clear command-specific message. Avoid checking `$?` after set -e;
#   use `if ! command; then ... fi` when custom handling is needed.
# - Keep local test, remote deployment, and hardware execution paths separate.

# ---------------------------------------------------------------------------
# 5. CONFIGURATION AND DOCUMENTATION
# ---------------------------------------------------------------------------
#
# Update files/env.template only after confirming each variable's consumer.
# The active template should document, as applicable:
#
#   ROBOT_HOST / SSHIP       remote hostname or address
#   NEWUSER                   remote deployment user
#   SSHPORT                   remote SSH port
#   GIT_BRANCH                deployment branch, if branch deployment remains
#   ROBOT_REMOTE_DIR          remote checkout directory
#   APIPORT                   API port
#   MICROSD_PATH              bootstrap-only mounted image path
#   NEWUSER/PASS              bootstrap-only initial credentials
#
# Do not keep a real password in the template. Use an obvious placeholder and
# make the installer require the user to replace it or accept PASS through a
# protected prompt/environment.
#
# Update active documentation to state:
#
# - which Raspberry Pi OS image/architecture is supported;
# - how to run bootstrap and deploy;
# - how to install/copy the runtime `.env`;
# - how to inspect service logs and status;
# - how to stop the robot safely;
# - that x86_64 GPIO tests use a mock and do not validate hardware;
# - that the first deployment must use disconnected motors or lifted wheels.
#
# Do not update archived documents except to prevent an active link from
# pointing to a moved file.

# ---------------------------------------------------------------------------
# 6. TEST PLAN
# ---------------------------------------------------------------------------
#
# Run from the repository root:
#
#   bash -n install_to_raspberry.sh robot.sh circuits/reset_gpio.sh
#   Run ShellCheck on install_to_raspberry.sh, robot.sh, and
#   circuits/reset_gpio.sh.
#   git diff --check
#   ./robot.sh help
#   ./robot.sh unknown-command
#
# Expected:
#   - syntax succeeds;
#   - ShellCheck has no errors;
#   - help succeeds without a .env;
#   - unknown commands fail non-zero and print usage;
#   - no command creates commits, pushes, kills processes, or contacts a robot
#     during the help/unknown-command checks.
#
# Configuration tests:
#
#   - Run with a missing configuration file and verify a concise non-zero error.
#   - Run with each required variable missing and verify its name is reported.
#   - Use a password containing spaces and shell metacharacters in a temporary
#     protected configuration; verify the generated password hash is not the
#     literal `${PASS}` and the plaintext is never printed.
#   - Verify paths containing spaces are handled or explicitly rejected.
#   - Verify an existing userconf.txt is preserved unless --force is used.
#
# robot.sh safety tests:
#
#   - Start a harmless process whose command contains the word `circuits` and
#     verify live_kill does not terminate it.
#   - Verify live_kill targets only the configured service/application process.
#   - Verify TERM is attempted before KILL and GPIO cleanup is attempted.
#   - Run deploy with a dirty local worktree and verify it refuses without an
#     explicit override.
#   - Verify deploy does not create a commit or push by default.
#   - Verify SSH failures stop before any remote build/start command.
#
# Deployment tests on a disposable Raspberry Pi image/device:
#
#   - Bootstrap creates the intended user and enables the documented access
#     path.
#   - Deploy is idempotent: running it twice does not duplicate users, units,
#     configuration, or repository state.
#   - The service starts with motors stopped.
#   - Service restart, stop, and SIGTERM leave GPIO/motors safe.
#   - `systemctl status`, `journalctl`, and the documented status endpoint work.
#   - No password, private key, or `.env` content appears in logs or Git.
#
# Hardware validation must be performed separately according to
# docs/104_Phase_0.md and docs/102_Roadmap.md. A development-machine test is
# not evidence that GPIO, motor direction, power, or battery behavior works.

# ---------------------------------------------------------------------------
# 7. REVIEWER ACCEPTANCE CHECKLIST
# ---------------------------------------------------------------------------
#
# The implementation is complete only if all are true:
#
# [ ] install_to_raspberry.sh has a clear, tested purpose and mode contract.
# [ ] The literal-password hashing bug is fixed and covered by a test.
# [ ] Fatal errors stop execution with non-zero status.
# [ ] Paths, arguments, and configuration are safely quoted/validated.
# [ ] No script executes an untrusted `.env` without an explicit trust decision.
# [ ] No script automatically commits or pushes user work.
# [ ] No broad process grep or unconditional kill -9 remains.
# [ ] Stop-before-kill and GPIO cleanup behavior is explicit.
# [ ] Hardcoded robotadm and brain references are absent from active scripts.
# [ ] SSH readiness is tested with bounded timeouts.
# [ ] The remote user, branch, checkout path, and service ownership are
#     configurable and documented.
# [ ] Active `.env` variables match the scripts' actual requirements.
# [ ] ShellCheck reports no errors.
# [ ] The focused tests above pass.
# [ ] The final report lists any hardware-dependent verification still pending.
