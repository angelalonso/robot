# Phase 0 decisions and open questions

## Current decisions

| Date | Decision | Reason | Revisit trigger |
|---|---|---|---|
| 2026-11-24 | Use Raspberry Pi OS armhf as the initial deployment target | The target hardware is a Pi 1 B+ and the active deployment path targets armhf | Hardware is replaced or a different architecture is selected |
| 2026-11-24 | Treat Rust `circuits/` as the software baseline | It is the active executable implementation; the old `brain/` tree is absent | Architecture is deliberately replaced |
| 2026-11-24 | Exclude Arduino from the minimum working-robot gate | Drive, power, and remote control must be proven independently first | Minimum robot is stable and sensing work begins |

## Open questions

- Exact Raspberry Pi model and board revision.
- Motor battery chemistry, voltage under load, and capacity.
- L298N regulator jumper and heat-sinking state.
- Physical motor-to-output mapping.
- Final right-motor IN1/IN2 direction.
- LED resistor value and physical wiring.
- Common-ground and signal-level arrangement for future Arduino use.
