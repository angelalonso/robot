# Phase 0: establish and test the physical baseline

Phase 0 is the active hardware bring-up phase. Build from zero; do not reuse
old wiring or configuration without explicitly re-verifying it.

## Objective

Safely identify enough hardware to perform a one-motor test, record the
measurements, and produce an approved wiring map. The exact motor model may
remain unknown during the first tests.

## Current starting point

- Raspberry Pi 3 Model B v1.2 is identified.
- The robot is dismantled and all wiring is disconnected.
- The six-AA pack is a temporary test supply only.
- The L298N image supplied by the user is a reference, not proof of the actual
  board layout or jumper state.

## Remaining work

### 0.1 Identify the minimum test set

- [ ] Photograph the physical L298N, especially terminal labels and `5V-EN`.
- [ ] Photograph the six-AA holder and record cell arrangement and polarity.
- [ ] Confirm one motor, the Pi supply, a multimeter, insulated wires, and a
      safe way to lift or restrain the motor.
- [ ] Mark each available item present, missing, damaged, or unknown.
- [ ] Update the reproducibility bill of materials in `phase_0-done.md` with
      each confirmed item's exact model, rating, and identifying source.

### 0.2 Make unpowered measurements

- [ ] Measure battery open-circuit voltage and polarity.
- [ ] Check battery-holder continuity.
- [ ] Record L298N terminal names and jumper state.
- [ ] Photograph the motor connector and encoder board; do not connect encoder
      wires until voltage and pinout are known.
- [ ] Confirm the Pi supply is separate from the motor supply.

### 0.3 Define fresh wiring

- [ ] Choose one L298N output pair for the first motor.
- [ ] Define motor-supply positive, motor-supply ground, Pi ground, enable, and
      two direction inputs.
- [ ] Decide the `5V-EN` jumper state only after the actual board and supply
      arrangement are identified.
- [ ] Record the complete cable-by-cable wiring and update software GPIO
      configuration to match it.

### 0.4 Perform staged tests

- [ ] Power the Pi alone and verify the operating system/GPIO method.
- [ ] Power the L298N with the motor disconnected and verify polarity, voltage,
      regulator behaviour, and temperature.
- [ ] Connect one motor, keep it lifted, and issue brief stop/forward/reverse
      tests with a physical power disconnect ready.
- [ ] Record startup current, voltage sag, direction, and L298N temperature.
- [ ] Repeat for the second motor only after the first test is understood.
- [ ] Use the six-AA pack only for low-speed, short-duration tests.

### 0.5 Close Phase 0

- [ ] Record the approved wiring, motor/output mapping, and measured results.
- [ ] Select a final motor battery and fuse from measured current/runtime needs.
- [ ] Update the bill of materials, `phase_0-done.md`, and this file after
      every hardware confirmation, change, or test.
- [ ] Do not add Arduino, sensors, encoders, or chassis assembly to the
      minimum gate until basic motor control is safe and repeatable.

## Safety gate

Do not energize motors while polarity, L298N terminals, ground paths, or the
motor-power disconnect remain unknown. Never connect motor supply voltage to a
Pi GPIO or encoder signal.
