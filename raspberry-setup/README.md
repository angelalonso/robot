# Raspberry Pi setup scripts

These scripts automate the parts of Task 1 that are safe to perform after
Raspberry Pi Imager has written the OS and the Pi is reachable over SSH.

They do **not** write the microSD card, configure Wi-Fi credentials, or connect
to any motor hardware. Configure the image in Raspberry Pi Imager first:

- Raspberry Pi OS Lite;
- a non-`pi` user, for example `robot`;
- the Wi-Fi network and country;
- SSH enabled with the laptop's Ed25519 public key;
- a unique hostname, for example `robot-pi`.

## Laptop-side use

From the repository root, copy the example configuration and edit it:

```bash
cp raspberry-setup/config.example raspberry-setup/config
$EDITOR raspberry-setup/config
```

The configuration file is intentionally ignored by Git. It contains no
password fields; SSH uses the key already configured in Raspberry Pi Imager.

Then run:

```bash
./raspberry-setup/from-laptop/setup.sh
```

The script checks key-based SSH access, copies the Pi-side scripts to a
temporary directory, runs the update and hardening steps, and removes the
temporary directory. It uses `ssh -t` because `sudo` may need to ask for the
initial user password.

Run individual stages when needed:

```bash
./raspberry-setup/from-laptop/setup.sh --stage update
./raspberry-setup/from-laptop/setup.sh --stage secure
```

Use `--dry-run` to inspect the SSH/SCP commands without connecting:

```bash
./raspberry-setup/from-laptop/setup.sh --dry-run
```

## Pi-side use

The scripts can also be copied and run directly:

```bash
scp raspberry-setup/pi/*.sh robot@robot-pi.local:/tmp/
ssh -t robot@robot-pi.local 'bash /tmp/01-update.sh'
ssh -t robot@robot-pi.local 'bash /tmp/02-secure.sh'
```

`02-secure.sh` allows SSH only from the address that opened the current SSH
connection, unless `--allow-ssh-from ADDRESS` is supplied. Keep the current
SSH session open and verify a second key-based session before closing it.

The scripts are intended to be rerunnable. Review their output and record the
hostname, OS release, kernel, IP address, and storage details in
`docs/phase_0.md`. Never place Wi-Fi passwords or private SSH keys in this
directory.
