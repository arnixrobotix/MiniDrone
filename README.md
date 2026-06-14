# Mini Drone Project

This is a Rust-based embedded project for controlling a mini-drone, targeting an STM32F401 microcontroller. The project uses the RTIC (Real-Time Interrupt-driven Concurrency) framework for embedded programming.

## Project Structure

```
.
├── .vscode/                        # VS Code configuration files
├── documentation/                  # Project documentation
├── luckfox/                        # Luckfox Pico Mini B flash images
│   └── Pico_Mini_B_Flash_images/   # Flash image files for Luckfox hardware
└── mini-drone/                     # Main project directory
    ├── crontroller/                # Controller code
    ├── cross/                      # Cross-compilation workspace
    │   ├── application/            # Main application code
    │   ├── flight-controller-unit/ # Flight controller unit code
    │   ├── Cargo.toml              # Target workspace configuration
    │   └── memory.x                # Memory layout for STM32F401
    ├── xtask/                      # Custom build/tests Cargo extension
    |── Cargo.toml                  # Host workspace configuration
    └── rust-toolchain.toml         # Rust toolchain configuration
```

## Dependencies

The project uses the following key dependencies:
- `cortex-m` - Core for ARM Cortex-M microcontrollers
- `defmt` - Low-level logging framework for embedded systems
- `defmt-brtt` - RTT support for defmt
- `panic-probe` - Panic handler
- `rtic` - Real-Time Interrupt-driven Concurrency framework
- `stm32f4xx-hal` - Hardware abstraction layer for STM32F4 series microcontrollers

## Features

- Real-time scheduling with RTIC framework
- RTT (Real-Time Transfer) debugging support via defmt
- STM32F401 microcontroller support
- Optimized build profiles for embedded development

## Setting up build environment

To build the project the following target needs to be added to Cargo:
```bash
rustup target add thumbv7em-none-eabihf
```

In addition `Flip-Link` and `probe-rs` needs to be installed:
```bash
cargo install flip-link
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Flashing

To flash the firmware on the microcontroller, use the following command:
```bash
cd mini-drone
cargo xtask flash
```

## Testing

To test the firmware on the host, use the following command:
```bash
cd mini-drone
cargo xtask test host
```

To test the firmware on the mini-drone itself (self-tests), use the following command:
```bash
cd mini-drone
cargo xtask test target
```

## Hardware

This project is designed for the STM32F401 microcontroller with:
- 128KB Flash memory
- 32KB RAM

## Debugging

Debugging is supported via RTT (Real-Time Transfer) using `defmt` for logging output.

## License

This project is licensed under the MIT License - see the LICENSE file for details.