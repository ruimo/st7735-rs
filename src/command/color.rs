//! Color format configuration commands
//!
//! This module contains commands for configuring the display's color format.

use core::time::Duration;
use crate::color_format::ColorFormat;
use super::Command;

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
/// use st7735_rs::command::{Command, color::Colmod};
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_colmod_cmd_byte() {
    let colmod = Colmod::new(ColorFormat::Bit16);
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
  fn test_colmod_post_delay() {
    let colmod = Colmod::new(ColorFormat::Bit16);
    assert_eq!(colmod.post_delay(), Duration::from_millis(0));
  }
}

// Made with Bob
