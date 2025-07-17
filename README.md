# MouseMover-RS

A Rust application that moves the mouse cursor to random positions on the screen with a configurable delay.

## Dependencies

### Linux

On Linux, this application depends on the `libxdo` library, which is part of the `xdotool` package. You need to install it before building the application:

#### Debian/Ubuntu
```bash
sudo apt-get install libxdo-dev
```

#### Fedora
```bash
sudo dnf install libxdo-devel
```

#### Arch Linux
```bash
sudo pacman -S xdotool
```

## Usage

Build and run the application:

```bash
cargo build
cargo run
```

### Command-line Options

- `--delay-ms <MILLISECONDS>`: Set the delay between mouse movements in milliseconds (default: 1000)
- `--help`: Display help information about the application and its options

Example:
```bash
cargo run -- --delay-ms 2000
```

This will move the mouse cursor to random positions with a 2-second delay between movements.

### Hotkeys

While the application is running, you can use the following hotkey:

- `CTRL + ALT + C`: Stop the application and exit
