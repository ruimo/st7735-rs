//! # ST7735 Display Driver
//!
//! A no_std Rust driver library for the ST7735 TFT LCD display controller.
//!
//! ## Features
//!
//! - **no_std support**: Can be used in embedded systems
//! - **Type-safe color formats**: Compile-time guarantees for 12-bit, 16-bit, and 18-bit color modes
//! - **Flexible range specification**: Supports Rust's standard range syntax
//! - **Zero-cost abstractions**: Type safety without runtime overhead
//!
//! ## Usage Example
//!
//! ```rust
//! use st7735_rs::color_format::{Pixel, Pixel16};
//! use st7735_rs::command::{Command, Slpout, Dispon, Colmod, Caset, Raset, Ramwr};
//! use st7735_rs::color_format::ColorFormat;
//!
//! // Initialization sequence
//! let slpout = Slpout;
//! let colmod = Colmod::new(ColorFormat::Bit16);
//! let dispon = Dispon;
//!
//! // Set drawing area
//! let caset = Caset::new(0..=127);
//! let raset = Raset::new(0..=159);
//!
//! // Fill rectangle with red color
//! let ramwr = Ramwr::fill_rect(0..=10, 0..=10, Pixel::<Pixel16>::RED);
//!
//! // Or draw with a function for dynamic patterns
//! let ramwr = Ramwr::draw_rect(0..=10, 0..=10, |x, y| {
//!     let intensity = ((x + y) * 2) as u8;
//!     Pixel::<Pixel16>::new(intensity, intensity, intensity)
//! });
//! ```
//!
//! For details on how to send commands via SPI, see the [`command`] module documentation.
//!
//! ## Modules
//!
//! - [`color_format`]: Color format and pixel definitions
//! - [`command`]: ST7735 command implementations

pub mod color_format;
pub mod command;