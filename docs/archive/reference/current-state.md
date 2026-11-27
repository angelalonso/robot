# Current state

This document describes what is present in the repository and what the documents
claim exists. It is not proof that the physical parts are currently assembled or
working; those points are marked **unverified**.

## Hardware

### Documented or expected parts

| Area | Current documented choice | Evidence | Confidence |
|---|---|---|---|
| Controller | Raspberry Pi 3 Model B v1.2 with onboard Wi-Fi | `docs/phase_0.md`, `docs/hardware-requirements.md` | Reported, physically unverified |
| Motor driver | L298N dual H-bridge | `docs/hardware-requirements.md`, `docs/hardware_l298n.md` | Reported, physical board unverified |
| Drive | Two DC encoder gearmotors and two wheels | `docs/hardware-requirements.md`, `docs/hardware_motor_encoder.md` | Reported, physically unverified |
| Chassis | Two A4 plywood layers, a cut plastic lunchbox, and a rear dummy/castor wheel | `docs/hardware_chassis.md` | Documented, physically unverified |
| Motor power | Existing six-AA pack for controlled temporary tests only; final supply not selected | `docs/phase_0.md` | Decision recorded, physically unverified |
| Raspberry power | Anker PowerCore 20100 or equivalent USB power bank | `docs/hardware_chassis.md` | Documented, physically unverified |
| Wiring | Breadboard, jumper wires, switch, fuse if available, nuts and bolts | `docs/hardware-requirements.md` | Reported, physically unverified |
| Status output | One LED on Raspberry GPIO 21, nominally with a resistor | `files/env.template`, `docs/hardware_gpio.md` | Software/documented, physical wiring unverified |
| Sensors | Arduino UNO, HC-SR04, and a reported VL53L0X time-of-flight breakout marked `CJUL53LOXV2` | `docs/archive/later/hardware_new_input.md`, `arduino/`, `docs/phase_0-done.md` | Later milestone, physically unverified |

### Current Raspberry/L298N pin configuration

The current `.env` template configures:

| Function | BCM GPIO | Physical pin |
|---|---:|---:|
| Left motor IN1 | 27 | 13 |
| Left motor IN2 | 17 | 11 |
| Left motor enable/PWM | 22 | 15 |
| Right motor IN1 | 24 | 18 |
| Right motor IN2 | 23 | 16 |
| Right motor enable/PWM | 25 | 22 |
| Main LED | 21 | 40 |

The chassis document labels the right motor inputs in the opposite order
(IN1=GPIO23 and IN2=GPIO24). Do not apply power until this is checked with a
multimeter and a lifted-wheel direction test.

### Power and safety gaps

The repository does not establish:

- the actual Raspberry model and revision;
- the exact motor current, stall current, or L298N heat-sinking;
- the battery chemistry, cell condition, polarity, or measured voltage under load;
- whether the Raspberry ground, L298N ground, and future Arduino ground are joined;
- a fuse, battery undervoltage cutoff, emergency stop, or motor kill switch suitable
  for unattended operation;
- the actual motor-to-output mapping (Motor A/B versus left/right);
- whether GPIO 21 is connected to an LED with a safe current-limiting resistor.

These are physical acceptance checks, not software assumptions.

## Software

### Current implemented baseline

The executable implementation is the Rust crate in `circuits/`:

- `circuits/src/main.rs` starts six long-lived components:
  API, Arduino, left motor, right motor, LED, and status.
- `circuits/src/api_node.rs` exposes an unauthenticated HTTP API on `APIPORT`
  (template value `5000`) and binds to all interfaces.
- The API currently supports discrete forward, backward, left, right, and stop
  commands, LED on/off/switch, scan (currently empty), mode (currently only
  sends an LED command), and status. The intended control design is changing
  to a continuous on-screen joystick/pointer; that API and motor mixing are
  not implemented yet.
- Nodes communicate over localhost UDP ports defined in `circuits/src/lib.rs`.
- `gpio_robot.rs` uses the legacy `/sys/class/gpio` interface and `rust-pigpio`
  for PWM on ARM; x86_64 uses a mock implementation.
- Motor direction is binary forward/backward/stop. The configured PWM value is
  initialized, but no API currently sets a variable speed.
- `arduino_node.rs` optionally reads `/dev/ttyACM0` at 9600 baud and parses
  messages shaped like `SENSOR: key=value|`.
- `status_node.rs` stores string key/value state in memory and returns regex-selected
  values.
- `robot.sh` can run local Cargo checks/tests and can SSH to a robot, pull, build,
  and run the binary manually.

### Other software present

- `arduino/` contains sketches for an HC-SR04 and experimental VL53L0X use.
- `AndroidApp/` contains a basic Kotlin controller with directional press-and-hold
  controls and a configurable HTTP base URL. It has no authentication or TLS.
  The target UI is a virtual joystick/pointer that sends a continuous movement
  vector and stops when centered, released, or disconnected.
- `tests/worlds/robot_testing_unit.wbt` is a small Webots two-wheel model, but no
  controller is present in this repository.
- `files/` contains the current runtime templates plus a legacy cloud-init/autosetup
  approach under `files/archive/maybe-obsolete/`; `install_to_raspberry.sh`
  targets Raspberry Pi OS/armhf.

### Known implementation limitations

- No repository `.env` is tracked; the program requires one at build/runtime
  because `env!` is used for configuration values.
- `files/archive/maybe-obsolete/autosetup.sh`, `files/archive/maybe-obsolete/robot.service.template`, and `docs/archive/maybe-obsolete/NEW_Raspberry.md`
  still refer to the removed `brain/` tree and do not install/build `circuits`.
- `files/archive/maybe-obsolete/autosetup.sh` installs ROS2, Python packages, Rust,
  and `colcon`, although
  the selected baseline is Rust and does not require ROS2 for the first milestone.
- Serial failure is logged as a mock/no-data condition rather than a hard fault.
- API requests return success-shaped empty responses even when the physical action
  cannot be confirmed.
- UDP parsing assumes a `message|sender` format and can panic on malformed input.
- The API has permissive CORS and no authentication, authorization, rate limiting,
  or network isolation.
- The current integration test file contains no tests.
- No evidence in the repository proves a complete Raspberry deployment, battery
  run, motor movement, or Android-to-robot run has passed.

