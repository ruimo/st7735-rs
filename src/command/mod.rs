//! ST7735 command implementations
//!
//! This module provides type-safe implementations of ST7735 display controller commands.
//! Each command is represented as a struct that implements the [`Command`] trait.
//!
//! # Command Categories
//!
//! - **Display Control**: [`display::Dispon`], [`display::Slpout`]
//! - **Color Configuration**: [`color::Colmod`]
//! - **Drawing Area**: [`range::Caset`], [`range::Raset`]
//! - **Memory Write**: [`memory::Ramwr`]
//! - **Address Control**: [`address::Madctl`]
//! - **Text Rendering**: [`text::draw_char`]
//!
//! # Usage Example
//!
//! ```rust
//! use st7735_rs::command::{Command, display::Slpout, color::Colmod, display::Dispon};
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

pub mod display;
pub mod color;
pub mod range;
pub mod memory;
pub mod address;
pub mod text;

// Re-export commonly used items for convenience
pub use display::{Dispon, Slpout};
pub use color::Colmod;
pub use range::{Caset, Raset};
pub use memory::Ramwr;
pub use address::{Madctl, AddressOrder, VerticalRefreshOrder, HorizontalRefreshOrder, RgbBgb};
pub use text::draw_char;

// Made with Bob
