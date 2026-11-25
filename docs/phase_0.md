# Phase 0: establish the physical baseline

## Objective

Replace assumptions in the repository with a verified one-page record of the
robot's actual hardware, wiring, power paths, and motor directions. Phase 0 is
complete only when another person can reproduce the wiring from the record
without guessing.

## Tasks

### 0.1 Identify and inventory

- [ ] Record the exact Raspberry Pi model, board revision, operating system
  image, and 40-pin header layout.
  >> Raspberry pi 3 Model B v1.2. Operating system can be any, I have used Ubuntu so far. I would like it to be a secured and updated system.
- [ ] List every installed component: Pi, power bank, battery holders and
  cells, L298N, motors, wheels, rear wheel, LED, resistor, switch, Arduino,
  sensors, breadboard, and cables.
  >> Raspberry, Arduino, Power Bank 10000 mah, 6xAA Batteries, L298N
  >> 2x 12V DC motors like https://www.manomano.de/p/dc-12-v-diy-encoder-getriebemotor-micro-drehzahlreduzierung-motor-vollmetallgetriebe-mit-halterung-65mm-magnetgetriebe-motor-rad-kit-fur-smart-auto-roboter-modell-diy200rpm-86186545#sidePanel-description 
- [ ] Photograph readable labels, connector positions, jumpers, and the
  underside of the L298N.
- [ ] Mark each item as `present`, `missing`, `damaged`, or `unknown`.

**Evidence:** dated inventory table and labelled photographs committed under
`docs/` or linked from the phase record.

### 0.2 Verify the mechanical layout

- [ ] Label the physical left and right motors while viewing the robot from
  its forward direction.
- [ ] Record which L298N output pair drives each motor.
- [ ] Check that motors, wheels, batteries, controller, and wiring are
  mechanically secured and cannot contact moving parts.
- [ ] Record the location and operation of the motor power switch/disconnect.

**Evidence:** annotated chassis photograph and motor/output mapping.

### 0.3 Verify power and safety

- [ ] Measure and record the motor battery voltage and polarity with no load.
- [ ] Record battery chemistry, cell count, holder wiring, and charger.
- [ ] Identify the L298N 5 V regulator jumper and document whether it is
  installed; do not use it to power the Pi without an electrical review.
- [ ] Confirm the Pi power bank is electrically independent from the motor
  supply.
- [ ] Verify the Pi ground, L298N logic ground, and any Arduino ground
  connections required by the design.
- [ ] Confirm that no motor-supply voltage can reach a Pi GPIO or 5 V-only
  Arduino signal path.
- [ ] Record the LED resistor value and verify that the LED is connected to
  GPIO 21 as intended.

**Evidence:** voltage/polarity measurements, continuity results, and a written
power-path diagram. Do not energize motors while an item remains unknown.

### 0.4 Resolve the GPIO and direction record

- [ ] Check every Pi-to-L298N wire against the configured GPIO table in
  `files/env.template`.
- [ ] Resolve the right-motor conflict: chassis notes describe IN1=GPIO23 and
  IN2=GPIO24, while the template uses IN1=GPIO24 and IN2=GPIO23.
- [ ] With wheels lifted and motor power limited, test forward, backward, and
  stop for each motor.
- [ ] Record the final GPIO-to-function table and the observed direction of
  each motor.

**Evidence:** signed-off pin table and a short direction-test log.

### 0.5 Publish the baseline and close the gate

- [ ] Update `docs/current-state.md` with verified facts and remove only
  claims that the evidence disproves.
- [ ] Update `docs/images-and-conflicts.md` to mark resolved conflicts.
- [ ] Create one final wiring diagram or annotate the authoritative existing
  diagram; label earlier diagrams as historical.
- [ ] Record all unresolved items explicitly with an owner and next action.

**Phase 0 gate:** exact board and components are known; every power and ground
path is documented; motor identity and direction are verified; GPIO 21 LED
wiring is safe; no unresolved item could cause hardware damage during a
disconnected-motor software test.
