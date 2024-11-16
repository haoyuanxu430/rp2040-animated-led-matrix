# RP2040 LED Matrix Tilt Animation

This project demonstrates using Rust on the Adafruit Feather RP2040 to create an interactive LED matrix display that responds to board orientation. The project uses the onboard LIS3DH accelerometer to detect tilt and displays different animations on an 8x8 WS2812 LED matrix based on the tilt direction.

## Features
- Four unique animations corresponding to different tilt directions
- Real-time accelerometer data reading
- Smooth LED matrix control
- USB serial debug output

## Hardware Requirements
- Adafruit Feather RP2040 board
- 8x8 WS2812 LED Matrix
- USB cable for programming and power

## Software Setup

### First Time Setup
1. Install Rustup (Rust toolchain manager)
   - Windows: Download and run [rustup-init.exe](https://rustup.rs/)
   - Unix-like: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Set up the embedded Rust toolchain:
   ```bash
   # Set default toolchain to stable
   rustup default stable

   # Add the ARM Cortex-M0+ target (used by RP2040)
   rustup target add thumbv6m-none-eabi

   # Install elf2uf2-rs conversion tool
   cargo install elf2uf2-rs
   ```

3. Install USB driver (if needed)
   - Windows may require the WinUSB driver
   - Linux may need udev rules for USB access

### Building and Running

For users with Rust already set up:
```bash
# Clone the repository
git clone https://github.com/haoyuanxu430/rp2040-animated-led-matrix
cd rp2040-animated-led-matrix

# Build and run the project
cargo run
```

For first-time users:
1. Connect your Feather RP2040 via USB
2. Press the BOOTSEL button while plugging in the board to enter bootloader mode
3. The board should appear as a mass storage device
4. Run `cargo run` to build and upload the program

## Project Structure
```
.
├── Cargo.toml              # Project dependencies and configuration
├── .cargo/
│   └── config             # Cargo configuration for RP2040
├── src/
│   ├── main.rs            # Main program logic and initialization
│   ├── animations.rs      # LED matrix animation implementations
│   ├── lis3dh.rs         # Accelerometer driver
│   └── usb_manager.rs     # USB communication handling
```

## Animations
- **Right Tilt**: Vertical green line bouncing left to right
- **Left Tilt**: Purple wave pattern
- **Forward Tilt**: Blue expanding/contracting circle
- **Backward Tilt**: Yellow diagonal pattern

## Troubleshooting
- If cargo run fails with a USB error, try pressing the reset button on the board
- If the LED matrix doesn't light up, check the power and data pin connections
- For USB communication issues, ensure you have the correct drivers installed
- Monitor the USB serial output for debugging information

## Contributing
Feel free to submit issues and enhancement requests!

## License
MIT License