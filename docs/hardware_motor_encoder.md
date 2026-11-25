# DC encoder gearmotor

## Identification status

The motor is a 12 V DC gearmotor with an integrated two-channel Hall
quadrature encoder. The exact manufacturer and model are not yet confirmed.
The construction is consistent with generic JGB37/JGA37-class encoder
gearmotors, but that family name must not be treated as an identification.

The associated product listing describes a DIY 200 RPM variant with a metal
gearbox, mounting bracket, and approximately 65 mm wheel:

<https://www.manomano.de/p/dc-12-v-diy-encoder-getriebemotor-micro-drehzahlreduzierung-motor-vollmetallgetriebe-mit-halterung-65mm-magnetgetriebe-motor-rad-kit-fur-smart-auto-roboter-modell-diy200rpm-86186545>

## Connector pinout

This is the observed pin schema on the red circuit board. The order is
top-to-bottom when viewing the board with the motor axle to the left:

| Pin | Function | Electrical domain |
|---|---|---|
| M1 | Motor power input, positive pole | Motor supply; nominally 12 V |
| GND | Hall/encoder supply ground | Logic ground |
| C1 | Encoder signal, phase A | Logic signal |
| C2 | Encoder signal, phase B | Logic signal |
| VCC | Hall/encoder supply positive | 3.3 V or 5 V, to be verified |
| M2 | Motor power input, negative pole | Motor supply return |

`M1` and `M2` are the two motor terminals and must be driven by an H-bridge
such as the L298N. `C1` and `C2` are the quadrature feedback channels; their
phase relationship indicates rotation direction.

## Safety and connection constraints

- Never connect 12 V to `VCC`, `C1`, or `C2`.
- `VCC` accepts 3.3 V or 5 V according to the observed schema, but the
  encoder's actual operating range and output type still require confirmation.
- Raspberry Pi GPIOs are 3.3 V only. Use 3.3 V encoder power or level shifting
  before connecting encoder outputs to the Pi.
- Connect encoder `GND` to the controller logic ground and do not assume cable
  colours identify pins.
- Do not connect the encoder until the connector orientation has been checked
  against the physical board and a continuity/voltage test is complete.
- The motor's no-load current, stall current, encoder counts per revolution,
  gearbox ratio, output speed, and torque remain unknown.

## Intended project integration

Initial drive validation should use:

```text
L298N OUT1/OUT2  -> M1/M2
logic ground     -> encoder GND
verified logic V -> encoder VCC
controller input -> C1/C2
```

The current software drives the motors through the L298N but does not read
`C1`/`C2`. Encoder integration therefore requires a later firmware/software
change. The Arduino currently uses D2 and D3 for the HC-SR04 echo and trigger,
so encoder input pins must be assigned deliberately if the Arduino reads the
encoder. Alternatively, the Raspberry Pi can read the channels after voltage
compatibility is established.

## Remaining verification

- [ ] Photograph the actual connector and board orientation.
- [ ] Confirm encoder `VCC` range and output type.
- [ ] Measure motor winding resistance and encoder supply current without
  applying excessive voltage.
- [ ] Determine encoder counts/pulses per motor and gearbox output revolution.
- [ ] Measure no-load and stall current using a current-limited test setup.
- [ ] Confirm output speed, wheel diameter, shaft dimensions, and torque.
- [ ] Record the exact motor-to-L298N output mapping after the lifted-wheel test.
