//! ST7735 command implementations
//!
//! This module provides type-safe implementations of ST7735 display controller commands.
//! Each command is represented as a struct that implements the [`Command`] trait.
//!
//! # Command Categories
//!
//! - **Display Control**: [`Dispon`], [`Slpout`]
//! - **Color Configuration**: [`Colmod`]
//! - **Drawing Area**: [`Caset`], [`Raset`]
//! - **Memory Write**: [`Ramwr`]
//!
//! # Usage Example
//!
//! ```rust
//! use st7735_rs::command::{Command, Slpout, Colmod, Dispon};
//! use st7735_rs::color_format::ColorFormat;
//! use core::time::Duration;
//!
//! // Sleep out command
//! let mut slpout = Slpout;
//! let cmd = slpout.cmd_byte(); // 0x11
//! let delay = slpout.post_delay(); // 120ms
//!
//! // Set color mode to 16-bit
//! let mut colmod = Colmod::new(ColorFormat::Bit16);
//! let params: Vec<u8> = colmod.parm_bytes().into_iter().collect();
//! ```

use core::time::Duration;
use crate::color_format::{ColorFormat, ColorFormatMarker, Pixel};
use core::ops::RangeBounds;

/// Trait for ST7735 commands
///
/// All ST7735 commands implement this trait, which provides:
/// - The command byte to send to the display
/// - Parameter bytes (if any) to follow the command
/// - Required delay after sending the command
pub trait Command {
  /// Returns the command byte
  fn cmd_byte(&self) -> u8;
  
  /// Returns an iterator over parameter bytes
  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8>;
  
  /// Returns the required delay after sending this command
  fn post_delay(&self) -> Duration;
}

/// Display ON command (0x29)
///
/// Turns on the display panel. This command should be sent after initialization
/// and sleep out commands.
///
/// # Timing
///
/// Requires a 120ms delay after execution.
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Dispon};
///
/// let mut dispon = Dispon;
/// assert_eq!(dispon.cmd_byte(), 0x29);
/// ```
pub struct Dispon;

impl Command for Dispon {
  fn cmd_byte(&self) -> u8 {
    0x29
  }

  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    [].into_iter()
  }

  fn post_delay(&self) -> Duration {
    Duration::from_millis(120)
  }
}

/// Interface Pixel Format command (0x3A)
///
/// Sets the color format for the display interface. Supports 12-bit, 16-bit,
/// and 18-bit color modes.
///
/// # Color Formats
///
/// - **12-bit (RGB 4:4:4)**: 2 pixels in 3 bytes
/// - **16-bit (RGB 5:6:5)**: 1 pixel in 2 bytes (most common)
/// - **18-bit (RGB 6:6:6)**: 1 pixel in 3 bytes (highest quality)
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Colmod};
/// use st7735_rs::color_format::ColorFormat;
///
/// let mut colmod = Colmod::new(ColorFormat::Bit16);
///
/// // Send command byte
/// let cmd = colmod.cmd_byte();
/// // spi.write(&[cmd]).unwrap();
///
/// // Send parameter bytes (assuming SPI HAL accepts IntoIterator<Item=u8>)
/// // spi.write(colmod.parm_bytes()).unwrap();
/// ```
pub struct Colmod {
  color_format: ColorFormat,
}

impl Colmod {
  /// Creates a new COLMOD command with the specified color format
  ///
  /// # Parameters
  ///
  /// * `color_format` - The desired color format ([`ColorFormat`])
  pub fn new(color_format: ColorFormat) -> Self {
    Self { color_format }
  }
}

impl Command for Colmod {
  fn cmd_byte(&self) -> u8 {
    0x3A
  }
  
  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    match self.color_format {
      ColorFormat::Bit12 => [0b0000_0011],
      ColorFormat::Bit16 => [0b0000_0101],
      ColorFormat::Bit18 => [0b0000_0110],
    }.into_iter()
  }

  fn post_delay(&self) -> Duration {
      Duration::from_millis(0)
  }
}

/// Sleep Out command (0x11)
///
/// Exits sleep mode. The display must wait at least 120ms after this command
/// before sending other commands.
///
/// # Timing
///
/// Requires a 120ms delay after execution.
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Slpout};
///
/// let mut slpout = Slpout;
/// assert_eq!(slpout.cmd_byte(), 0x11);
/// ```
pub struct Slpout;

impl Command for Slpout {
  fn cmd_byte(&self) -> u8 {
    0x11
  }

  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    [].into_iter()
  }
  
  fn post_delay(&self) -> Duration {
        Duration::from_millis(120)
    }
}

/// Internal structure for range-based commands
///
/// This is used internally by [`Caset`] and [`Raset`] to handle column and row
/// address setting with flexible range syntax.
struct RangeSetCommand<R: RangeBounds<u16>> {
  cmd_byte: u8,
  range: R,
}

impl<R: RangeBounds<u16>> RangeSetCommand<R> {
  fn new(cmd_byte: u8, range: R) -> Self {
    Self { cmd_byte, range }
  }
}

impl<R: RangeBounds<u16>> Command for RangeSetCommand<R> {
  fn cmd_byte(&self) -> u8 {
    self.cmd_byte
  }
 
  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    let start = range_start(&self.range);
    let end = range_end(&self.range);
    
    [
      (start >> 8) as u8,
      start as u8,
      (end >> 8) as u8,
      end as u8,
    ].into_iter()
  }
  
  fn post_delay(&self) -> Duration {
    Duration::from_millis(0)
  }
}

/// Helper function to extract the start value from a range
fn range_start<R: RangeBounds<u16>>(range: &R) -> u16 {
  use core::ops::Bound;
  match range.start_bound() {
    Bound::Included(&n) => n,
    Bound::Excluded(&n) => n.saturating_add(1),
    Bound::Unbounded => 0,
  }
}

/// Helper function to extract the end value from a range
fn range_end<R: RangeBounds<u16>>(range: &R) -> u16 {
  use core::ops::Bound;
  match range.end_bound() {
    Bound::Included(&n) => n,
    Bound::Excluded(&n) => n.saturating_sub(1),
    Bound::Unbounded => u16::MAX,
  }
}

/// Column Address Set command (0x2A)
///
/// Sets the column (X) address range for subsequent memory write operations.
/// Supports flexible Rust range syntax.
///
/// # Range Syntax
///
/// - `0..=127` - Inclusive range from 0 to 127
/// - `10..100` - Exclusive end, from 10 to 99
/// - `50..` - From 50 to maximum (65535)
/// - `..=200` - From 0 to 200
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Caset};
///
/// // Set columns 0 to 127
/// let mut caset = Caset::new(0..=127);
/// let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
/// assert_eq!(params, vec![0x00, 0x00, 0x00, 0x7F]);
/// ```
pub struct Caset<RX: RangeBounds<u16>> {
  inner: RangeSetCommand<RX>,
}

impl<RX: RangeBounds<u16>> Caset<RX> {
  /// Creates a new CASET command with the specified column range
  ///
  /// # Parameters
  ///
  /// * `x_range` - Column address range (supports any [`RangeBounds<u16>`])
  pub fn new(x_range: RX) -> Self {
    Self {
      inner: RangeSetCommand::new(0x2A, x_range)
    }
  }
}

impl<RX: RangeBounds<u16>> Command for Caset<RX> {
  fn cmd_byte(&self) -> u8 {
    self.inner.cmd_byte()
  }
 
  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    self.inner.parm_bytes()
  }
  
  fn post_delay(&self) -> Duration {
    self.inner.post_delay()
  }
}

/// Row Address Set command (0x2B)
///
/// Sets the row (Y) address range for subsequent memory write operations.
/// Supports flexible Rust range syntax.
///
/// # Range Syntax
///
/// - `0..=159` - Inclusive range from 0 to 159
/// - `10..100` - Exclusive end, from 10 to 99
/// - `50..` - From 50 to maximum (65535)
/// - `..=200` - From 0 to 200
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Raset};
///
/// // Set rows 0 to 159
/// let mut raset = Raset::new(0..=159);
/// let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
/// assert_eq!(params, vec![0x00, 0x00, 0x00, 0x9F]);
/// ```
pub struct Raset<RY: RangeBounds<u16>> {
  inner: RangeSetCommand<RY>,
}

impl<RY: RangeBounds<u16>> Raset<RY> {
  /// Creates a new RASET command with the specified row range
  ///
  /// # Parameters
  ///
  /// * `y_range` - Row address range (supports any [`RangeBounds<u16>`])
  pub fn new(y_range: RY) -> Self {
    Self {
      inner: RangeSetCommand::new(0x2B, y_range)
    }
  }
}

impl<RY: RangeBounds<u16>> Command for Raset<RY> {
  fn cmd_byte(&self) -> u8 {
    self.inner.cmd_byte()
  }
 
  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    self.inner.parm_bytes()
  }
  
  fn post_delay(&self) -> Duration {
    self.inner.post_delay()
  }
}

/// Memory Write command (0x2C)
///
/// Writes pixel data to the display memory. The data format depends on the
/// color mode set by [`Colmod`]. The write area is defined by previous
/// [`Caset`] and [`Raset`] commands.
///
/// # Generic Parameter
///
/// * `T` - An iterator that yields bytes to write to display memory
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, Ramwr};
/// use st7735_rs::color_format::{Pixel, Pixel16};
///
/// // Fill a 10x10 rectangle with red
/// let mut ramwr = Ramwr::fill_rect(0..=9, 0..=9, Pixel::<Pixel16>::RED);
/// assert_eq!(ramwr.cmd_byte(), 0x2C);
/// ```
pub struct Ramwr<T: IntoIterator<Item = u8>> {
  bytes: Option<T>,
}

impl<T: IntoIterator<Item = u8>> Command for Ramwr<T> {
    fn cmd_byte(&self) -> u8 {
        0x2c
    }

    fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
        self.bytes.take().expect("parm_bytes() can only be called once")
    }

    fn post_delay(&self) -> Duration {
        Duration::from_millis(0)
    }
}

/// Iterator for efficiently repeating pixel patterns
///
/// This iterator generates pixel data by repeating a pattern, which is more
/// memory-efficient than storing all pixel data in a buffer.
pub struct PixelIterator {
  pattern: [u8; 3],
  pattern_len: usize,
  total_bytes: usize,
  current_index: usize,
}

impl PixelIterator {
  fn new(pattern: [u8; 3], pattern_len: usize, total_bytes: usize) -> Self {
    Self {
      pattern,
      pattern_len,
      total_bytes,
      current_index: 0,
    }
  }
}

impl Iterator for PixelIterator {
  type Item = u8;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.current_index >= self.total_bytes {
      return None;
    }
    
    let byte = self.pattern[self.current_index % self.pattern_len];
    self.current_index += 1;
    Some(byte)
  }
}

impl Ramwr<PixelIterator> {
  /// Creates a RAMWR command to fill a rectangular area with a single color
  ///
  /// This method efficiently generates pixel data for filling a rectangle without
  /// allocating a large buffer. The pixel format is determined by the generic
  /// parameter `F`.
  ///
  /// # Type Parameters
  ///
  /// * `F` - Color format marker ([`Pixel12`], [`Pixel16`], or [`Pixel18`])
  ///
  /// # Parameters
  ///
  /// * `x_range` - Column range (X coordinates)
  /// * `y_range` - Row range (Y coordinates)
  /// * `pixel` - The pixel color to fill with
  ///
  /// # Panics
  ///
  /// Panics in 12-bit mode if the total pixel count is odd, as 12-bit mode
  /// requires an even number of pixels (2 pixels per 3 bytes).
  ///
  /// # Example
  ///
  /// ```
  /// use st7735_rs::command::Ramwr;
  /// use st7735_rs::color_format::{Pixel, Pixel16};
  ///
  /// // Fill a 128x160 screen with blue
  /// let mut ramwr = Ramwr::fill_rect(0..=127, 0..=159, Pixel::<Pixel16>::BLUE);
  /// ```
  pub fn fill_rect<F: ColorFormatMarker>(x_range: impl RangeBounds<u16>, y_range: impl RangeBounds<u16>, pixel: Pixel<F>) -> Self {
    let color_format = F::FORMAT;
    // Calculate pixel count from ranges
    let x_start = range_start(&x_range);
    let x_end = range_end(&x_range);
    let y_start = range_start(&y_range);
    let y_end = range_end(&y_range);
    
    let width = (x_end - x_start + 1) as usize;
    let height = (y_end - y_start + 1) as usize;
    let pixel_count = width * height;
    
    // Generate pixel pattern based on color format
    let (pattern, pattern_len, total_bytes) = match color_format {
      ColorFormat::Bit12 => {
        // 12-bit: RGB 4:4:4, 2 pixels in 3 bytes
        // Byte pattern: [R4G4, B4R4, G4B4] for two pixels
        
        // Panic if pixel count is odd
        if pixel_count % 2 == 1 {
          panic!("Pixel count must be even in 12-bit mode. Current pixel count: {}", pixel_count);
        }
        
        let r4 = pixel.r & 0x0F;
        let g4 = pixel.g & 0x0F;
        let b4 = pixel.b & 0x0F;
        
        let pattern = [
          (r4 << 4) | g4,  // Even pixel: R4G4
          (b4 << 4) | r4,  // Odd pixel: B4R4
          (g4 << 4) | b4,  // Odd pixel: G4B4
        ];
        
        (pattern, 3, (pixel_count * 3) / 2)
      },
      ColorFormat::Bit16 => {
        // 16-bit: RGB 5:6:5, 1 pixel in 2 bytes
        let r5 = (pixel.r & 0x1F) as u16;  // 5 bits
        let g6 = (pixel.g & 0x3F) as u16;  // 6 bits
        let b5 = (pixel.b & 0x1F) as u16;  // 5 bits
        
        let color16 = (r5 << 11) | (g6 << 5) | b5;
        let high = (color16 >> 8) as u8;
        let low = color16 as u8;
        
        let pattern = [high, low, 0];
        
        (pattern, 2, pixel_count * 2)
      },
      ColorFormat::Bit18 => {
        // 18-bit: RGB 6:6:6, 1 pixel in 3 bytes
        let r6 = pixel.r & 0x3F;  // 6 bits
        let g6 = pixel.g & 0x3F;  // 6 bits
        let b6 = pixel.b & 0x3F;  // 6 bits
        
        let pattern = [r6, g6, b6];
        
        (pattern, 3, pixel_count * 3)
      },
    };
    
    Self {
      bytes: Some(PixelIterator::new(pattern, pattern_len, total_bytes))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_colmod_cmd_byte() {
    let mut colmod = Colmod::new(ColorFormat::Bit16);
    assert_eq!(colmod.cmd_byte(), 0x3A);
  }

  #[test]
  fn test_colmod_bit12_parm_bytes() {
    let mut colmod = Colmod::new(ColorFormat::Bit12);
    let params: Vec<u8> = colmod.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0011]);
  }

  #[test]
  fn test_colmod_bit16_parm_bytes() {
    let mut colmod = Colmod::new(ColorFormat::Bit16);
    let params: Vec<u8> = colmod.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0101]);
  }

  #[test]
  fn test_colmod_bit18_parm_bytes() {
    let mut colmod = Colmod::new(ColorFormat::Bit18);
    let params: Vec<u8> = colmod.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0110]);
  }

  #[test]
  fn test_slpout_cmd_byte() {
    let mut slpout = Slpout;
    assert_eq!(slpout.cmd_byte(), 0x11);
  }

  #[test]
  fn test_slpout_parm_bytes() {
    let mut slpout = Slpout;
    let params: Vec<u8> = slpout.parm_bytes().into_iter().collect();
    assert_eq!(params, Vec::<u8>::new());
  }

  #[test]
  fn test_colmod_post_delay() {
    let mut colmod = Colmod::new(ColorFormat::Bit16);
    assert_eq!(colmod.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_slpout_post_delay() {
    let mut slpout = Slpout;
    assert_eq!(slpout.post_delay(), Duration::from_millis(120));
  }

  #[test]
  fn test_dispon_cmd_byte() {
    let mut dispon = Dispon;
    assert_eq!(dispon.cmd_byte(), 0x29);
  }

  #[test]
  fn test_dispon_parm_bytes() {
    let mut dispon = Dispon;
    let params: Vec<u8> = dispon.parm_bytes().into_iter().collect();
    assert_eq!(params, Vec::<u8>::new());
  }

  #[test]
  fn test_dispon_post_delay() {
    let mut dispon = Dispon;
    assert_eq!(dispon.post_delay(), Duration::from_millis(120));
  }

  #[test]
  fn test_caset_cmd_byte() {
    let mut caset = Caset::new(0..=128);
    assert_eq!(caset.cmd_byte(), 0x2A);
  }

  #[test]
  fn test_caset_parm_bytes_simple() {
    let mut caset = Caset::new(0..=128);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 0..=128 -> [0x00, 0x00, 0x00, 0x80]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x80]);
  }

  #[test]
  fn test_caset_parm_bytes_with_offset() {
    let mut caset = Caset::new(10..=100);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 10..=100 -> [0x00, 0x0A, 0x00, 0x64]
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x64]);
  }

  #[test]
  fn test_caset_parm_bytes_large_values() {
    let mut caset = Caset::new(256..=512);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 256..=512 -> [0x01, 0x00, 0x02, 0x00]
    assert_eq!(params, vec![0x01, 0x00, 0x02, 0x00]);
  }

  #[test]
  fn test_caset_post_delay() {
    let mut caset = Caset::new(0..=128);
    assert_eq!(caset.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_caset_exclusive_range() {
    let mut caset = Caset::new(10..100);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 10..100 (exclusive end) -> [0x00, 0x0A, 0x00, 0x63]
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x63]);
  }

  #[test]
  fn test_caset_range_from() {
    let mut caset = Caset::new(50..);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 50.. (unbounded end) -> [0x00, 0x32, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x32, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_range_to_inclusive() {
    let mut caset = Caset::new(..=200);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: ..=200 (unbounded start) -> [0x00, 0x00, 0x00, 0xC8]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC8]);
  }

  #[test]
  fn test_caset_range_to() {
    let mut caset = Caset::new(..150);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: ..150 (exclusive end) -> [0x00, 0x00, 0x00, 0x95]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x95]);
  }

  #[test]
  fn test_caset_full_range() {
    let mut caset = Caset::new(..);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: .. (fully unbounded) -> [0x00, 0x00, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_single_point() {
    let mut caset = Caset::new(42..=42);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 42..=42 (single point) -> [0x00, 0x2A, 0x00, 0x2A]
    assert_eq!(params, vec![0x00, 0x2A, 0x00, 0x2A]);
  }

  #[test]
  fn test_caset_max_value() {
    let mut caset = Caset::new(0..=u16::MAX);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 0..=65535 -> [0x00, 0x00, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_high_values() {
    let mut caset = Caset::new(1000..=2000);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    // x_range: 1000..=2000 -> [0x03, 0xE8, 0x07, 0xD0]
    assert_eq!(params, vec![0x03, 0xE8, 0x07, 0xD0]);
  }

  #[test]
  fn test_raset_cmd_byte() {
    let mut raset = Raset::new(0..=128);
    assert_eq!(raset.cmd_byte(), 0x2B);
  }

  #[test]
  fn test_raset_parm_bytes_simple() {
    let mut raset = Raset::new(0..=128);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 0..=128 -> [0x00, 0x00, 0x00, 0x80]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x80]);
  }

  #[test]
  fn test_raset_parm_bytes_with_offset() {
    let mut raset = Raset::new(10..=100);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 10..=100 -> [0x00, 0x0A, 0x00, 0x64]
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x64]);
  }

  #[test]
  fn test_raset_parm_bytes_large_values() {
    let mut raset = Raset::new(256..=512);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 256..=512 -> [0x01, 0x00, 0x02, 0x00]
    assert_eq!(params, vec![0x01, 0x00, 0x02, 0x00]);
  }

  #[test]
  fn test_raset_post_delay() {
    let mut raset = Raset::new(0..=128);
    assert_eq!(raset.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_raset_exclusive_range() {
    let mut raset = Raset::new(10..100);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 10..100 (exclusive end) -> [0x00, 0x0A, 0x00, 0x63]
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x63]);
  }

  #[test]
  fn test_raset_range_from() {
    let mut raset = Raset::new(50..);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 50.. (unbounded end) -> [0x00, 0x32, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x32, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_range_to_inclusive() {
    let mut raset = Raset::new(..=200);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: ..=200 (unbounded start) -> [0x00, 0x00, 0x00, 0xC8]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC8]);
  }

  #[test]
  fn test_raset_range_to() {
    let mut raset = Raset::new(..150);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: ..150 (exclusive end) -> [0x00, 0x00, 0x00, 0x95]
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x95]);
  }

  #[test]
  fn test_raset_full_range() {
    let mut raset = Raset::new(..);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: .. (fully unbounded) -> [0x00, 0x00, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_single_point() {
    let mut raset = Raset::new(42..=42);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 42..=42 (single point) -> [0x00, 0x2A, 0x00, 0x2A]
    assert_eq!(params, vec![0x00, 0x2A, 0x00, 0x2A]);
  }

  #[test]
  fn test_raset_max_value() {
    let mut raset = Raset::new(0..=u16::MAX);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 0..=65535 -> [0x00, 0x00, 0xFF, 0xFF]
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_high_values() {
    let mut raset = Raset::new(1000..=2000);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    // y_range: 1000..=2000 -> [0x03, 0xE8, 0x07, 0xD0]
    assert_eq!(params, vec![0x03, 0xE8, 0x07, 0xD0]);
  }

  #[test]
  fn test_fill_rect_16bit_red_2x2() {
    use crate::color_format::{Pixel, Pixel16};
    // 16bit: RGB 5:6:5
    // Pixel16::RED = (31, 0, 0) -> 0b11111_000000_00000 = 0xF800
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel16>::RED);
    assert_eq!(ramwr.cmd_byte(), 0x2c);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 2x2 = 4ピクセル, 各ピクセル2バイト = 8バイト
    assert_eq!(bytes, vec![0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00]);
  }

  #[test]
  fn test_fill_rect_16bit_green_3x2() {
    use crate::color_format::{Pixel, Pixel16};
    // 16bit: RGB 5:6:5
    // Pixel16::GREEN = (0, 63, 0) -> 0b00000_111111_00000 = 0x07E0
    let mut ramwr = Ramwr::fill_rect(0..=2, 0..=1, Pixel::<Pixel16>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 3x2 = 6ピクセル, 各ピクセル2バイト = 12バイト
    assert_eq!(bytes, vec![
      0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0,
      0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0
    ]);
  }

  #[test]
  fn test_fill_rect_16bit_blue_1x1() {
    use crate::color_format::{Pixel, Pixel16};
    // 16bit: RGB 5:6:5
    // Pixel16::BLUE = (0, 0, 31) -> 0b00000_000000_11111 = 0x001F
    let mut ramwr = Ramwr::fill_rect(5..=5, 10..=10, Pixel::<Pixel16>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 1x1 = 1ピクセル, 2バイト
    assert_eq!(bytes, vec![0x00, 0x1F]);
  }

  #[test]
  fn test_fill_rect_16bit_white_2x3() {
    use crate::color_format::{Pixel, Pixel16};
    // 16bit: RGB 5:6:5
    // Pixel16::WHITE = (31, 63, 31) -> 0b11111_111111_11111 = 0xFFFF
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=2, Pixel::<Pixel16>::WHITE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 2x3 = 6ピクセル, 各ピクセル2バイト = 12バイト
    assert_eq!(bytes, vec![
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ]);
  }

  #[test]
  fn test_fill_rect_18bit_red_2x2() {
    use crate::color_format::{Pixel, Pixel18};
    // 18bit: RGB 6:6:6
    // Pixel18::RED = (63, 0, 0) -> 0x3F, 0x00, 0x00
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel18>::RED);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 2x2 = 4ピクセル, 各ピクセル3バイト = 12バイト
    assert_eq!(bytes, vec![
      0x3F, 0x00, 0x00, 0x3F, 0x00, 0x00,
      0x3F, 0x00, 0x00, 0x3F, 0x00, 0x00
    ]);
  }

  #[test]
  fn test_fill_rect_18bit_green_3x1() {
    use crate::color_format::{Pixel, Pixel18};
    // 18bit: RGB 6:6:6
    // Pixel18::GREEN = (0, 63, 0) -> 0x00, 0x3F, 0x00
    let mut ramwr = Ramwr::fill_rect(0..=2, 5..=5, Pixel::<Pixel18>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 3x1 = 3ピクセル, 各ピクセル3バイト = 9バイト
    assert_eq!(bytes, vec![
      0x00, 0x3F, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x3F, 0x00
    ]);
  }

  #[test]
  fn test_fill_rect_18bit_blue_1x4() {
    use crate::color_format::{Pixel, Pixel18};
    // 18bit: RGB 6:6:6
    // Pixel18::BLUE = (0, 0, 63) -> 0x00, 0x00, 0x3F
    let mut ramwr = Ramwr::fill_rect(10..=10, 0..=3, Pixel::<Pixel18>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 1x4 = 4ピクセル, 各ピクセル3バイト = 12バイト
    assert_eq!(bytes, vec![
      0x00, 0x00, 0x3F, 0x00, 0x00, 0x3F,
      0x00, 0x00, 0x3F, 0x00, 0x00, 0x3F
    ]);
  }

  #[test]
  fn test_fill_rect_12bit_red_2x2() {
    use crate::color_format::{Pixel, Pixel12};
    // 12bit: RGB 4:4:4, 2ピクセルで3バイト
    // RED = (15, 0, 0) -> R4=0xF, G4=0x0, B4=0x0
    // パターン: [0xF0, 0x0F, 0x00]
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel12>::RED);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 2x2 = 4ピクセル, 2ピクセルで3バイト = 6バイト
    assert_eq!(bytes, vec![0xF0, 0x0F, 0x00, 0xF0, 0x0F, 0x00]);
  }

  #[test]
  fn test_fill_rect_12bit_green_4x1() {
    use crate::color_format::{Pixel, Pixel12};
    // 12bit: RGB 4:4:4
    // GREEN = (0, 15, 0) -> R4=0x0, G4=0xF, B4=0x0
    // パターン: [0x0F, 0x00, 0xF0]
    let mut ramwr = Ramwr::fill_rect(0..=3, 5..=5, Pixel::<Pixel12>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 4x1 = 4ピクセル, 2ピクセルで3バイト = 6バイト
    assert_eq!(bytes, vec![0x0F, 0x00, 0xF0, 0x0F, 0x00, 0xF0]);
  }

  #[test]
  fn test_fill_rect_12bit_blue_2x4() {
    use crate::color_format::{Pixel, Pixel12};
    // 12bit: RGB 4:4:4
    // BLUE = (0, 0, 15) -> R4=0x0, G4=0x0, B4=0xF
    // パターン: [0x00, 0xF0, 0x0F]
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=3, Pixel::<Pixel12>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 2x4 = 8ピクセル, 2ピクセルで3バイト = 12バイト
    assert_eq!(bytes, vec![
      0x00, 0xF0, 0x0F, 0x00, 0xF0, 0x0F,
      0x00, 0xF0, 0x0F, 0x00, 0xF0, 0x0F
    ]);
  }

  #[test]
  fn test_fill_rect_12bit_white_6x2() {
    use crate::color_format::{Pixel, Pixel12};
    // 12bit: RGB 4:4:4
    // WHITE = (15, 15, 15) -> R4=0xF, G4=0xF, B4=0xF
    // パターン: [0xFF, 0xFF, 0xFF]
    let mut ramwr = Ramwr::fill_rect(0..=5, 0..=1, Pixel::<Pixel12>::WHITE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 6x2 = 12ピクセル, 2ピクセルで3バイト = 18バイト
    assert_eq!(bytes, vec![
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ]);
  }

  #[test]
  #[should_panic(expected = "Pixel count must be even in 12-bit mode")]
  fn test_fill_rect_12bit_odd_pixel_count_panics() {
    use crate::color_format::{Pixel, Pixel12};
    // 3x1 = 3ピクセル (奇数) -> パニックするはず
    let _ramwr = Ramwr::fill_rect(0..=2, 0..=0, Pixel::<Pixel12>::RED);
  }

  #[test]
  fn test_fill_rect_16bit_custom_color() {
    use crate::color_format::{Pixel, Pixel16};
    // カスタムカラー: R=10(0x0A), G=20(0x14), B=5(0x05)
    // 16bit変換: R5=10, G6=20, B5=5 -> 0b01010_010100_00101 = 0x5285
    let custom = Pixel::<Pixel16>::new(10, 20, 5);
    let mut ramwr = Ramwr::fill_rect(0..=0, 0..=0, custom);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // 1x1 = 1ピクセル, 2バイト
    assert_eq!(bytes, vec![0x52, 0x85]);
  }

  #[test]
  fn test_fill_rect_post_delay() {
    use crate::color_format::{Pixel, Pixel16};
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel16>::RED);
    assert_eq!(ramwr.post_delay(), Duration::from_millis(0));
  }
}
