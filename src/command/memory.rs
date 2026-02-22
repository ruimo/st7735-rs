//! Memory write commands and iterators
//!
//! This module contains commands for writing pixel data to display memory,
//! along with efficient iterators for generating pixel data.

use core::time::Duration;
use core::ops::RangeBounds;
use crate::color_format::{ColorFormat, ColorFormatMarker, Pixel};
use super::Command;
use super::range::{range_start, range_end};
use heapless::Deque;

/// Memory Write command (0x2C)
///
/// Writes pixel data to the display memory. The data format depends on the
/// color mode set by [`crate::command::Colmod`]. The write area is defined by previous
/// [`crate::command::Caset`] and [`crate::command::Raset`] commands.
///
/// # Generic Parameter
///
/// * `T` - An iterator that yields bytes to write to display memory
///
/// # Example
///
/// ```
/// use st7735_rs::command::{Command, memory::Ramwr};
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

/// Iterator for generating pixel data on-demand using a function
///
/// This iterator computes pixel colors dynamically by calling a function
/// for each (x, y) coordinate, converting pixels to bytes on the fly.
pub struct DrawRectIterator<C: ColorFormatMarker, F: Fn(u16, u16) -> Pixel<C>> {
  f: F,
  color_format: ColorFormat,
  x_start: u16,
  x_end: u16,
  y_end: u16,
  current_x: u16,
  current_y: u16,
  byte_buffer: Deque<u8, 3>,
  pending_pixel: Option<Pixel<C>>,
  _marker: core::marker::PhantomData<C>,
}

impl<C: ColorFormatMarker, F: Fn(u16, u16) -> Pixel<C>> DrawRectIterator<C, F> {
  fn new(
    x_start: u16,
    x_end: u16,
    y_start: u16,
    y_end: u16,
    f: F,
  ) -> Self {
    Self {
      f,
      color_format: C::FORMAT,
      x_start,
      x_end,
      y_end,
      current_x: x_start,
      current_y: y_start,
      byte_buffer: Deque::new(),
      pending_pixel: None,
      _marker: core::marker::PhantomData,
    }
  }
}

impl<C: ColorFormatMarker, F: Fn(u16, u16) -> Pixel<C>> Iterator for DrawRectIterator<C, F> {
  type Item = u8;
  
  fn next(&mut self) -> Option<Self::Item> {
    // If we have buffered bytes, return them first
    if let Some(byte) = self.byte_buffer.pop_front() {
      return Some(byte);
    }
    
    // Check if we've finished all pixels
    if self.current_y > self.y_end {
      return None;
    }
    
    // Generate next pixel
    let pixel = (self.f)(self.current_x, self.current_y);
    
    // Convert pixel to bytes based on color format and push to buffer
    match self.color_format {
      ColorFormat::Bit12 => {
        // 12-bit: RGB 4:4:4, 2 pixels in 3 bytes
        // Format: [R1R1R1R1 G1G1G1G1] [B1B1B1B1 R2R2R2R2] [G2G2G2G2 B2B2B2B2]
        if let Some(pixel1) = self.pending_pixel.take() {
          // We have a pending pixel, pack it with the current pixel
          let r1 = pixel1.r & 0x0F;
          let g1 = pixel1.g & 0x0F;
          let b1 = pixel1.b & 0x0F;
          let r2 = pixel.r & 0x0F;
          let g2 = pixel.g & 0x0F;
          let b2 = pixel.b & 0x0F;
          
          let _ = self.byte_buffer.push_back((r1 << 4) | g1);
          let _ = self.byte_buffer.push_back((b1 << 4) | r2);
          let _ = self.byte_buffer.push_back((g2 << 4) | b2);
        } else {
          // Store this pixel and wait for the next one
          self.pending_pixel = Some(pixel);
          // Advance to next pixel position
          self.current_x += 1;
          if self.current_x > self.x_end {
            self.current_x = self.x_start;
            self.current_y += 1;
          }
          // Recursively call next to get the second pixel
          return self.next();
        }
      },
      ColorFormat::Bit16 => {
        // 16-bit: RGB 5:6:5, 1 pixel in 2 bytes
        let r5 = (pixel.r & 0x1F) as u16;
        let g6 = (pixel.g & 0x3F) as u16;
        let b5 = (pixel.b & 0x1F) as u16;
        
        let color16 = (r5 << 11) | (g6 << 5) | b5;
        let _ = self.byte_buffer.push_back((color16 >> 8) as u8);
        let _ = self.byte_buffer.push_back(color16 as u8);
      },
      ColorFormat::Bit18 => {
        // 18-bit: RGB 6:6:6, 1 pixel in 3 bytes
        let _ = self.byte_buffer.push_back(pixel.r & 0x3F);
        let _ = self.byte_buffer.push_back(pixel.g & 0x3F);
        let _ = self.byte_buffer.push_back(pixel.b & 0x3F);
      },
    }
    
    // Advance to next pixel position
    self.current_x += 1;
    if self.current_x > self.x_end {
      self.current_x = self.x_start;
      self.current_y += 1;
    }
    
    // Return first byte from buffer
    self.byte_buffer.pop_front()
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
  /// * `F` - Color format marker ([`crate::color_format::Pixel12`], [`crate::color_format::Pixel16`], or [`crate::color_format::Pixel18`])
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
  /// use st7735_rs::command::memory::Ramwr;
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

impl<C: ColorFormatMarker, F: Fn(u16, u16) -> Pixel<C>> Ramwr<DrawRectIterator<C, F>> {
  /// Creates a RAMWR command to draw a rectangular area using a pixel function
  ///
  /// This method generates pixel data on-demand by calling a function for each (x, y)
  /// coordinate in the specified rectangle. The function is called lazily as bytes are
  /// consumed from the iterator, making it memory-efficient.
  ///
  /// # Type Parameters
  ///
  /// * `C` - Color format marker ([`crate::color_format::Pixel12`], [`crate::color_format::Pixel16`], or [`crate::color_format::Pixel18`])
  /// * `F` - Function that takes (x, y) coordinates and returns a [`Pixel<C>`]
  ///
  /// # Parameters
  ///
  /// * `x_range` - Column range (X coordinates)
  /// * `y_range` - Row range (Y coordinates)
  /// * `f` - Function to generate pixel color for each coordinate
  ///
  /// # Panics
  ///
  /// Panics in 12-bit mode if the total pixel count is odd, as 12-bit mode
  /// requires an even number of pixels (2 pixels per 3 bytes).
  ///
  /// # Example
  ///
  /// ```
  /// use st7735_rs::command::memory::Ramwr;
  /// use st7735_rs::color_format::{Pixel, Pixel16};
  ///
  /// // Create a gradient pattern
  /// let ramwr = Ramwr::draw_rect(0..=10, 0..=10, |x, y| {
  ///     let intensity = ((x + y) * 2) as u8;
  ///     Pixel::<Pixel16>::new(intensity, intensity, intensity)
  /// });
  /// ```
  pub fn draw_rect(
    x_range: impl RangeBounds<u16>,
    y_range: impl RangeBounds<u16>,
    f: F,
  ) -> Self {
    let x_start = range_start(&x_range);
    let x_end = range_end(&x_range);
    let y_start = range_start(&y_range);
    let y_end = range_end(&y_range);
    
    let width = (x_end - x_start + 1) as usize;
    let height = (y_end - y_start + 1) as usize;
    let pixel_count = width * height;
    
    // 12-bit mode requires even pixel count
    if matches!(C::FORMAT, ColorFormat::Bit12) && pixel_count % 2 == 1 {
      panic!("Pixel count must be even in 12-bit mode. Current pixel count: {}", pixel_count);
    }
    
    Self {
      bytes: Some(DrawRectIterator::new(x_start, x_end, y_start, y_end, f))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::color_format::{Pixel16, Pixel18, Pixel12};

  #[test]
  fn test_fill_rect_16bit_red_2x2() {
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel16>::RED);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // RED in 16-bit: R=31, G=0, B=0 -> 0xF800
    assert_eq!(bytes, vec![0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00]);
  }

  #[test]
  fn test_fill_rect_16bit_green_3x2() {
    let mut ramwr = Ramwr::fill_rect(0..=2, 0..=1, Pixel::<Pixel16>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // GREEN in 16-bit: R=0, G=63, B=0 -> 0x07E0
    let expected = vec![0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0];
    assert_eq!(bytes, expected);
  }

  #[test]
  fn test_fill_rect_16bit_blue_1x1() {
    let mut ramwr = Ramwr::fill_rect(5..=5, 10..=10, Pixel::<Pixel16>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // BLUE in 16-bit: R=0, G=0, B=31 -> 0x001F
    assert_eq!(bytes, vec![0x00, 0x1F]);
  }

  #[test]
  fn test_fill_rect_16bit_white_2x3() {
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=2, Pixel::<Pixel16>::WHITE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // WHITE in 16-bit: R=31, G=63, B=31 -> 0xFFFF
    assert_eq!(bytes, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
  }

  #[test]
  fn test_fill_rect_18bit_red_2x2() {
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel18>::RED);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // RED in 18-bit: R=63, G=0, B=0
    assert_eq!(bytes, vec![63, 0, 0, 63, 0, 0, 63, 0, 0, 63, 0, 0]);
  }

  #[test]
  fn test_fill_rect_18bit_green_3x1() {
    let mut ramwr = Ramwr::fill_rect(0..=2, 5..=5, Pixel::<Pixel18>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // GREEN in 18-bit: R=0, G=63, B=0
    assert_eq!(bytes, vec![0, 63, 0, 0, 63, 0, 0, 63, 0]);
  }

  #[test]
  fn test_fill_rect_18bit_blue_1x4() {
    let mut ramwr = Ramwr::fill_rect(10..=10, 0..=3, Pixel::<Pixel18>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // BLUE in 18-bit: R=0, G=0, B=63
    assert_eq!(bytes, vec![0, 0, 63, 0, 0, 63, 0, 0, 63, 0, 0, 63]);
  }

  #[test]
  fn test_fill_rect_12bit_red_2x2() {
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, Pixel::<Pixel12>::RED);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // RED in 12-bit: R=15, G=0, B=0
    // Pattern: [R4G4, B4R4, G4B4] = [0xF0, 0x0F, 0x00]
    assert_eq!(bytes, vec![0xF0, 0x0F, 0x00, 0xF0, 0x0F, 0x00]);
  }

  #[test]
  fn test_fill_rect_12bit_green_4x1() {
    let mut ramwr = Ramwr::fill_rect(0..=3, 5..=5, Pixel::<Pixel12>::GREEN);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // GREEN in 12-bit: R=0, G=15, B=0
    // Pattern: [R4G4, B4R4, G4B4] = [0x0F, 0x00, 0xF0]
    assert_eq!(bytes, vec![0x0F, 0x00, 0xF0, 0x0F, 0x00, 0xF0]);
  }

  #[test]
  fn test_fill_rect_12bit_blue_2x4() {
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=3, Pixel::<Pixel12>::BLUE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // BLUE in 12-bit: R=0, G=0, B=15
    // Pattern: [R4G4, B4R4, G4B4] = [0x00, 0xF0, 0x0F]
    assert_eq!(bytes, vec![0x00, 0xF0, 0x0F, 0x00, 0xF0, 0x0F, 0x00, 0xF0, 0x0F, 0x00, 0xF0, 0x0F]);
  }

  #[test]
  fn test_fill_rect_12bit_white_6x2() {
    let mut ramwr = Ramwr::fill_rect(0..=5, 0..=1, Pixel::<Pixel12>::WHITE);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // WHITE in 12-bit: R=15, G=15, B=15
    // Pattern: [R4G4, B4R4, G4B4] = [0xFF, 0xFF, 0xFF]
    assert_eq!(bytes, vec![0xFF; 18]);
  }

  #[test]
  #[should_panic(expected = "Pixel count must be even in 12-bit mode")]
  fn test_fill_rect_12bit_odd_pixel_count_panics() {
    let _ramwr = Ramwr::fill_rect(0..=2, 0..=0, Pixel::<Pixel12>::RED);
  }

  #[test]
  fn test_fill_rect_16bit_custom_color() {
    let custom = Pixel::<Pixel16>::new(10, 20, 15);
    let mut ramwr = Ramwr::fill_rect(0..=1, 0..=1, custom);
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    // Custom color: R=10, G=20, B=15
    // 16-bit: (10 << 11) | (20 << 5) | 15 = 0x5297
    assert_eq!(bytes, vec![0x52, 0x8F, 0x52, 0x8F, 0x52, 0x8F, 0x52, 0x8F]);
  }

  #[test]
  fn test_fill_rect_post_delay() {
    let ramwr = Ramwr::fill_rect(0..=10, 0..=10, Pixel::<Pixel16>::RED);
    assert_eq!(ramwr.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_draw_rect_16bit_gradient() {
    let ramwr = Ramwr::draw_rect(0..=1, 0..=1, |x, y| {
      let val = ((x + y) * 8) as u8;
      Pixel::<Pixel16>::new(val, val, val)
    });
    let mut ramwr = ramwr;
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // (0,0): val=0  -> 0x0000
    // (1,0): val=8  -> (8<<11)|(16<<5)|8 = 0x4208
    // (0,1): val=8  -> 0x4208
    // (1,1): val=16 -> (16<<11)|(32<<5)|16 = 0x8410
    assert_eq!(bytes.len(), 8);
  }

  #[test]
  fn test_draw_rect_16bit_checkerboard() {
    let ramwr = Ramwr::draw_rect(0..=1, 0..=1, |x, y| {
      if (x + y) % 2 == 0 {
        Pixel::<Pixel16>::WHITE
      } else {
        Pixel::<Pixel16>::BLACK
      }
    });
    let mut ramwr = ramwr;
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // (0,0): WHITE -> 0xFFFF
    // (1,0): BLACK -> 0x0000
    // (0,1): BLACK -> 0x0000
    // (1,1): WHITE -> 0xFFFF
    assert_eq!(bytes, vec![0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_draw_rect_18bit_position_based() {
    let ramwr = Ramwr::draw_rect(0..=1, 0..=0, |x, _y| {
      Pixel::<Pixel18>::new(x as u8 * 10, x as u8 * 20, x as u8 * 30)
    });
    let mut ramwr = ramwr;
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // (0,0): (0, 0, 0)
    // (1,0): (10, 20, 30)
    assert_eq!(bytes, vec![0, 0, 0, 10, 20, 30]);
  }

  #[test]
  fn test_draw_rect_12bit_simple() {
    let ramwr = Ramwr::draw_rect(0..=1, 0..=0, |x, _y| {
      if x == 0 {
        Pixel::<Pixel12>::RED
      } else {
        Pixel::<Pixel12>::BLUE
      }
    });
    let mut ramwr = ramwr;
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // RED: R=15, G=0, B=0
    // BLUE: R=0, G=0, B=15
    // Packed: [R1G1, B1R2, G2B2] = [0xF0, 0x00, 0x0F]
    assert_eq!(bytes, vec![0xF0, 0x00, 0x0F]);
  }

  #[test]
  fn test_draw_rect_12bit_line() {
    let ramwr = Ramwr::draw_rect(0..=3, 0..=0, |x, _y| {
      if x % 2 == 0 {
        Pixel::<Pixel12>::WHITE
      } else {
        Pixel::<Pixel12>::BLACK
      }
    });
    let mut ramwr = ramwr;
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // WHITE, BLACK, WHITE, BLACK
    // Pair 1: WHITE+BLACK -> [0xFF, 0xF0, 0x00]
    // Pair 2: WHITE+BLACK -> [0xFF, 0xF0, 0x00]
    assert_eq!(bytes, vec![0xFF, 0xF0, 0x00, 0xFF, 0xF0, 0x00]);
  }

  #[test]
  #[should_panic(expected = "Pixel count must be even in 12-bit mode")]
  fn test_draw_rect_12bit_odd_pixel_count_panics() {
    let _ramwr = Ramwr::draw_rect(0..=2, 0..=0, |_x, _y| Pixel::<Pixel12>::RED);
  }
}

// Made with Bob
