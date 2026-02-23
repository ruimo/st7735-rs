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
st7735-rs = "0.1.9"
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

### Drawing with Functions

[examples/draw_rect_function.rs](examples/draw_rect_function.rs)

Demonstrates how to use `draw_rect` with a function to create dynamic pixel patterns based on coordinates.

```bash
cargo run --example draw_rect_function
```

Example output:
```
=== draw_rect Function Example ===

1. Gradient Pattern (2x2):
   Generated 8 bytes
   Bytes: [00, 00, 08, 41, 08, 41, 10, 82]

2. Checkerboard Pattern (4x4):
   Generated 32 bytes for 16 pixels

3. Horizontal Gradient (8x1):
   Generated 16 bytes

4. Circular Pattern (10x10):
   Generated 200 bytes for 100 pixels

=== Key Benefits ===
✓ No memory allocation for pixel data
✓ Pixels computed on-demand as bytes are consumed
✓ Efficient for large displays or complex patterns
✓ Function composition enables flexible pixel generation
```

## Text Drawing

### Drawing Characters

The library provides two functions for drawing text characters:

#### `draw_char` - Standard 8x8 Character

Draws a character at its original 8x8 pixel size:

```rust
use st7735_rs::command::text::draw_char;
use st7735_rs::color_format::{Pixel, Pixel16};

// Draw '5' in white on black background (8x8 pixels)
if let Some(ramwr) = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK) {
    // Send the command via SPI
}
```

#### `draw_char_scaled` - Scaled Character

Draws a character with specified horizontal and vertical scale factors:

```rust
use st7735_rs::command::text::draw_char_scaled;
use st7735_rs::color_format::{Pixel, Pixel16};

// Draw '5' at 2x scale (16x16 pixels)
if let Some(ramwr) = draw_char_scaled(
    '5',
    Pixel::<Pixel16>::WHITE,
    Pixel::<Pixel16>::BLACK,
    2,  // x scale
    2   // y scale
) {
    // Send the command via SPI
}

// Draw '5' at 3x2 scale (24x16 pixels - wider but not taller)
if let Some(ramwr) = draw_char_scaled(
    '5',
    Pixel::<Pixel16>::WHITE,
    Pixel::<Pixel16>::BLACK,
    3,  // x scale
    2   // y scale
) {
    // Send the command via SPI
}
```

**Important**: Before calling either function, you must set the drawing area using `Caset` and `Raset` commands:
- For `draw_char`: Set an 8x8 pixel region
- For `draw_char_scaled`: Set a (8×scale_x) × (8×scale_y) pixel region

### Customizing Available Characters

You can specify which characters to include in the font data by setting the `ST7735_FONT_PATH` environment variable to point to a file containing the desired characters.

**Default Characters**: If `ST7735_FONT_PATH` is not specified, the following characters are included by default:
```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ
```

Create a text file with the characters you want to display:

```bash
# Create a file with your desired characters
echo "0123456789ABCDEFabcdef" > characters.txt
```

#### Method 1: Using Environment Variable (Full Path Required)

When setting the environment variable directly, you must use an **absolute path**:

```bash
# Use full path
export ST7735_FONT_PATH=/home/user/project/characters.txt
cargo build
```

**Note**: Relative paths will not work when setting the environment variable directly.

#### Method 2: Using .cargo/config.toml (Relative Path Supported)

To use relative paths from your project directory, configure `.cargo/config.toml`:

```toml
[env]
ST7735_FONT_PATH = { value = "characters.txt", relative = true }
```

With `relative = true`, the path is resolved relative to your project root directory, making it easier to share the configuration across different environments and team members.

### Character Set Configuration

This configuration:
- Reduces binary size by including only the characters you need
- Characters are automatically sorted and deduplicated during build
- Uses the `font8x8` crate's BASIC_FONTS for character bitmaps
- Bitmaps are automatically bit-reversed for correct display orientation

Example character sets:

```
# Numbers only
0123456789

# Hexadecimal digits
0123456789ABCDEFabcdef

# Alphanumeric
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
```

**Note**: The font generation happens at build time via `build.rs`, so you need to rebuild your project after changing the character file.

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

Both `fill_rect()` and `draw_rect()` generate pixel data using iterators without allocating large buffers:

- `fill_rect()`: Fills a rectangular area with a single color
- `draw_rect()`: Generates pixels dynamically using a function that takes (x, y) coordinates

This approach is particularly efficient for large displays or complex patterns.

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

## Display Control Commands

### Memory Access Control (MADCTL)

The `Madctl` command controls the display orientation and color order:

```rust
use st7735_rs::command::{Madctl, AddressOrder, RgbBgb};

// Portrait mode (0° rotation)
let madctl = Madctl {
    my: AddressOrder::Normal,
    mx: AddressOrder::Normal,
    exchange_row_col: false,
    ml: VerticalRefreshOrder::TopToBottom,
    rgb_bgr: RgbBgb::Rgb,
    mh: HorizontalRefreshOrder::LeftToRight,
    ..Default::default()
};

// Landscape mode (90° rotation)
let madctl = Madctl {
    my: AddressOrder::Normal,
    mx: AddressOrder::Reverse,
    exchange_row_col: true,
    ..Default::default()
};
```

Available rotation configurations:
- **0°**: Portrait mode (default)
- **90°**: Landscape mode (clockwise)
- **180°**: Portrait mode (upside down)
- **270°**: Landscape mode (counter-clockwise)

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