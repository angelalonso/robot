# Phase 0 software baseline

Phase 0 is primarily a physical verification phase. Software is used only to
record the intended configuration and to validate documentation consistency.
No software result can replace a multimeter measurement or a lifted-wheel test.

## Authoritative software inputs

| Item | Location | Role | Status |
|---|---|---|---|
| GPIO and runtime configuration template | `files/env.template` | Intended pins and runtime defaults | Must be reconciled with `tests.md` |
| Rust executable | `circuits/` | Current robot implementation | Not a Phase 0 hardware proof |
| GPIO cleanup helper | `circuits/reset_gpio.sh` | Development cleanup aid | Must not be treated as an emergency stop |
| Project documentation | `docs/` | Requirements and evidence for the current phase | Active |

## Software-only checks

Run from the repository root:

```text
bash -n circuits/reset_gpio.sh
git diff --check
```

The configured GPIO table must be compared manually with the approved physical
wiring record in `docs/phase_0.md`. Do not run
the motor-control binary on connected hardware until the Phase 0 gate is closed.

## Known limitations

- The x86_64 GPIO implementation is a mock.
- The current application does not prove that a physical motor stopped after a
  process crash.
- The current API is unauthenticated and is not approved for exposure beyond a
  controlled network.
- Runtime deployment and service installation belong to a later phase.
