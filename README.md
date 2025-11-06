# Overview

This is FT6336U driver implementation developed to be used in userspace. The driver communicates with FT6336U via `/dev/i2c-N` I2C userspace interface. Events from chip are sending to uinput subsystem (use `/dev/uinput` device).

The driver has hardcoded devices: `/dev/i2c-4` and `/dev/uinput`

## Building and installing

Use `make` to build the project:

1. Build - `make build`
    Compile driver in release configuration.

2. Install - `sudo make install`
    - Copy driver bin to `/usr/local/bin/`
    - Create systemd unit in `/etc/systemd/system`
    - Enable and launch driver systemd service

## Uninstalling

`sudo make uninstall`
