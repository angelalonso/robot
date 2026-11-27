# Phase 0 test record

This file defines the evidence required to close Phase 0. Replace `pending`
with measured values and link photographs or logs in `evidence/` when tests are
performed. A software mock is not evidence of physical GPIO behavior.

| ID | Test | Preconditions | Expected result | Result |
|---|---|---|---|---|
| P0-HW-01 | Identify controller | Power disconnected | Exact Pi model/revision and header layout recorded | pending |
| P0-HW-02 | Inventory hardware | Robot unpowered | Every component has status and photo/label evidence | pending |
| P0-HW-03 | Measure motor supply | Batteries isolated from Pi | Voltage, polarity, chemistry, and cell arrangement recorded | pending |
| P0-HW-04 | Check Pi supply isolation | Batteries disconnected | Motor supply cannot reach Pi rails or GPIOs | pending |
| P0-HW-05 | Verify grounds | Power disconnected | Required Pi/L298N ground continuity recorded | pending |
| P0-HW-06 | Verify GPIO continuity | Power disconnected | Every configured wire reaches the intended terminal | pending |
| P0-HW-07 | Verify LED circuit | GPIO output unpowered | GPIO21, resistor value, LED polarity, and return path recorded | pending |
| P0-HW-08 | Identify motors | Wheels lifted, current limited | Physical left/right and L298N output mapping recorded | pending |
| P0-SW-01 | Validate configuration table | Repository checkout | `files/env.template` matches the signed-off pin table | pending |
| P0-SW-02 | Validate documentation links | Repository checkout | Active phase/project links resolve | pending |

## Required test record

For each completed test, add:

- date and operator;
- hardware identifiers and battery state;
- instrument and measurement range;
- exact measured values, not only pass/fail;
- photograph, terminal log, or other evidence path;
- follow-up action for every deviation.

## Phase gate

Phase 0 remains blocked until P0-HW-01 through P0-HW-08 pass and P0-SW-01
through P0-SW-02 pass. No motor-powered test may begin with a pending hardware
identity, polarity, ground, or GPIO-direction result.
