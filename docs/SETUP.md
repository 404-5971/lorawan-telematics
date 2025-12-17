# Setup and Installation

## 1. Install Rust and System Dependencies

Ensure you have Rust installed (stable):

```bash
sudo pacman -S rustup
```

```bash
rustup install stable
```

Make sure $HOME/.cargo/bin is in your PATH. I.e. add it to your .bashrc or .zshrc:

```bash
export PATH=$PATH:$HOME/.cargo/bin
```

## 2. Install Espressif Toolchain

Install `espup`, the tool for managing the Espressif Rust ecosystem:

```bash
cargo install espup --locked
```

Then, install the specific toolchain version used in this project:

```bash
espup install --toolchain-version 1.90.0.0 --name esp
```

**Important:** After installation, you must source the environment variables. `espup` generates a file (usually `~/export-esp.sh`). Source it in your shell:

```bash
. ~/export-esp.sh
```

_Tip: You may want to add this command to your shell profile (e.g., `.bashrc` or `.zshrc`) so it loads automatically._

## 3. Install Build Tools

This project uses `ldproxy` and `espflash` for flashing and monitoring:

```bash
cargo install ldproxy espflash --locked
```

## 4. Build and Flash

Connect your Heltec Tracker via USB and run:

```bash
cargo run --release
```

This command will compile the project, flash it to the device, and start the serial monitor using `espflash`.

## Monitoring Output

For watching device output, I'm using [tio](https://aur.archlinux.org/packages/tio). It isn't that great and I'll likely be switching tools in the future, but it is useful for staying connected and reconnecting automatically between device resets.

To determine which serial port your device is connected to, run:

```bash
tio --list
```

Once you've identified the correct port (e.g., `/dev/ttyACM0` or `/dev/ttyUSB0`), use it to monitor the output:

```bash
tio <device_port>
```

## Editor Configuration
