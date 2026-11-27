# Image inventory and document conflicts

## Image meanings

### Chassis photographs

| File | Meaning |
|---|---|
| `img/chassis_lunchbox_1.jpg` | Obsolete first build: lunchbox body with wheels attached. |
| `img/chassis_lunchbox_cut.jpg` | Cutting the lunchbox into a usable motor/chassis section. |
| `img/chassis_cardboard.jpg` | Cardboard extension layer for the obsolete first build. |
| `img/chassis_cardboard_mounted.jpg` | Cardboard extension mounted to the lunchbox section. |
| `img/chassisply_layer_bottom.jpg` | Current documented lower plywood layer with batteries, motors, and L298N. |
| `img/chassisply_layer_top.jpg` | Current documented upper layer for Raspberry, breadboard, Arduino, and future I/O. |
| `img/chassis_bottom.jpg` | Close view of the newer lower chassis assembly. |
| `img/chassis_front_open_bottom.jpg` | Open front/lower view of the newer chassis. |
| `img/chassis_front_open_top.jpg` | Open front/upper view of the newer chassis. |
| `img/chassis_front_closed.jpg` | Closed front view of the newer chassis. |
| `img/conn_raspi_ardu.jpg` | Physical Raspberry-to-Arduino USB/serial connection example, with optional ICSP wires visible. |

### Wiring diagrams

The SVG is treated as the authoritative readable source where a PNG and SVG
share a basename. The files are historical design iterations, not all-current
instructions.

| File family | Meaning |
|---|---|
| `diagram.202009.*` | Early power/controller diagram; labels include the Anker 20100 and 5 V supply. |
| `diagram.202010.*` | Adds tracking sensor KY-033, HC-SR04, L298N, and two motors. |
| `diagram.202011.svg` | Adds buttons and LEDs to the previous sensor/motor design. |
| `diagram.latest.svg` | Latest labelled version of the same Arduino/L298N/sensor concept; it still describes a design later than the current Rust-only milestone. |
| `diagram.ICSP.*` | Arduino ICSP signals: VCC, MOSI, MISO, GND, RESET, SCK. It documents an optional direct-flashing route, not the normal USB runtime path. |
| `diagram.test1.*` | Bench-test arrangement with ICSP, HC-SR04, LEDs, and a 330-ohm resistor. |
| `diagram.test2.*` | Bench-test arrangement expanded with L298N motor-controller terminals, motor outputs, grounds, 12 V/5 V labels, inputs, and enables. |

The diagrams do not by themselves prove the Raspberry GPIO numbers used by the
current Rust configuration. Use the pin tables in `docs/current-state.md`
and verify them electrically.

## Conflicts found

| Topic | Conflicting sources | Decision for this plan |
|---|---|---|
| Raspberry OS | Archived setup documents describe Ubuntu 64-bit/aarch64 or Pi 1-era Raspberry Pi OS/armhf. | User selected Raspberry Pi 3 Model B v1.2. Select and document the OS during bring-up; 32-bit armhf remains the conservative software target. |
| Software tree | Several setup documents refer to `brain/`; the tracked executable code is `circuits/`. | User selected Rust `circuits` as the baseline. Provisioning must be corrected before deployment. |
| ROS2 | README describes abandoned ROS2 Python/C++ experiments; current crate has no ROS2 dependency. | Do not install or target ROS2 for the minimum robot. |
| Arduino timing | The archived Arduino document describes sensor integration; README says the current robot manages two motors through an API; `main.rs` still starts an Arduino node. | First working milestone excludes Arduino/sensors; retain them as Phase 6. |
| Right motor pins | Chassis/L298N text says IN1=GPIO23 and IN2=GPIO24; `files/env.template` says IN1=GPIO24 and IN2=GPIO23. | Unresolved. Verify with continuity and lifted-wheel direction test before motor power. |
| LED pin | `files/env.template` uses GPIO21; older documents show generic LED tests and no single final LED wiring record. | Keep GPIO21 as the intended software default, but verify resistor and physical connection. |
| Provisioning | `files/archive/maybe-obsolete/autosetup.sh` uses cloud-init/Ubuntu assumptions, installs ROS2/Python, and copies to `/robot/brain`; `install_to_raspberry.sh` is a separate manual Raspberry Pi OS path. | Use the simpler armhf path as the basis, then replace it with a tested Rust-specific installer. |
| Runtime service | `files/archive/maybe-obsolete/robot.service.template` starts `/home/NEWUSER/robot/brain/run.sh`, which is absent. | Service is not deployable as written; create a `circuits` service in the roadmap. |

## Questions that remain hardware-dependent

These are intentionally not guessed: exact Pi revision, motor ratings, battery
condition/capacity, L298N jumper/regulator state, final motor-side labels,
grounding, LED resistor, and measured runtime must be checked on the physical
robot.

