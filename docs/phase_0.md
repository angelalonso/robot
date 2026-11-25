# Phase 0: establish the physical baseline

## Objective

Replace assumptions in the repository with a verified one-page record of the
robot's actual hardware, wiring, power paths, motor directions, and encoder
interfaces. Phase 0 is complete only when another person can reproduce the
wiring from the record without guessing.

## Tasks

### 0.1 Identify and inventory

- [ ] Record the exact Raspberry Pi model, board revision, operating system
  image, and 40-pin header layout.
  >> Raspberry Pi 3 Model B v1.2. Operating system can be any; Ubuntu has
  been used so far. The target is a secured and updated system.
- [ ] List every installed component: Pi, power bank, battery holders and
  cells, L298N, motors, wheels, rear wheel, LED, resistor, switch, Arduino,
  sensors, breadboard, and cables.
  >> Raspberry, Arduino, Power Bank 10000 mah, 6xAA Batteries, L298N
  >> Two 12 V DC encoder gear motors, similar to the ManoMano listing:
  https://www.manomano.de/p/dc-12-v-diy-encoder-getriebemotor-micro-drehzahlreduzierung-motor-vollmetallgetriebe-mit-halterung-65mm-magnetgetriebe-motor-rad-kit-fur-smart-auto-roboter-modell-diy200rpm-86186545
- [ ] Switch off and disconnect both power sources before removing anything.
- [ ] Remove all external cables and unmount the Pi, Arduino, L298N, motors,
  sensors, battery holders, breadboard, switch, LED, and loose wiring while
  keeping each component intact.
- [ ] Label every cable, connector, motor, and loose part, even if its purpose
  is currently unknown.
- [ ] Photograph readable labels, connector positions, jumpers, cable colours,
  motor encoder boards/connectors, and both sides of the L298N.
- [ ] Mark each item as `present`, `missing`, `damaged`, or `unknown`.

**Evidence:** dated inventory table and labelled photographs committed under
`docs/` or linked from the phase record.

### 0.2 Explore and identify components while disconnected

- [ ] Confirm the exact Pi, Arduino, L298N, sensor, battery-holder, and motor
  models from markings or datasheets.
- [ ] For each motor, record nominal voltage, gearbox/output speed, shaft and
  wheel dimensions, encoder type, encoder supply voltage, channel count,
  pulses/counts per revolution, connector pin order, no-load current, and
  stall current. Do not infer encoder voltage or pin order from cable colours.
- [ ] Identify the L298N motor-supply, logic-supply, ground, enable, input, and
  output terminals, regulator jumper, and any onboard indicator.
- [ ] Measure battery-holder continuity, cell count, polarity, and open-circuit
  voltage without connecting it to electronics.
- [ ] Check motors, wheels, brackets, batteries, controller, and wiring for
  damage, loose conductors, exposed shorts, or mechanical interference.

**Evidence:** disconnected-component inventory, motor/encoder pin record, and
unpowered continuity and voltage measurements.

### 0.3 Verify power and safety

- [ ] Decide whether the six-AA pack is electrically suitable for the selected
  motors; six cells are not automatically a 12 V supply.
- [ ] Select a motor supply and fuse based on measured motor stall current and
  the L298N's actual limits, not the motor's no-load current.
- [ ] Identify the L298N 5 V regulator jumper and document whether it is
  installed; do not use it to power the Pi without an electrical review.
- [ ] Confirm the Pi power bank is electrically independent from the motor
  supply.
- [ ] Define the common-ground arrangement for Pi, L298N, Arduino, encoder, and
  sensors; connect grounds only after voltage levels are verified.
- [ ] Confirm that no motor-supply voltage can reach a Pi GPIO, encoder input,
  or 5 V-only Arduino signal path.
- [ ] Record the LED resistor value and defer LED reconnection until the motor
  wiring is independently verified.

**Evidence:** voltage/polarity measurements, continuity results, and a written
power-path diagram. Do not energize motors or encoders while an item remains
unknown.

### 0.4 Resolve the GPIO and direction record

- [ ] Use the GPIO table in `files/env.template` as the software starting point:
  left IN1/IN2/ENA = GPIO27/17/22 and right IN1/IN2/ENA = GPIO24/23/25.
- [ ] Resolve the conflicting historical diagrams before reconnecting direction
  wires; label direction as a software convention, not a motor property.
- [ ] Decide whether encoder signals will be read by the Raspberry Pi or
  Arduino, and verify that their voltage levels and interrupt capacity are
  compatible. The current software does not read motor encoders.

**Evidence:** signed-off Pi/L298N/encoder pin table and an interface decision.

### 0.5 Reassemble and verify in controlled stages

Reconnect only after the disconnected exploration is complete. Build from the
bottom layer upward so the heavy and high-current components are secured before
the low-voltage control wiring is routed:

- [ ] Bench-test the Raspberry Pi and Arduino separately from motor power.
- [ ] On the bottom layer, mount the motors, wheels, sensors, L298N, battery
  holders, switch, fuse, and any rear wheel; keep wheels and moving parts
  accessible for testing.
- [ ] On the bottom layer, route and label motor wires and the battery-to-L298N
  power path, leaving the battery disconnected.
- [ ] Connect the motor battery through the switch/fuse to the L298N, with both
  motors still disconnected, and verify supply voltage and regulator state.
- [ ] Connect one motor to one L298N output, lift the wheel, and test stop,
  forward, backward, current draw, and emergency power removal.
- [ ] Repeat for the second motor and record the physical motor-to-output
  mapping and final direction convention.
- [ ] Connect one encoder only after its supply and pinout are verified; measure
  A/B transitions by hand before enabling closed-loop software.
- [ ] On the upper layer, mount the Raspberry Pi, breadboard, and Arduino with
  clearances from the lower-layer motor and battery wiring.
- [ ] Connect Pi ground and the L298N logic interface only; verify GPIO levels
  with motor power disconnected.
- [ ] Route a labelled inter-layer harness for L298N control, common ground,
  encoder signals, and sensor connections; keep it away from motor terminals.
- [ ] Connect Arduino and the HC-SR04 separately: Arduino 5 V/GND, trigger D3,
  echo D2, then USB serial to the Pi at 9600 baud.
- [ ] Connect the Pi power bank independently and verify that it cannot contact
  the motor supply.
- [ ] Secure all verified components, route cables away from wheels and shafts,
  and perform a final continuity and polarity check.
- [ ] Keep encoder integration out of the minimum drive gate until firmware and
  software support are implemented and tested.

**Evidence:** staged power-up log, motor direction/current results, encoder
signal test, sensor serial test, and final annotated assembly photograph.

### 0.6 Publish the baseline and close the gate

- [ ] Update `docs/current-state.md` with verified facts and remove only
  claims that the evidence disproves.
- [ ] Update `docs/images-and-conflicts.md` to mark resolved conflicts.
- [ ] Create one final wiring diagram or annotate the authoritative existing
  diagram; label earlier diagrams as historical.
- [ ] Record all unresolved items explicitly with an owner and next action,
  especially encoder specifications, motor stall current, supply choice, and
  L298N thermal/current suitability.

**Phase 0 gate:** exact board and components are known; every power and ground
path is documented; motor identity, direction, and encoder interface are
verified; GPIO 21 LED wiring is safe; the Pi, Arduino, L298N, sensors, and
motors have passed staged tests; no unresolved item could cause hardware damage
during normal operation.
