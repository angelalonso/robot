# Hardware requirements and bring-up plan

## Purpose

Define the minimum hardware needed to build and test the robot from a
disconnected state. Previous wiring, jumper positions, GPIO assignments, and
power choices are not authoritative.

## Available or reported hardware

These items have been reported or documented, but physical presence and
condition still need to be checked:

| Item | Current information | Status |
|---|---|---|
| Controller | Raspberry Pi 3 Model B, revision 1.2, 40-pin header, onboard Wi-Fi | Reported; verify board label |
| Motor driver | Common L298N dual H-bridge module | Reported; verify physical board and jumper |
| Drive motors | Two nominally 12 V DC encoder gearmotors | Exact model and electrical ratings unknown |
| Motor battery | Six-AA holder/pack | Chemistry, capacity, and loaded voltage unknown |
| Pi supply | USB power bank | Exact model and output capability to verify |
| Mechanical drive | Two wheels, chassis parts, rear support wheel | Presence and condition to verify |
| Wiring parts | Switch, fuse if available, breadboard, jumper wires, fasteners | Inventory incomplete |
| Expansion hardware | Arduino UNO and distance sensors | Not required for the first drive test |

## Minimum first-test requirements

The first powered test requires only:

1. Raspberry Pi 3 Model B.
2. L298N board.
3. One motor.
4. Six-AA pack, used only as a temporary test supply.
5. Pi USB power supply or power bank.
6. A multimeter.
7. Insulated wires suitable for the motor current.
8. A way to keep the motor/wheel lifted and prevent free cables from contacting
   moving parts.

The Arduino, sensors, encoders, chassis, second motor, and Android controller
are explicitly out of scope for the first motor test.

## Breadboard role and limits

The available breadboard may be used later as a temporary, clearly labelled
low-current prototyping area for sensor power and signals, Arduino connections,
logic-level testing, and a common-ground distribution point. Its exact type,
rail continuity, current rating, and condition must be checked before use.

Do not route motor supply current, L298N motor outputs, unfused battery current,
or any other high-current path through the breadboard. The breadboard does not
replace a fuse, switch, strain relief, level shifter, or permanent connector.
Any breadboard wiring used during a test must be photographed or transcribed
and removed or replaced with a secured harness before mobile operation.

## Requirements before energizing anything

- Identify the labels on the physical L298N board and record the `5V-EN`
  jumper state.
- Identify the battery holder polarity and measure its open-circuit voltage.
- Confirm the Pi supply is separate from the motor supply.
- Confirm the motor supply cannot reach a Pi GPIO or encoder signal.
- Confirm the L298N ground and Pi ground connection plan.
- Use a fuse or current-limited source where available.
- Keep the motor mechanically restrained or lifted for the first test.
- Have a physical motor-power disconnect within reach.

## Bring-up sequence

### Test 1: inventory and unpowered checks

Photograph the L298N, motor connector/encoder board, battery holder, and power
bank. Record visible labels, cable colours only as observations, jumper
positions, continuity, polarity, and measured battery voltage.

### Test 2: Pi alone

Power the Pi independently. Confirm it boots, Wi-Fi works, and the intended
operating system and GPIO access method are available. Do not connect motor
power.

### Test 3: L298N power section

With the motor disconnected, connect only the verified temporary motor supply
and L298N power terminals according to the board labels. Verify polarity,
voltage, regulator/jumper behaviour, and temperature. Do not connect the Pi
GPIO header until the logic-power arrangement is known.

### Test 4: one lifted motor

Connect one motor to one L298N output. Add the Pi ground and only the explicitly
chosen enable/direction signals. Start with a stop state, then use short
forward and reverse pulses. Record direction, startup current, loaded voltage,
and driver temperature. Disconnect motor power immediately if anything is
unexpected.

### Test 5: second motor

Repeat Test 4 for the second output and motor. Record the physical motor-to-
output mapping rather than assuming left/right identity.

### Test 6: low-speed floor test

Only after both lifted-wheel tests pass, mount the motors securely and perform
brief low-speed tests. Record voltage sag, current, driver temperature, control
stability, and emergency-stop behaviour.

## Deferred requirements

The following are required before normal operation but not before the first
lifted-wheel test:

- Final motor battery selected from measured motor current, runtime, and
  L298N thermal results.
- Fuse and switch rated for the measured motor current.
- Encoder voltage, pinout, counts, and input destination verified.
- Chassis mechanically secured with protected wiring and strain relief.
- Software GPIO configuration matched to the approved fresh wiring plan.
- Arduino and sensor wiring tested independently.

## Current acceptance rule

No component is considered verified because it appears in an old diagram or
shopping list. A requirement becomes verified only after a physical
observation, measurement, or controlled test is recorded in
`docs/phase_0.md`.

Every change to the hardware, wiring, power plan, test result, or project goal
must be reflected in `docs/phase_0.md` and in any affected active document
before the change is treated as part of the current build.
