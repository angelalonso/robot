# Phase 0: completed work

This file records facts and decisions already established. It is not a
substitute for physical measurements.

## Reproducibility bill of materials

This is the current list of hardware needed to reproduce the robot. An item is
only marked `confirmed` after its physical identity is checked and recorded.

| Item | Required for | Current record | Status |
|---|---|---|---|
| Raspberry Pi 3 Model B v1.2 | Controller | Onboard Wi-Fi, 40-pin header | Confirmed by user; physical label pending |
| L298N dual H-bridge module | Motor control | Common module layout; exact board pending | Provisional |
| Two DC encoder gearmotors | Drive | Nominally 12 V; exact model/rating pending | Provisional |
| Two compatible wheels | Drive | Dimensions and shaft fit pending | Unknown |
| Six-AA battery holder and six cells | Temporary motor tests | Chemistry and capacity pending | Provisional |
| Final motor battery and charger | Normal operation | To be selected from measurements | Not selected |
| USB power supply/power bank | Raspberry Pi | Existing power bank reported; model/output pending | Provisional |
| Motor switch and fuse | Safe motor power | Availability and ratings pending | Unknown |
| Multimeter and insulated test wires | Bring-up | Required test tools | Required, availability pending |
| Chassis and rear support wheel | Mobile robot | Existing parts reported; condition/fit pending | Provisional |
| Breadboard | Low-current prototyping and signal distribution | Available; exact size/type pending | Reported by user |
| Arduino and sensors | Later phases only | Not required for first drive milestone | Deferred |

When a part is confirmed, record its exact model, rating, supplier or
identifying link, and any compatibility notes in this table.

## Confirmed decisions

- The robot is being rebuilt from a disconnected state.
- All previous wiring, GPIO assignments, jumper positions, and configuration
  are treated as untrusted historical material.
- The controller is a Raspberry Pi 3 Model B revision 1.2 with onboard Wi-Fi
  and a standard 40-pin header.
- The existing six-AA pack is allowed only for controlled temporary tests.
  It is not the final motor supply.
- The breadboard is reserved for low-current prototyping, sensor wiring,
  signal breakout, and temporary logic-ground distribution. It must not carry
  motor current or be used as an unverified battery-power distribution path.
- The Arduino and sensors are excluded from the first working-drive milestone.
- Every hardware observation, wiring change, test result, or goal change must
  update this record and the active Phase 0/task documents.

## Work completed

- The robot was dismantled and external wiring disconnected.
- The original documentation was separated into active working documents and
  archived reference/history folders.
- A minimum bring-up approach was defined: Pi, L298N, one motor, temporary
  battery, Pi supply, multimeter, and safe mechanical restraint.
- The reproducibility bill of materials is the authoritative list of what must
  be bought or supplied to recreate the current robot.

## Still unverified

- Physical L298N board labels and `5V-EN` jumper state.
- Battery chemistry, polarity, capacity, and loaded voltage.
- Motor model, current, encoder pinout, and voltage compatibility.
- Final wiring, GPIO assignment, motor-output mapping, and motor direction.
- Operating system image and physical component condition.
- Exact purchase/model information for every provisional or unknown item.

