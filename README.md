# Mini Drone Project

This is a Rust-based embedded project for controlling a mini-drone, targeting an STM32F401 microcontroller. The project uses the RTIC (Real-Time Interrupt-driven Concurrency) framework for embedded programming.

## Project Structure

```
.
├── .vscode/                    # VS Code configuration files
├── documentation/              # Project documentation
│   └── dff_200276_whitepaper_motorcalculation_fin.pdf
├── mini-drone/                 # Main project directory
│   ├── cross/                  # Cross-compilation workspace
│   │   ├── application/        # Main application code
│   │   │   └── src/
│   │   │       ├── bin/
│   │   │       │   └── mini-drone.rs  # Main binary
│   │   │       └── lib.rs       # Library code
│   │   └── Cargo.toml          # Workspace configuration
│   ├── Cargo.toml              # Empty workspace config (references cross directory)
│   ├── memory.x                # Memory layout for STM32F401
│   └── rust-toolchain.toml     # Rust toolchain configuration
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

## Building

To build the project, use the following command:
```bash
cargo build
```

For optimized release builds:
```bash
cargo build --release
```

## Hardware

This project is designed for the STM32F401 microcontroller with:
- 128KB Flash memory
- 32KB RAM

## Debugging

Debugging is supported via RTT (Real-Time Transfer) using `defmt` for logging output.

## License

This project is licensed under the MIT License - see the LICENSE file for details.