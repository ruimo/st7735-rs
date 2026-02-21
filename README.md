# ST7735 Display Driver

A no_std Rust driver library for the ST7735 TFT LCD display controller

[![Crates.io](https://img.shields.io/crates/v/st7735-rs.svg)](https://crates.io/crates/st7735-rs)
[![Documentation](https://docs.rs/st7735-rs/badge.svg)](https://docs.rs/st7735-rs)
[![License](https://img.shields.io/crates/l/st7735-rs.svg)](https://github.com/ruimo/st7735-rs/blob/main/LICENSE)

## Features

- **no_std support**: Can be used in embedded systems
- **Type-safe color formats**: Compile-time guarantees for 12-bit, 16-bit, and 18-bit color modes
- **Flexible range specification**: Supports Rust's standard range syntax
- **Zero-cost abstractions**: Type safety without runtime overhead
- **Comprehensive testing**: Validates behavior of each command and color format

## Supported Color Formats

| Format | Bit Layout | Bytes/Pixel | Description |
|--------|-----------|-------------|-------------|
| 12-bit | RGB 4:4:4 | 1.5 (3 bytes for 2 pixels) | Low memory usage |
| 16-bit | RGB 5:6:5 | 2 | Most common, well-balanced |
| 18-bit | RGB 6:6:6 | 3 | Highest quality |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
st7735-rs = "0.1.0"
```

## Usage Examples

The `examples/` directory contains runnable sample code demonstrating various features of this library.

### Basic Initialization

[examples/basic_initialization.rs](examples/basic_initialization.rs)

Demonstrates the basic initialization sequence for the display.

```bash
cargo run --example basic_initialization
```

Example output:
```
Slpout command byte: 0x11
Slpout post delay: Some(120ms)
Colmod command byte: 0x3A
Colmod parameter bytes: [5]
Dispon command byte: 0x29
Dispon post delay: Some(100ms)

Basic initialization example completed successfully!
```

### Drawing a Rectangle

[examples/drawing_rectangle.rs](examples/drawing_rectangle.rs)

Shows how to specify a rectangular area and fill it with a color.

```bash
cargo run --example drawing_rectangle
```

Example output:
```
=== Caset (Column Address Set) ===
Command byte: 0x2A
Parameter bytes: [0, 0, 0, 9]
Post delay: None

=== Raset (Row Address Set) ===
Command byte: 0x2B
Parameter bytes: [0, 0, 0, 9]
Post delay: None

=== Ramwr (Memory Write) ===
Command byte: 0x2C
Parameter bytes count: 200 bytes
First 20 bytes: [F8, 00, F8, 00, F8, 00, F8, 00, F8, 00, F8, 00, F8, 00, F8, 00, F8, 00, F8, 00]
Post delay: None

Drawing rectangle example completed successfully!
```

### Using Custom Colors

[examples/custom_colors.rs](examples/custom_colors.rs)

Demonstrates how to use predefined colors and create custom colors.

```bash
cargo run --example custom_colors
```

Example output:
```
=== Predefined Colors ===
RED: R=31, G=0, B=0
GREEN: R=0, G=63, B=0
BLUE: R=0, G=0, B=31
WHITE: R=31, G=63, B=31
BLACK: R=0, G=0, B=0

=== Custom Color ===
CUSTOM: R=20, G=40, B=15

Custom colors example completed successfully!
```

### Different Color Formats

[examples/different_color_formats.rs](examples/different_color_formats.rs)

Shows the differences between 12-bit, 16-bit, and 18-bit color formats.

```bash
cargo run --example different_color_formats
```

Example output:
```
=== 12-bit Color Format (RGB 4:4:4) ===
RED: R=15, G=0, B=0 (max: 15)
GREEN: R=0, G=15, B=0 (max: 15)
BLUE: R=0, G=0, B=15 (max: 15)

=== 16-bit Color Format (RGB 5:6:5) ===
RED: R=31, G=0, B=0 (R,B max: 31, G max: 63)
GREEN: R=0, G=63, B=0 (R,B max: 31, G max: 63)
BLUE: R=0, G=0, B=31 (R,B max: 31, G max: 63)

=== 18-bit Color Format (RGB 6:6:6) ===
RED: R=63, G=0, B=0 (max: 63)
GREEN: R=0, G=63, B=0 (max: 63)
BLUE: R=0, G=0, B=63 (max: 63)

Different color formats example completed successfully!
```

## Architecture

### Type Safety

This library leverages Rust's type system to guarantee color format correctness at compile time:

```rust
// Types are determined at compile time
let pixel16 = Pixel::<Pixel16>::RED;
let pixel18 = Pixel::<Pixel18>::RED;

// Cannot mix different types (compile error)
// let mixed = if condition { pixel16 } else { pixel18 };
```

### Zero-Cost Abstractions

By using `PhantomData`, type information exists only at compile time with no runtime memory overhead.

### Efficient Memory Usage

`fill_rect()` generates pixel data using iterators without allocating large buffers.

## Hardware Connection

ST7735 displays are typically connected via SPI interface:

| Pin | Description |
|-----|-------------|
| VCC | Power supply (3.3V or 5V) |
| GND | Ground |
| SCL | SPI clock |
| SDA | SPI data (MOSI) |
| RES | Reset |
| DC(A0)  | Data/Command select |
| CS  | Chip select |
| BL  | Backlight (optional) |

## SPI Communication Pattern

The typical pattern for sending commands is:

1. Set CS pin LOW.
1. Set DC pin LOW (command mode)
1. Send command byte via SPI
1. Set DC pin HIGH (data mode)
1. Send parameter bytes via SPI
1. Set CS pin HIGH.
1. Wait for required delay (if any)

Example pseudo-code:
```rust
// Send command
cs.set_low();
dc.set_low();
spi.write(&[command.cmd_byte()]).unwrap();
cs.set_high();

// Send parameters
cs.set_low();
dc.set_high();
for byte in command.parm_bytes() {
    spi.write(&[byte]).unwrap();
}
cs.set_high();

// Wait if needed
delay(command.post_delay());
```

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for:
- Command byte generation
- Parameter byte generation
- Color format conversions
- Range handling
- Pixel data generation

## License

See [LICENSE](https://github.com/ruimo/st7735-rs/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Before submitting a pull request, please ensure:

1. Code is formatted (`cargo fmt`)
2. All tests pass (`cargo test`)
3. New features include tests

## References

- [ST7735 Datasheet](https://www.displayfuture.com/Display/datasheet/controller/ST7735.pdf)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

## Author

Shisei Hanai <ruimo.uno@gmail.com>