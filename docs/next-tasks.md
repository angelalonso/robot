# Next tasks

These are small, concrete tasks intended to take roughly 5 minutes to 2 hours.
Complete them in order and update `phase_0.md` and `phase_0-done.md` when their
results change the project state. Report each completion using the template in
`docs/testing-and-calibration.md`; a task is not complete until its result is
recorded.

## First task: headless Raspberry Pi setup and security (60–120 minutes)

This task uses the Linux laptop to prepare the Raspberry Pi 3 Model B v1.2
without an external monitor or keyboard. Keep the Pi disconnected from the
L298N, motors, Arduino, and motor battery throughout this task. Use only a
known-good Pi power supply or power bank.

### 1. Prepare the laptop

1. Install Raspberry Pi Imager from the official Raspberry Pi website or your
   Linux distribution's trusted package source.
2. Make sure the laptop is connected to the Wi-Fi network that the Pi will
   use. The Pi and laptop must be on the same local network for the first SSH
   connection.
3. Create an SSH key if you do not already have one:

   ```bash
   ssh-keygen -t ed25519 -C "robot-pi"
   ```

   Accept the default file location and protect the key with a passphrase.
   Never copy the private key to the Pi or commit it to this repository.
4. Have an empty microSD card (at least 16 GB, preferably a reliable 32 GB
   card) available. Imager will erase it completely.

### 2. Install Raspberry Pi OS

1. Insert the microSD card into the laptop.
2. Open **Raspberry Pi Imager**.
3. Choose **Raspberry Pi 3** as the device.
4. Choose **Raspberry Pi OS Lite (64-bit)** unless a later task identifies a
   compatibility reason to use the 32-bit image. Do not install a desktop
   environment; it is unnecessary for this robot.
5. Select the microSD card as the storage device and confirm that it may be
   erased.
6. Open the OS customization settings before writing the card and configure:
   - a unique hostname, for example `robot-pi`;
   - a normal user name that is not `pi`, for example `robot`;
   - a long, unique temporary password;
   - the correct Wi-Fi country, SSID, and Wi-Fi password;
   - the correct time zone and keyboard layout;
   - **Enable SSH** with **public-key authentication only**;
   - paste the laptop's public key from `~/.ssh/id_ed25519.pub`;
   - do not enable password-based SSH.
7. Write the image, wait for verification to finish, safely eject the card,
   and insert it into the powered-off Pi.

If the Imager version does not offer these settings, write the image first,
then use its documented first-boot customization method. Do not create
`ssh`/Wi-Fi files by guessing an obsolete format.

### 3. Boot and find the Pi

1. Disconnect every robot peripheral and insert the prepared card.
2. Power the Pi and wait at least two minutes for its first boot.
3. From the laptop, try mDNS:

   ```bash
   ssh robot@robot-pi.local
   ```

4. If `.local` does not resolve, find the Pi's DHCP lease in the router, or
   use the laptop's local-network discovery tools. Do not expose the Pi
   directly to the Internet or forward port 22 from the router.
5. On the first connection, verify the host fingerprint. If it unexpectedly
   changes later, stop and investigate instead of accepting it blindly.

### 4. Update the system before installing robot software

Use the repository's `raspberry-setup/` scripts from the Linux laptop. These
scripts are the supported path for this task; do not copy commands from old
setup files in `files/archive/maybe-obsolete/`.

1. Copy `raspberry-setup/config.example` to the untracked file
   `raspberry-setup/config` and set `PI_HOST`, `PI_USER`, and `SSH_KEY` for the
   Pi created in Raspberry Pi Imager.
2. Read `raspberry-setup/README.md` and run the laptop setup script with the
   `update` stage. This checks key-based SSH, copies the Pi-side update script
   to a temporary directory, runs it with `sudo`, and removes the temporary
   files afterward.
3. If the update stage reports that a reboot is required, reboot the Pi and
   wait for SSH to become available again before continuing.
4. Record the hostname, OS release, kernel, IP address, and microSD capacity in
   `phase_0.md`. Do not record Wi-Fi passwords or private SSH keys.

### 5. Apply the baseline security configuration

1. Run the laptop setup script with the `secure` stage. It verifies that the
   current SSH user has an authorized key, installs UFW and fail2ban, writes
   the SSH hardening drop-in, validates the SSH configuration before reload,
   and permits SSH only from the laptop address used by the current
   connection.
2. Keep the SSH session used by the script open. Start a second key-based SSH
   session from the laptop and verify that it works before closing the first.
3. If the laptop has multiple network addresses or the script cannot identify
   the correct source address, use its documented `--allow-ssh-from` option
   with the laptop's LAN address. Do not allow SSH from the whole Internet.
4. The script intentionally does not disable arbitrary services or open the
   robot API port. Review listening services separately, and add narrowly
   scoped ports only when a later task requires them.

The `raspberry-setup/README.md` also documents the alternative Pi-side
workflow: copy the two Pi scripts over SSH and run them directly. Use that
workflow only if the laptop orchestrator cannot be used; it has the same
update and security stages.

### 6. Establish the task acceptance record

Task 1 is complete only when:

- the Pi boots without a monitor or keyboard;
- SSH works from the Linux laptop using the `robot` user's Ed25519 key;
- password SSH and root SSH are disabled;
- the system is fully updated and reboots cleanly;
- the firewall allows SSH only from the laptop;
- fail2ban and unattended security updates are enabled;
- the OS, kernel, hostname, IP, and storage details are recorded in
  `phase_0.md`;
- no motor, L298N, Arduino, or motor-battery wiring has been connected.

If SSH access is lost, do not disable security controls blindly. Power the Pi
off, remove the card, and use the Linux laptop to inspect or repair the
configuration only after identifying the exact cause.

## Second task: physical L298N record (15–30 minutes)

1. Put the L298N on a non-conductive surface with no power connected.
2. Photograph both sides and the terminal labels.
3. Photograph the `5V-EN` jumper close-up.
4. Transcribe the labels exactly into `phase_0.md`.

## Third task: temporary battery record (10–20 minutes)

1. Identify whether the six cells are alkaline, NiMH, or another chemistry.
2. Confirm holder polarity without connecting it to electronics.
3. Measure and record open-circuit voltage.
4. Photograph the holder and its wires.

## Fourth task: minimum test readiness (15–30 minutes)

Confirm that the following are available: Pi 3 B v1.2, Pi USB supply/power
bank, L298N, one motor, temporary battery, multimeter, insulated wires,
breadboard, and a safe lifted-wheel arrangement. Mark missing items in
`phase_0.md`. The breadboard is for later low-current logic and sensor
prototyping; do not use it for motor or unfused battery current.

## Fifth task: Pi-only robot software bring-up (30–120 minutes)

Using the secured Pi from Task 1, power it independently from motor hardware.
Install and verify only the software needed for the first Pi-side test.
Confirm the OS, Wi-Fi, SSH access, and GPIO tooling. Do not connect motor
power or Pi GPIO wires to the L298N until its physical labels and jumper state
are recorded.

## Rule

Do not start a later task if an earlier task reveals an unknown polarity,
voltage path, or damaged component. Record the blocker and update the active
documents first. Whenever a component is identified, update the reproducibility
bill of materials in `phase_0-done.md` with its exact model and ratings.
