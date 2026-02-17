# ST7735 Display Driver

A no_std Rust driver library for the ST7735 TFT LCD display controller

[![Crates.io](https://img.shields.io/crates/v/st7735-rs.svg)](https://crates.io/crates/st7735-rs)
[![Documentation](https://docs.rs/st7735-rs/badge.svg)](https://docs.rs/st7735-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

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

### Basic Initialization

```rust
use st7735_rs::color_format::{Pixel, Pixel16, ColorFormat};
use st7735_rs::command::{Command, Slpout, Dispon, Colmod, Caset, Raset, Ramwr};

// 1. Sleep out
let slpout = Slpout;
// spi.write(slpout.cmd_byte()).unwrap();
// delay(slpout.post_delay());

// 2. Set color mode (16-bit)
let colmod = Colmod::new(ColorFormat::Bit16);
// spi.write(colmod.cmd_byte()).unwrap();
// spi.write(colmod.parm_bytes()).unwrap();

// 3. Display ON
let dispon = Dispon;
// spi.write(dispon.cmd_byte()).unwrap();
// delay(dispon.post_delay());
```

### Drawing a Rectangle

```rust
use st7735_rs::color_format::{Pixel, Pixel16};
use st7735_rs::command::{Caset, Raset, Ramwr};

// Set drawing area (X: 0-9, Y: 0-9)
let caset = Caset::new(0..10);
let raset = Raset::new(0..10);

// Fill a 10x10 rectangle with red
let ramwr = Ramwr::fill_rect(0..=9, 0..=9, Pixel::<Pixel16>::RED);

// Send commands and parameters
// spi.write(caset.cmd_byte()).unwrap();
// spi.write(caset.parm_bytes()).unwrap();
// spi.write(raset.cmd_byte()).unwrap();
// spi.write(raset.parm_bytes()).unwrap();
// spi.write(ramwr.cmd_byte()).unwrap();
// spi.write(ramwr.parm_bytes()).unwrap();
```

### Using Custom Colors

```rust
use st7735_rs::color_format::{Pixel, Pixel16};

// Predefined colors
let red = Pixel::<Pixel16>::RED;
let green = Pixel::<Pixel16>::GREEN;
let blue = Pixel::<Pixel16>::BLUE;
let white = Pixel::<Pixel16>::WHITE;
let black = Pixel::<Pixel16>::BLACK;

// Custom color (16-bit: R=20, G=40, B=15)
let custom = Pixel::<Pixel16>::new(20, 40, 15);
```

### Different Color Formats

```rust
use st7735_rs::color_format::{Pixel, Pixel12, Pixel16, Pixel18};

// 12-bit mode (R,G,B: 0-15)
let red12 = Pixel::<Pixel12>::RED;  // (15, 0, 0)

// 16-bit mode (R,B: 0-31, G: 0-63)
let red16 = Pixel::<Pixel16>::RED;  // (31, 0, 0)

// 18-bit mode (R,G,B: 0-63)
let red18 = Pixel::<Pixel18>::RED;  // (63, 0, 0)
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
1. Wait for required delay (if any)
1. Set CS pin HIGH.

Example pseudo-code:
```rust
// Send command
cs.set_low();
dc.set_low();
spi.write(&[command.cmd_byte()]).unwrap();

// Send parameters
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

See [LICENSE](LICENSE) file for details.

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