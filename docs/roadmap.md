# Roadmap to a working robot

The order below minimizes the chance of damaging hardware. Do not skip a gate
because a software test passed on x86_64: the repository's x86_64 GPIO layer is a
mock.

## Phase 0: resolve and record physical facts

**Goal:** replace assumptions with a one-page wiring record.

- Identify the exact Raspberry model/revision and confirm 40-pin header layout.
- Inventory every physical component and photograph labels, connectors, and the
  underside of the L298N.
- Measure battery pack voltage and polarity with no load.
- Identify motor output pairs and label left/right motor mechanically.
- Confirm whether the L298N has its 5 V regulator jumper installed and do not
  use it to power the Pi without an electrical design review.
- Verify the LED resistor and GPIO 21 wiring.
- Decide the right-motor IN1/IN2 order by continuity and a lifted-wheel test.
- Draw one final wiring diagram and retire or label superseded diagrams.

**Gate:** no unknown power path, ground path, motor identity, or GPIO direction.

## Phase 1: build a safe mechanical/electrical base

- Secure both motors, wheels, rear wheel, batteries, L298N, Pi, switch, and
  wiring to the chassis.
- Add a physical motor power disconnect; keep the Pi power path independent.
- Join grounds where required and insulate all exposed conductors.
- Test with motors disconnected: Pi boots, LED works, and every GPIO has the
  expected idle level.

**Gate:** Pi remains powered while motor power is disconnected; no GPIO exceeds
 3.3 V; no unexpected continuity exists between motor supply and Pi rails.

## Phase 2: make the Rust baseline deployable

- Create a documented armhf build command for `circuits`; verify dependency
  compatibility with Pi 1 B+.
- Replace `brain/` references in provisioning/service scripts with `circuits`.
- Define a runtime `.env` installation method without committing credentials.
- Install only the dependencies needed by the Rust baseline, including the
  correct GPIO/pigpio runtime.
- Add a real systemd service with restart policy, logs, shutdown stop, and
  non-root permissions where feasible.
- Make startup fail explicitly on invalid configuration instead of silently
  mocking missing serial/GPIO on the real robot.

**Gate:** service starts on boot, reports its version/config, and leaves motors
stopped after startup.

## Phase 3: validate GPIO and motors at low risk

- Run the existing GPIO cleanup procedure and verify it on the chosen OS.
- Test LED on/off.
- Test each motor with wheels lifted at minimum duty cycle.
- Resolve the right-motor direction conflict and update `.env` plus the final
  wiring diagram.
- Test stop after every direction command.
- Add a current/temperature observation sheet for the L298N and motors.

**Gate:** ten repeated command cycles produce the expected movement and stop
without brownout, uncontrolled motion, or excessive driver temperature.

## Phase 4: validate API and remote control

- Test each endpoint locally with `curl`.
- Replace empty/success-shaped responses with explicit command acknowledgement
  and failure responses.
- Add authentication or strict network restriction before exposing the API beyond
  a trusted isolated LAN.
- Test the Android app against the real API, including URL configuration, press,
  release, timeout, and stop behaviour.
- Test API loss and process termination: both motors must stop.

**Gate:** a person can command the robot, release the control, and recover from
  connection loss without the robot continuing to drive.

## Phase 5: battery and endurance

- Measure startup current, moving current, idle current, and battery voltage
  under load.
- Run a stationary ten-minute service test.
- Run a controlled low-speed floor test, then progressively longer runs.
- Record runtime, brownouts, motor-driver temperature, Pi temperature, and Wi-Fi
  stability.
- Add a low-battery procedure and safe shutdown policy.

**Gate:** documented runtime meets the intended use and the robot fails safe.

## Phase 6: Arduino and sensing

- Confirm the Arduino model and USB device path.
- Build and upload only `arduino/sensors/sensors.ino` first.
- Verify HC-SR04 5 V, trigger, echo, and ground wiring; protect any signal that
  could reach a Pi GPIO.
- Connect USB to the Pi and validate `/dev/ttyACM0` (or actual device) and 9600
  baud.
- Harden `get_msg` against malformed/incomplete frames and add sensor freshness,
  timeout, and out-of-range states.
- Decide whether VL53L0X replaces or complements HC-SR04; do not combine the
  experimental sketches without a wiring and protocol decision.

**Gate:** stale or missing sensor data causes a known safe state, not autonomy.

## Phase 7: autonomy and maintenance

- Add obstacle-stop behaviour with motors disconnected, then on a tethered test.
- Add bounded action sequences and cancellation.
- Add structured logs, health checks, and diagnostics.
- Add integration tests for UDP/API/state transitions and a hardware-in-the-loop
  test procedure.
- Update the top-level README and remove obsolete installation instructions.
- Keep a dated wiring photo and a release checklist with every hardware change.

## Information still needed from the hardware

The following cannot be safely inferred from this repository:

- exact board revisions and voltage regulator state;
- actual battery type/voltage/capacity and charger;
- motor ratings and stall current;
- physical L298N terminal/jumper arrangement;
- final left/right motor and wheel orientation;
- LED resistor value and wiring;
- whether the chassis can safely carry the power bank and battery pack;
- measured Wi-Fi range and battery runtime.

