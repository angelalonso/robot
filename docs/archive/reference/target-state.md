# Target working robot

The target is split into a minimum safe working robot and later capabilities.
The first target deliberately excludes the Arduino so that drive, power, and
remote control can be proven independently.

## Minimum working robot: hardware

1. A verified Raspberry Pi 3 Model B v1.2 running
   32-bit armhf Raspberry Pi OS.
2. A bootable microSD card and a reliable power bank capable of powering the Pi
   during Wi-Fi and motor operation.
3. A mechanically secure two-wheel differential-drive chassis with a free rear
   wheel, no exposed sharp edges, and room for wiring.
4. Two motors connected to the L298N outputs, with the physical left/right identity
   recorded.
5. A verified final motor supply connected to the L298N motor supply through a
   switch and fuse. The temporary six-AA pack is permitted only for controlled
   bring-up tests; the motor supply must not feed Pi GPIO.
6. A verified common ground between Pi and L298N logic ground.
7. Six verified Pi-to-L298N control wires using the selected GPIO table, including
   two enable/PWM wires.
8. A visible status LED with a suitable resistor on GPIO 21, or a documented
   replacement pin.
9. Strain relief, insulation, cable routing away from wheels, and a physical
   motor power disconnect.

The breadboard is an optional temporary prototyping aid for low-current logic,
sensor signals, and common-ground distribution. It is not part of the motor
power path and is not considered a permanent mobile-robot connection.

Before any powered test, the builder must record measured supply voltages,
polarity, motor current behaviour, and continuity for every control and ground
wire.

## Minimum working robot: software

### Raspberry service

- A reproducible armhf build of `circuits`.
- A versioned configuration file containing API port, serial settings, GPIO
  assignments, and logging level.
- A systemd service that starts after networking, runs as a non-root user where
  GPIO permissions permit, restarts on failure, and stops motors on shutdown.
- A startup self-test that leaves both motors stopped unless an explicit test
  command is issued.
- A GPIO cleanup path that stops motors and releases pins on normal shutdown and
  controlled failure.

### Drive and API features

- A continuous joystick command carries normalized forward/backward and
  left/right values in the range `-1..1`.
- The Raspberry performs differential-drive mixing to calculate bounded left
  and right motor targets.
- `POST /do/stop`: both motors stopped.
- `POST /do/led_on`, `/do/led_off`, `/do/led_switch`.
- `GET /get/status`: returns structured state including command, motor states,
  LED state, startup time, last error, and safety state.
- The Android app presents an on-screen pointer/joystick rather than discrete
  directional buttons. Centering or releasing it commands a stop.
- Every movement command has a bounded timeout/dead-man policy; loss of the
  controller or process must result in stop.
- Invalid commands and hardware failures return explicit non-2xx errors and are
  logged.
- The API is reachable only from the intended LAN or authenticated clients.

### Verification

The first release is working only when all of these pass:

1. GPIO continuity and direction tests with wheels lifted.
2. Stop command stops both wheels repeatedly.
3. Forward/backward and left/right directions match the chassis labels.
4. Ten-minute stationary service run does not crash or leak processes.
5. Ten-minute low-speed floor run does not brown out, overheat the driver, or
   lose control.
6. Battery runtime and safe shutdown are measured and recorded.
7. Android control, or an equivalent `curl` client, passes the same command test.

## Later target: sensors and autonomy

After the minimum robot is stable:

- Arduino UNO is powered and connected to the Raspberry by USB serial.
- The breadboard is used, if still needed, for labelled low-current sensor and
  logic prototyping; motor power remains on secured wiring outside the board.
- HC-SR04 reports distance with explicit timeout/out-of-range handling.
- VL53L0X is added only after its exact wiring, library, address, and usefulness
  are chosen; the experimental sketches are not a production interface.
- Sensor messages use a documented, framed protocol with validation and timestamps.
- Status exposes sensor freshness and fault state.
- Obstacle-stop logic is implemented on the Raspberry and tested with the motors
  physically disconnected first.
- Automatic/action-sequence mode is added only with a hard stop, bounded actions,
  and recovery after stale sensor data.
- The Android app gains connection health, clear error display, and safe stop on
  app disconnect; security is addressed before use beyond a trusted LAN.

ROS2 is not part of this target unless a later decision explicitly reintroduces
it. The selected baseline is the Rust architecture, with ROS2-like node separation
only where it helps the implementation.

