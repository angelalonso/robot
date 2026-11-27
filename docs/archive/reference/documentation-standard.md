# Robot project documentation standard

This document is the operating standard for turning this repository into a
reproducible, step-by-step build guide for this robot. Apply it whenever a new
phase is created or an existing phase changes. A phase is not complete because
the code works on a development machine; it is complete only when its required
evidence and acceptance gate are recorded.

## Repository documentation roles

### Project documents

The active files directly under `docs/` contain decisions and instructions for
the current phase:

- `current-state.md`: verified facts and explicitly unverified assumptions;
- `target-state.md`: the intended working robot and its safety boundaries;
- `roadmap.md`: phase order, dependencies, and high-level gates;
- `images-and-conflicts.md`: image meaning, historical material, and conflicts;
- this document: the standard every phase must follow.

### Phase documents

The current phase uses clearly prefixed files directly under `docs/`, such as
`phase_0.md`, `phase_0_software.md`, and `phase_0_tests.md`, plus the current
hardware requirements document. When a phase is completed, move its complete document set
to `docs/phase_N/`. Its main document should be named `README.md` only after it
is historical, so the active documentation remains visible at `docs/`.

- `hardware.txt`: ASCII schematic for the phase's physical setup;
- `software.md`: exact software, versions, configuration, interfaces, and
  runtime behavior;
- `tests.md`: test procedures, expected results, and recorded results;
- `decisions.md`: decisions, alternatives rejected, and unresolved questions;
- `evidence/`: photographs, measurements, logs, captures, and test records
  that are safe and appropriate to keep in the repository.

Use sequential phase numbers. Do not mix historical experiments into an active
phase. Move superseded material to `docs/archive/` and explain why it is no
longer authoritative.

## Required phase structure

Every phase document must contain the following sections in this order.

### 1. Phase identity and status

Record:

- phase number and stable name;
- status: `planned`, `in-progress`, `blocked`, `verified`, or `superseded`;
- date started and date verified;
- prerequisites and dependent phases;
- hardware/software baseline used;
- owner or responsible person;
- links to the previous and next phase.

Do not call a phase `verified` until its gate is met and the evidence is
linked. If a physical check has not happened, mark it `unverified` rather than
inferring success from documentation or simulation.

### 2. Goal

State one measurable outcome in one or two sentences. The goal must describe
what the builder will have at the end, not the activities used to get there.

Good:

> The Raspberry Pi and L298N are mounted safely, powered through independent
> paths, and all six control wires are verified without motor movement.

Bad:

> Work on the robot wiring.

### 3. Lower goals and ordered tasks

Break the goal into small, ordered lower goals. Each lower goal must include:

- purpose;
- prerequisites;
- exact actions;
- expected result;
- evidence to capture;
- failure/recovery instructions;
- a checkbox that can be marked only after the evidence exists.

Tasks must be safe to stop and resume. Identify destructive actions explicitly,
including battery connection, motor power, firmware flashing, filesystem
changes, package upgrades, and service replacement.

### 4. Bill of materials and tools

List every required item before the first action:

| Item | Required specification | Quantity | Verification |
|---|---|---:|---|
| Controller | Exact board/model/revision | 1 | Photograph or label |
| Instrument | Multimeter or specified tool | 1 | Calibration/known-good check |

Include:

- exact component model and relevant ratings;
- quantity and acceptable alternatives;
- cables, connectors, fasteners, insulation, fuses, switches, and tools;
- required computer/OS and privileges;
- replacement parts and known incompatible parts;
- required measurements and unit conventions.

Never describe a component only as “a battery”, “a motor”, or “a Raspberry”.
Record voltage, current, capacity, connector, polarity, and model whenever
those properties affect safety or compatibility.

### 5. Safety and constraints

Every phase must state:

- electrical hazards and maximum permitted voltages/currents;
- mechanical hazards, pinch points, loose parts, and wheel clearance;
- battery, heat, fire, and polarity precautions;
- whether motors must be disconnected, wheels lifted, or a physical kill
  switch within reach;
- software safety behavior on startup, timeout, crash, disconnect, and shutdown;
- network/security constraints and whether credentials are involved;
- actions that are prohibited until a prior gate is complete.

Use a stop condition:

> Stop immediately if any voltage, polarity, ground, motor identity, or
> software state is unknown.

Do not use “be careful” as the only safety instruction. State the observable
condition that allows work to continue.

### 6. Hardware schematic

Every phase that changes or depends on hardware must provide an ASCII schematic
in `hardware.txt`. It must be readable in a terminal and version-controlled.
Include:

- power sources, voltage rails, switches, fuses, and polarity;
- all grounds and whether they are common;
- controller pins by both logical name and physical/header pin where applicable;
- motor-driver inputs, enables, outputs, and motor identity;
- sensors, signal direction, voltage level, pull-ups, and protection;
- unused or intentionally disconnected pins;
- measurement points and test states;
- a legend and assumptions.

Use explicit labels rather than relying on spatial position. Example:

```text
Pi 5V USB power bank
  +5V ------------------> Pi 5V input
  GND -------------------+--------------------+
                         |                    |
Motor battery pack      Pi GND          L298N logic GND
  +V -- fuse -- switch -> L298N VMOT
  0V --------------------+--------------------+

Pi BCM GPIO22 / pin 15 ------> L298N ENA / left PWM
Pi BCM GPIO27 / pin 13 ------> L298N IN1 / left direction
Pi BCM GPIO17 / pin 11 ------> L298N IN2 / left direction
L298N OUT1/OUT2 -------------> physically labelled LEFT MOTOR
```

The schematic is not sufficient by itself. Pair it with a pin table and
continuity/direction measurements. Mark diagrams as `authoritative`,
`historical`, or `bench-only`.

### 7. Software specification

Every phase that uses software must provide `software.md` containing:

- operating system, architecture, board, and kernel assumptions;
- language/toolchain versions and package/dependency versions;
- exact installation commands;
- repository commit/version used;
- configuration variables, defaults, valid ranges, and secret handling;
- filesystem paths and ownership/permissions;
- processes, services, ports, devices, and environment variables;
- API/CLI/serial/UDP protocols and message examples;
- startup, normal operation, error, timeout, restart, and shutdown behavior;
- logging location and how to inspect diagnostics;
- resource limits and expected CPU, memory, storage, and network use;
- reproducibility and rollback instructions;
- known limitations and hardware-only behavior.

Do not document a command without stating the directory where it runs. Do not
document a secret value. Use placeholders and explain how the builder supplies
the secret securely.

### 8. Tests and evidence

Each lower goal must map to at least one test. Prefer the strongest available
test in this order:

1. automatic unit/integration test;
2. deterministic software command or protocol test;
3. static check, build, lint, or configuration validation;
4. instrumented hardware test with measured output;
5. manual physical inspection or observation.

The phase must define:

- test ID and purpose;
- prerequisites and safe test state;
- exact command/procedure;
- expected output and pass/fail criteria;
- actual result, date, hardware/software version, and operator;
- log/photo/measurement evidence;
- cleanup and recovery procedure.

A test that only says “it worked” is not evidence. Record numbers, states,
responses, timings, temperatures, voltages, or repeat counts where applicable.

For hardware tests, record ambient conditions and instrument accuracy when they
could affect the result. For movement tests, record wheel orientation, speed,
duration, surface, battery state, and emergency-stop readiness.

### 9. Troubleshooting and recovery

Document the likely failures for the phase:

| Symptom | Likely causes | Safe diagnosis | Recovery |
|---|---|---|---|
| Pi resets when motor starts | Brownout, shared power path | Disconnect motor power and measure rail | Stop, isolate supplies, recheck wiring |

Include how to:

- return the hardware to a safe unpowered state;
- stop or disable software;
- undo configuration changes;
- restore a known-good version;
- collect logs without exposing credentials;
- decide whether the phase is blocked.

### 10. Completion gate and handoff

End every phase with a gate containing objective pass criteria. The gate must
cover hardware, software, tests, safety, and documentation. A phase may advance
only when:

- all required lower goals are checked;
- all mandatory tests pass;
- failed or skipped tests are explicitly listed with a reason;
- the final schematic and software specification match the tested setup;
- current-state and roadmap documents are updated;
- open risks have an owner and next action;
- the next phase's prerequisites are satisfied;
- a clean handoff can be performed by someone who did not do the work.

The handoff must state the exact starting point for the next phase, including
power state, software version, configuration path, and known limitations.

## Cross-phase records that must be maintained

### Configuration and version record

Keep a table of the authoritative versions:

- operating system image and architecture;
- Raspberry board revision;
- Rust toolchain and Cargo lockfile;
- Android/Arduino toolchain where applicable;
- installed system packages;
- firmware and bootloader versions;
- repository commit/tag;
- wiring revision and date.

### Interface register

Record every boundary:

- GPIO pin and electrical level;
- motor-driver terminal and direction;
- serial device, baud, framing, and timeout;
- UDP/API port, route, method, request, response, and failure status;
- systemd service, user, working directory, and environment file;
- Android configuration and connection behavior.

### Decision log

For every consequential choice, record:

- decision and date;
- context and constraints;
- alternatives considered;
- reason for selection;
- consequences and revisit trigger.

Do not silently replace a wiring, operating system, library, protocol, or
architecture decision. Mark the old version historical and update the
authoritative references.

### Risk register

Track risks with severity, likelihood, affected phase, mitigation, owner, and
status. Include at least power isolation, GPIO voltage, motor runaway, battery
overcurrent, thermal behavior, network exposure, credential handling, and
stale sensor data when relevant.

### Change and evidence policy

- Date photographs and measurements.
- Keep original measurement values; do not replace them with conclusions.
- Never commit passwords, private keys, tokens, personal data, or unreviewed
  logs containing them.
- Prefer text, SVG, and compressed images that remain reviewable in Git.
- Attribute external schematics, libraries, and copied material.
- Keep generated build output, caches, and local credentials ignored.
- Link evidence from the phase document instead of relying on filenames alone.

## Standard phase template

Copy this outline when starting a new phase:

```text
# Phase NN: Name

Status: planned
Prerequisites:
Previous phase:
Next phase:

## Goal

## Lower goals and tasks

### NN.1 Lower goal
- [ ] Action
- Expected result:
- Evidence:
- Recovery:

## Bill of materials and tools

## Safety and constraints

## Completion gate

## Open questions
```

Create `hardware.txt`, `software.md`, `tests.md`, and `decisions.md` when the
phase needs them. Do not create empty files solely for symmetry; every file
must contain the information needed to reproduce or verify the phase.

## Rules for future work

Before changing code or hardware:

1. Identify the phase and its current gate.
2. Update the phase task and acceptance criteria if the work is not already
   specified.
3. Record the intended hardware/software baseline.
4. Make the smallest safe change.
5. Run automatic tests first, then software and hardware tests in increasing
   risk order.
6. Record evidence and update current state.
7. Reconcile schematics, specifications, diagrams, and archived material.
8. Do not advance the roadmap until the gate is objectively satisfied.
