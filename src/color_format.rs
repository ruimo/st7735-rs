//! Color format and pixel definitions
//!
//! This module provides type-safe color format representations for the ST7735 display.
//! It supports three color modes: 12-bit (RGB 4:4:4), 16-bit (RGB 5:6:5), and 18-bit (RGB 6:6:6).
//!
//! # Usage Example
//!
//! ```
//! use st7735_rs::color_format::{Pixel, Pixel12, Pixel16, Pixel18};
//!
//! // Red color in 12-bit mode (R=15, G=0, B=0)
//! let red12 = Pixel::<Pixel12>::RED;
//!
//! // Red color in 16-bit mode (R=31, G=0, B=0)
//! let red16 = Pixel::<Pixel16>::RED;
//!
//! // Red color in 18-bit mode (R=63, G=0, B=0)
//! let red18 = Pixel::<Pixel18>::RED;
//!
//! // Custom color
//! let custom = Pixel::<Pixel16>::new(20, 40, 15);
//! ```

/// Color format enumeration for ST7735 display
///
/// Represents the three supported color modes of the ST7735 controller.
pub enum ColorFormat {
    /// 12-bit color mode (RGB 4:4:4)
    Bit12,
    /// 16-bit color mode (RGB 5:6:5)
    Bit16,
    /// 18-bit color mode (RGB 6:6:6)
    Bit18,
}

/// Marker trait for representing color formats at the type level
///
/// This trait enables compile-time color format validation and provides
/// format-specific constants for maximum color component values.
pub trait ColorFormatMarker {
    const FORMAT: ColorFormat;
    const R_MAX: u8;
    const G_MAX: u8;
    const B_MAX: u8;
}

/// 12-bit color format marker (RGB 4:4:4)
///
/// In 12-bit mode, each color component uses 4 bits, allowing values from 0 to 15.
/// Two pixels are packed into 3 bytes for efficient storage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel12;
impl ColorFormatMarker for Pixel12 {
    const FORMAT: ColorFormat = ColorFormat::Bit12;
    const R_MAX: u8 = 15; // 4-bit
    const G_MAX: u8 = 15; // 4-bit
    const B_MAX: u8 = 15; // 4-bit
}

/// 16-bit color format marker (RGB 5:6:5)
///
/// In 16-bit mode, red and blue use 5 bits (0-31), while green uses 6 bits (0-63).
/// Each pixel occupies exactly 2 bytes. This is the most commonly used format.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel16;
impl ColorFormatMarker for Pixel16 {
    const FORMAT: ColorFormat = ColorFormat::Bit16;
    const R_MAX: u8 = 31; // 5-bit
    const G_MAX: u8 = 63; // 6-bit
    const B_MAX: u8 = 31; // 5-bit
}

/// 18-bit color format marker (RGB 6:6:6)
///
/// In 18-bit mode, each color component uses 6 bits, allowing values from 0 to 63.
/// Each pixel occupies 3 bytes, providing the highest color depth.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel18;
impl ColorFormatMarker for Pixel18 {
    const FORMAT: ColorFormat = ColorFormat::Bit18;
    const R_MAX: u8 = 63; // 6-bit
    const G_MAX: u8 = 63; // 6-bit
    const B_MAX: u8 = 63; // 6-bit
}

/// Type-safe pixel structure
///
/// Represents a pixel with a specific color format. The format is enforced at compile time
/// through the generic parameter `F`, which must implement [`ColorFormatMarker`].
///
/// # Type Parameters
///
/// * `F` - A color format marker type (e.g., [`Pixel12`], [`Pixel16`], or [`Pixel18`])
///
/// # Examples
///
/// ```
/// use st7735_rs::color_format::{Pixel, Pixel16};
///
/// // Using predefined colors
/// let red = Pixel::<Pixel16>::RED;
/// let green = Pixel::<Pixel16>::GREEN;
///
/// // Creating custom colors
/// let purple = Pixel::<Pixel16>::new(20, 10, 25);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel<F: ColorFormatMarker> {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    _format: core::marker::PhantomData<F>,
}

impl<F: ColorFormatMarker> Pixel<F> {
    /// Creates a new pixel with the specified RGB values
    ///
    /// # Parameters
    ///
    /// * `r` - Red component (0 to `F::R_MAX`)
    /// * `g` - Green component (0 to `F::G_MAX`)
    /// * `b` - Blue component (0 to `F::B_MAX`)
    ///
    /// # Note
    ///
    /// Values exceeding the maximum for the color format will be masked to fit.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            _format: core::marker::PhantomData,
        }
    }

    /// Red color (maximum red, zero green and blue)
    pub const RED: Self = Self::new(F::R_MAX, 0, 0);
    
    /// Green color (maximum green, zero red and blue)
    pub const GREEN: Self = Self::new(0, F::G_MAX, 0);
    
    /// Blue color (maximum blue, zero red and green)
    pub const BLUE: Self = Self::new(0, 0, F::B_MAX);
    
    /// White color (maximum values for all components)
    pub const WHITE: Self = Self::new(F::R_MAX, F::G_MAX, F::B_MAX);
    
    /// Black color (zero for all components)
    pub const BLACK: Self = Self::new(0, 0, 0);
    
    /// Yellow color (maximum red and green, zero blue)
    pub const YELLOW: Self = Self::new(F::R_MAX, F::G_MAX, 0);
    
    /// Cyan color (maximum green and blue, zero red)
    pub const CYAN: Self = Self::new(0, F::G_MAX, F::B_MAX);
    
    /// Magenta color (maximum red and blue, zero green)
    pub const MAGENTA: Self = Self::new(F::R_MAX, 0, F::B_MAX);
}