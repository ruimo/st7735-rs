//! Text rendering functions
//!
//! This module contains functions for rendering text characters
//! using bitmap fonts.

use crate::color_format::{ColorFormatMarker, Pixel};
use super::memory::{Ramwr, DrawRectIterator};

/// Helper function to create a RAMWR command for drawing a single character
///
/// This function renders an 8x8 character from the font bitmap using the
/// existing `draw_rect()` infrastructure. Each bit in the bitmap represents
/// a pixel: 1 for foreground color, 0 for background color.
///
/// The font data is searched using binary search, assuming the data is sorted
/// by character code in ascending order.
///
/// # Type Parameters
///
/// * `C` - Color format marker ([`crate::color_format::Pixel12`], [`crate::color_format::Pixel16`], or [`crate::color_format::Pixel18`])
///
/// # Parameters
///
/// * `ch` - The character to draw
/// * `fg_color` - Foreground color (for '1' bits in the bitmap)
/// * `bg_color` - Background color (for '0' bits in the bitmap)
///
/// # Returns
///
/// Returns `Some(Ramwr)` if the character is found in the font data,
/// or `None` if the character is not available.
///
/// # Note
///
/// Before calling this function, you must set the drawing area using
/// [`crate::command::Caset`] and [`crate::command::Raset`] commands to define an 8x8 pixel region
/// at the desired position.
///
/// # Example
///
/// ```
/// use st7735_rs::command::text::draw_char;
/// use st7735_rs::color_format::{Pixel, Pixel16};
///
/// // Draw the character '5' in white on black background
/// if let Some(ramwr) = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK) {
///     // Send the command via SPI
/// }
/// ```
pub fn draw_char<C: ColorFormatMarker + Copy>(
  ch: char,
  fg_color: Pixel<C>,
  bg_color: Pixel<C>,
) -> Option<Ramwr<DrawRectIterator<C, impl Fn(u16, u16) -> Pixel<C>>>> {
  // Binary search for the character in FONT_DATA (assumes sorted by char)
  let bitmap = crate::FONT_DATA
    .binary_search_by_key(&ch, |(c, _)| *c)
    .ok()
    .map(|idx| crate::FONT_DATA[idx].1)?;
  
  // Use draw_rect with a closure that reads from the bitmap
  // The character is 8x8 pixels
  Some(Ramwr::draw_rect(0..=7, 0..=7, move |px, py| {
    let row_byte = bitmap[py as usize];
    let bit_mask = 1 << (7 - px);
    if (row_byte & bit_mask) != 0 {
      fg_color
    } else {
      bg_color
    }
  }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::color_format::{Pixel16, Pixel18, Pixel12};
  use crate::command::Command;

  #[test]
  fn test_draw_char_16bit_found() {
    let ramwr = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr.is_some());
    let ramwr = ramwr.unwrap();
    assert_eq!(ramwr.cmd_byte(), 0x2C);
  }

  #[test]
  fn test_draw_char_16bit_not_found() {
    let ramwr = draw_char('あ', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr.is_none());
  }

  #[test]
  fn test_draw_char_bitmap_content_16bit() {
    let ramwr = draw_char('0', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // Should be 8x8 pixels = 64 pixels * 2 bytes/pixel = 128 bytes
    assert_eq!(bytes.len(), 128);
    
    // Check that we have both white (0xFF, 0xFF) and black (0x00, 0x00) pixels
    let has_white = bytes.chunks(2).any(|chunk| chunk == [0xFF, 0xFF]);
    let has_black = bytes.chunks(2).any(|chunk| chunk == [0x00, 0x00]);
    assert!(has_white, "Should have white pixels");
    assert!(has_black, "Should have black pixels");
  }

  #[test]
  fn test_draw_char_bitmap_content_18bit() {
    let ramwr = draw_char('1', Pixel::<Pixel18>::WHITE, Pixel::<Pixel18>::BLACK);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // Should be 8x8 pixels = 64 pixels * 3 bytes/pixel = 192 bytes
    assert_eq!(bytes.len(), 192);
    
    // Check that we have both white (63, 63, 63) and black (0, 0, 0) pixels
    let has_white = bytes.chunks(3).any(|chunk| chunk == [63, 63, 63]);
    let has_black = bytes.chunks(3).any(|chunk| chunk == [0, 0, 0]);
    assert!(has_white, "Should have white pixels");
    assert!(has_black, "Should have black pixels");
  }

  #[test]
  fn test_draw_char_bitmap_content_12bit() {
    let ramwr = draw_char('2', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    // Should be 8x8 pixels = 64 pixels
    // In 12-bit mode: 2 pixels = 3 bytes, so 64 pixels = 96 bytes
    assert_eq!(bytes.len(), 96);
    
    // In 12-bit mode, white pixels are 0xFF (all bits set in 4-bit components)
    // and black pixels are 0x00
    // The pattern depends on pixel pairing, but we should see both 0xFF and 0x00
    let has_ff = bytes.iter().any(|&b| b == 0xFF);
    let has_00 = bytes.iter().any(|&b| b == 0x00);
    assert!(has_ff, "Should have 0xFF bytes");
    assert!(has_00, "Should have 0x00 bytes");
  }

  #[test]
  fn test_draw_char_consistent_output() {
    // Drawing the same character twice should produce the same output
    let ramwr1 = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    let ramwr2 = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    
    assert!(ramwr1.is_some());
    assert!(ramwr2.is_some());
    
    let mut ramwr1 = ramwr1.unwrap();
    let mut ramwr2 = ramwr2.unwrap();
    
    let bytes1: Vec<u8> = ramwr1.parm_bytes().into_iter().collect();
    let bytes2: Vec<u8> = ramwr2.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes1, bytes2);
  }

  #[test]
  fn test_draw_char_all_digits() {
    // Test that all digits 0-9 can be drawn
    for ch in '0'..='9' {
      let ramwr = draw_char(ch, Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
      assert!(ramwr.is_some(), "Digit '{}' should be available", ch);
    }
  }

  #[test]
  fn test_draw_char_post_delay() {
    let ramwr = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr.is_some());
    let ramwr = ramwr.unwrap();
    assert_eq!(ramwr.post_delay(), core::time::Duration::from_millis(0));
  }
}

// Made with Bob
