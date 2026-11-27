# Testing, setup, and calibration

This document defines how the robot is configured, tested, and calibrated.
Simulation results can validate software and geometry, but they do not prove
electrical safety, motor current, battery runtime, or physical GPIO behavior.

Record measured values, configuration changes, and failures in the relevant
phase document. Do not treat an old diagram, software default, or simulated
value as a physical measurement.

## 0. Reporting task completion

When a task is finished, add a short result to the active document named by
the task. Use `docs/phase_0.md` for current hardware and Raspberry results,
`docs/phase_0-done.md` for confirmed inventory and decisions, and this
document for repeatable test/calibration results. Do not create a success
record until every acceptance condition passes.

Use this format:

```text
### Task/result: <task name or test ID>
- Date:
- Hardware/software identity:
- Setup:
- Actions performed:
- Measurements/observations:
- Result: PASS | BLOCKED | INCONCLUSIVE
- Evidence: photo, log, command output, or file path
- Follow-up:
```

Guidelines:

- Record exact model names, board labels, software versions, GPIO numbers,
  measured voltages/currents, temperatures, durations, and test conditions.
- Separate `observed` facts from `assumed` or `not tested` details.
- For a physical task, add dated photos or a wiring sketch when they prove
  identity, polarity, connector orientation, or routing.
- For a software task, record the script/stage used, commit or version, and
  relevant pass/fail output; do not paste secrets or private keys.
- Mark the result `BLOCKED` when an unknown, unsafe condition, missing part, or
  failed check prevents the next step. State exactly what must be resolved.
- Mark the result `INCONCLUSIVE` when the test ran but the evidence is not
  sufficient to make a decision. Do not treat it as a pass.
- Update the relevant checklist only after writing the result. If a result
  changes the wiring, configuration, or project decision, update all affected
  active documents.

## 1. Reproducible setup

### Raspberry Pi

1. Prepare Raspberry Pi OS headlessly using `raspberry-setup/`.
2. Record the OS release, architecture, kernel, hostname, storage, and network
   identity in `docs/phase_0.md`.
3. Run the update and security stages before installing robot software.
4. Install only the dependencies required by the Rust controller.
5. Store runtime configuration in an untracked `.env` file. Never commit
   passwords, private keys, or Wi-Fi credentials.
6. Deploy the controller through a documented script or systemd unit, not by
   relying on an interactive shell session.

### Development laptop

1. Run formatting, compilation, linting, and unit tests before deployment.
2. Use the GPIO mock only for software behavior; it is not evidence that Pi
   GPIO wiring works.
3. Keep simulator configuration separate from physical robot configuration.

### Arduino

When sensing is introduced, record the board model, sketch version, USB device
path, baud rate, and serial message format. Upload and test the sketch from the
laptop before connecting the Arduino to the Pi.

## 2. Simulation tests

The first simulator should be a small 2D differential-drive model. Its
configuration must include, at minimum:

- body length and width;
- wheel radius and wheel separation;
- maximum wheel speed;
- control update interval;
- sensor positions and ranges, when sensors are simulated.

### Simulation calibration

1. Set dimensions from the measured chassis and wheel values.
2. Place the simulated robot at a known position and heading.
3. Verify that a centered joystick produces zero wheel velocity.
4. Verify forward input produces equal wheel targets.
5. Verify left/right input produces the expected differential speeds.
6. Verify maximum joystick input never exceeds configured motor limits.
7. Verify release, center, timeout, and controller disconnect stop the robot.
8. Compare a measured travel distance and turn with the simulation after the
   first physical tests; update parameters only with recorded evidence.

Simulation tests should cover API behavior, joystick mapping, motor mixing,
collision handling, obstacle-stop behavior, and recovery after stale input.

## 3. Raspberry and GPIO tests

Perform these tests with motor power disconnected first:

1. Confirm the controller starts with both motors stopped.
2. Confirm every configured GPIO has the expected idle level.
3. Test the status LED and its resistor.
4. Verify GPIO-to-L298N continuity one wire at a time.
5. Verify that motor supply voltage cannot reach a Pi GPIO or encoder signal.
6. Verify the common-ground plan with power disconnected.
7. Confirm shutdown and process termination invoke GPIO cleanup.

Record GPIO number, physical header pin, measured state, test equipment, and
result. Do not use the breadboard for motor current or unfused battery power.

## 4. L298N and motor calibration

Only begin after the L298N labels, jumper state, battery polarity, and power
path are recorded.

### L298N checks

- Verify motor-supply polarity and voltage with the motor disconnected.
- Confirm the `5V-EN`/regulator arrangement and logic-power implications.
- Check driver temperature during short, current-limited tests.
- Confirm the physical motor-output mapping.

### Motor checks

For each motor, record:

- left/right physical identity;
- L298N output pair;
- direction for each command;
- no-load current;
- startup current;
- loaded voltage and voltage sag;
- minimum reliable movement command;
- driver and motor temperature.

Begin with wheels lifted and short pulses. Calibrate direction and motor
balance only after both motors can stop reliably. Any correction for different
motor speeds must be recorded in the runtime configuration.

## 5. Joystick and API tests

The Android app uses an on-screen pointer/joystick. Test the complete path:

```text
Android pointer -> movement vector -> Pi API -> motor mixing -> L298N
```

Verify:

- center produces zero output;
- forward/backward produces the correct direction;
- left/right changes the differential wheel speeds;
- diagonal input remains bounded;
- the dead zone prevents jitter;
- release immediately requests stop;
- stale commands expire;
- API/network loss stops both motors;
- invalid commands produce explicit errors;
- the API is reachable only through the intended security boundary.

Run these tests in the simulator first, then with wheels lifted, then during a
controlled low-speed floor test.

## 6. Battery and endurance tests

Use the final battery only after its chemistry, voltage, capacity, fuse, switch,
and charger are identified.

Record:

- battery voltage before and after each test;
- startup, moving, and idle current;
- voltage sag;
- runtime;
- Pi and L298N temperature;
- Wi-Fi stability;
- brownouts, resets, or loss of control.

Required progression:

1. Pi alone.
2. L298N powered with motors disconnected.
3. One lifted motor.
4. Two lifted motors.
5. Short, low-speed floor test.
6. Longer endurance test with an accessible physical motor disconnect.

## 7. Arduino and sensor calibration

When the drive system is stable:

1. Test the Arduino and sensor independently from the motor system.
2. Use the breadboard only for labelled low-current sensor and logic wiring.
3. Verify sensor supply voltage, ground, trigger, echo, and signal levels.
4. Measure known distances and record sensor offset and useful range.
5. Validate serial framing, malformed messages, timeouts, freshness, and
   out-of-range values.
6. Confirm stale or missing sensor data causes a safe state.

Encoder calibration, if used, must additionally record pulses per motor
revolution, gearbox/output revolution, direction phase, supply voltage, and
whether level shifting is required.

## Completion rule

A test is complete only when its setup, inputs, measured result, expected
result, and pass/fail decision are recorded. A failed or inconclusive test
blocks dependent hardware or autonomy work until the cause is understood.
