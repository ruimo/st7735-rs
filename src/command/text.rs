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
  // Simply call draw_char_scaled with scale 1x1
  draw_char_scaled(ch, fg_color, bg_color, 1, 1)
}

/// Helper function to create a RAMWR command for drawing a scaled character
///
/// This function renders a character from the font bitmap with specified scale factors.
/// Each pixel in the original 8x8 bitmap is expanded to scale_x × scale_y pixels.
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
/// * `scale_x` - Horizontal scale factor (must be >= 1)
/// * `scale_y` - Vertical scale factor (must be >= 1)
///
/// # Returns
///
/// Returns `Some(Ramwr)` if the character is found in the font data,
/// or `None` if the character is not available.
///
/// # Note
///
/// Before calling this function, you must set the drawing area using
/// [`crate::command::Caset`] and [`crate::command::Raset`] commands to define a
/// (8×scale_x) × (8×scale_y) pixel region at the desired position.
///
/// # Panics
///
/// Panics if scale_x or scale_y is 0, or if the resulting pixel count is odd in 12-bit mode.
///
/// # Example
///
/// ```
/// use st7735_rs::command::text::draw_char_scaled;
/// use st7735_rs::color_format::{Pixel, Pixel16};
///
/// // Draw the character '5' at 2x scale in white on black background
/// if let Some(ramwr) = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2) {
///     // Send the command via SPI
///     // The character will be 16x16 pixels instead of 8x8
/// }
/// ```
pub fn draw_char_scaled<C: ColorFormatMarker + Copy>(
  ch: char,
  fg_color: Pixel<C>,
  bg_color: Pixel<C>,
  scale_x: u16,
  scale_y: u16,
) -> Option<Ramwr<DrawRectIterator<C, impl Fn(u16, u16) -> Pixel<C>>>> {
  // Validate scale factors
  if scale_x == 0 || scale_y == 0 {
    panic!("Scale factors must be at least 1. Got scale_x={}, scale_y={}", scale_x, scale_y);
  }

  // Binary search for the character in FONT_DATA (assumes sorted by char)
  let bitmap = crate::FONT_DATA
    .binary_search_by_key(&ch, |(c, _)| *c)
    .ok()
    .map(|idx| crate::FONT_DATA[idx].1)?;
  
  // Calculate the scaled dimensions
  let width = 8 * scale_x;
  let height = 8 * scale_y;
  
  // Use draw_rect with a closure that reads from the bitmap and scales
  Some(Ramwr::draw_rect(0..=(width - 1), 0..=(height - 1), move |px, py| {
    // Map scaled coordinates back to original 8x8 bitmap
    let orig_x = px / scale_x;
    let orig_y = py / scale_y;
    
    let row_byte = bitmap[orig_y as usize];
    let bit_mask = 1 << (7 - orig_x);
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

  #[test]
  fn test_draw_char_scaled_16bit_found() {
    let ramwr = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2);
    assert!(ramwr.is_some());
    let ramwr = ramwr.unwrap();
    assert_eq!(ramwr.cmd_byte(), 0x2C);
  }

  #[test]
  fn test_draw_char_scaled_16bit_not_found() {
    let ramwr = draw_char_scaled('あ', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2);
    assert!(ramwr.is_none());
  }

  #[test]
  fn test_draw_char_scaled_bitmap_size_16bit() {
    // 2x2 scale: 8x8 becomes 16x16 = 256 pixels * 2 bytes = 512 bytes
    let ramwr = draw_char_scaled('0', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes.len(), 512);
    
    // Check that we have both white and black pixels
    let has_white = bytes.chunks(2).any(|chunk| chunk == [0xFF, 0xFF]);
    let has_black = bytes.chunks(2).any(|chunk| chunk == [0x00, 0x00]);
    assert!(has_white, "Should have white pixels");
    assert!(has_black, "Should have black pixels");
  }

  #[test]
  fn test_draw_char_scaled_3x3_16bit() {
    // 3x3 scale: 8x8 becomes 24x24 = 576 pixels * 2 bytes = 1152 bytes
    let ramwr = draw_char_scaled('1', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 3, 3);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes.len(), 1152);
  }

  #[test]
  fn test_draw_char_scaled_asymmetric_16bit() {
    // 2x3 scale: 8x8 becomes 16x24 = 384 pixels * 2 bytes = 768 bytes
    let ramwr = draw_char_scaled('2', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 3);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes.len(), 768);
  }

  #[test]
  fn test_draw_char_scaled_1x1_same_as_draw_char() {
    // 1x1 scale should produce the same result as draw_char
    let ramwr1 = draw_char('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    let ramwr2 = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 1, 1);
    
    assert!(ramwr1.is_some());
    assert!(ramwr2.is_some());
    
    let mut ramwr1 = ramwr1.unwrap();
    let mut ramwr2 = ramwr2.unwrap();
    
    let bytes1: Vec<u8> = ramwr1.parm_bytes().into_iter().collect();
    let bytes2: Vec<u8> = ramwr2.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes1, bytes2);
  }

  #[test]
  fn test_draw_char_scaled_18bit() {
    // 2x2 scale: 8x8 becomes 16x16 = 256 pixels * 3 bytes = 768 bytes
    let ramwr = draw_char_scaled('3', Pixel::<Pixel18>::WHITE, Pixel::<Pixel18>::BLACK, 2, 2);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes.len(), 768);
    
    // Check that we have both white (63, 63, 63) and black (0, 0, 0) pixels
    let has_white = bytes.chunks(3).any(|chunk| chunk == [63, 63, 63]);
    let has_black = bytes.chunks(3).any(|chunk| chunk == [0, 0, 0]);
    assert!(has_white, "Should have white pixels");
    assert!(has_black, "Should have black pixels");
  }

  #[test]
  fn test_draw_char_scaled_12bit() {
    // 2x2 scale: 8x8 becomes 16x16 = 256 pixels
    // In 12-bit mode: 2 pixels = 3 bytes, so 256 pixels = 384 bytes
    let ramwr = draw_char_scaled('4', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK, 2, 2);
    assert!(ramwr.is_some());
    
    let mut ramwr = ramwr.unwrap();
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    
    assert_eq!(bytes.len(), 384);
    
    let has_ff = bytes.iter().any(|&b| b == 0xFF);
    let has_00 = bytes.iter().any(|&b| b == 0x00);
    assert!(has_ff, "Should have 0xFF bytes");
    assert!(has_00, "Should have 0x00 bytes");
  }

  #[test]
  #[should_panic(expected = "Scale factors must be at least 1")]
  fn test_draw_char_scaled_zero_scale_x_panics() {
    let _ramwr = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 0, 2);
  }

  #[test]
  #[should_panic(expected = "Scale factors must be at least 1")]
  fn test_draw_char_scaled_zero_scale_y_panics() {
    let _ramwr = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 0);
  }

  #[test]
  fn test_draw_char_scaled_post_delay() {
    let ramwr = draw_char_scaled('5', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2);
    assert!(ramwr.is_some());
    let ramwr = ramwr.unwrap();
    assert_eq!(ramwr.post_delay(), core::time::Duration::from_millis(0));
  }

  #[test]
  fn test_draw_char_scaled_pixel_pattern_verification() {
    // Test that scaling actually duplicates pixels correctly
    // Using a simple pattern: draw '0' which has a known bitmap
    
    // First, get the original 8x8 pattern
    let ramwr_1x = draw_char('0', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    // Now get the 2x2 scaled pattern
    let ramwr_2x = draw_char_scaled('0', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 2);
    assert!(ramwr_2x.is_some());
    let mut ramwr_2x = ramwr_2x.unwrap();
    let bytes_2x: Vec<u8> = ramwr_2x.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels * 2 bytes = 128 bytes
    assert_eq!(bytes_1x.len(), 128);
    // Scaled: 16x16 = 256 pixels * 2 bytes = 512 bytes
    assert_eq!(bytes_2x.len(), 512);
    
    // Verify that each pixel in the original is represented by a 2x2 block in the scaled version
    // Check a few specific positions to ensure scaling is working correctly
    for orig_y in 0..8 {
      for orig_x in 0..8 {
        let orig_idx = (orig_y * 8 + orig_x) * 2;
        let orig_pixel = [bytes_1x[orig_idx], bytes_1x[orig_idx + 1]];
        
        // In the scaled version, this pixel should appear as a 2x2 block
        for dy in 0..2 {
          for dx in 0..2 {
            let scaled_x = orig_x * 2 + dx;
            let scaled_y = orig_y * 2 + dy;
            let scaled_idx = (scaled_y * 16 + scaled_x) * 2;
            let scaled_pixel = [bytes_2x[scaled_idx], bytes_2x[scaled_idx + 1]];
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "Pixel mismatch at original ({}, {}) -> scaled ({}, {})",
              orig_x, orig_y, scaled_x, scaled_y
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_draw_char_scaled_3x_pattern_verification() {
    // Test 3x scaling to ensure it works for non-power-of-2 scales
    let ramwr_1x = draw_char('1', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    let ramwr_3x = draw_char_scaled('1', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 3, 3);
    assert!(ramwr_3x.is_some());
    let mut ramwr_3x = ramwr_3x.unwrap();
    let bytes_3x: Vec<u8> = ramwr_3x.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels * 2 bytes = 128 bytes
    assert_eq!(bytes_1x.len(), 128);
    // Scaled: 24x24 = 576 pixels * 2 bytes = 1152 bytes
    assert_eq!(bytes_3x.len(), 1152);
    
    // Verify a few sample positions
    for orig_y in [0, 3, 7].iter() {
      for orig_x in [0, 3, 7].iter() {
        let orig_idx = (orig_y * 8 + orig_x) * 2;
        let orig_pixel = [bytes_1x[orig_idx], bytes_1x[orig_idx + 1]];
        
        // Check the 3x3 block in the scaled version
        for dy in 0..3 {
          for dx in 0..3 {
            let scaled_x = orig_x * 3 + dx;
            let scaled_y = orig_y * 3 + dy;
            let scaled_idx = (scaled_y * 24 + scaled_x) * 2;
            let scaled_pixel = [bytes_3x[scaled_idx], bytes_3x[scaled_idx + 1]];
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "3x scale: Pixel mismatch at original ({}, {}) -> scaled ({}, {})",
              orig_x, orig_y, scaled_x, scaled_y
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_draw_char_scaled_asymmetric_pattern_verification() {
    // Test asymmetric scaling (2x3) to ensure x and y scales work independently
    let ramwr_1x = draw_char('2', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    let ramwr_2x3 = draw_char_scaled('2', Pixel::<Pixel16>::WHITE, Pixel::<Pixel16>::BLACK, 2, 3);
    assert!(ramwr_2x3.is_some());
    let mut ramwr_2x3 = ramwr_2x3.unwrap();
    let bytes_2x3: Vec<u8> = ramwr_2x3.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels * 2 bytes = 128 bytes
    assert_eq!(bytes_1x.len(), 128);
    // Scaled: 16x24 = 384 pixels * 2 bytes = 768 bytes
    assert_eq!(bytes_2x3.len(), 768);
    
    // Verify that each original pixel becomes a 2x3 block
    for orig_y in [0, 4, 7].iter() {
      for orig_x in [0, 4, 7].iter() {
        let orig_idx = (orig_y * 8 + orig_x) * 2;
        let orig_pixel = [bytes_1x[orig_idx], bytes_1x[orig_idx + 1]];
        
        // Check the 2x3 block (2 wide, 3 tall)
        for dy in 0..3 {
          for dx in 0..2 {
            let scaled_x = orig_x * 2 + dx;
            let scaled_y = orig_y * 3 + dy;
            let scaled_idx = (scaled_y * 16 + scaled_x) * 2;
            let scaled_pixel = [bytes_2x3[scaled_idx], bytes_2x3[scaled_idx + 1]];
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "2x3 scale: Pixel mismatch at original ({}, {}) -> scaled ({}, {})",
              orig_x, orig_y, scaled_x, scaled_y
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_draw_char_scaled_18bit_pattern_verification() {
    // Verify scaling works correctly for 18-bit color format
    let ramwr_1x = draw_char('3', Pixel::<Pixel18>::WHITE, Pixel::<Pixel18>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    let ramwr_2x = draw_char_scaled('3', Pixel::<Pixel18>::WHITE, Pixel::<Pixel18>::BLACK, 2, 2);
    assert!(ramwr_2x.is_some());
    let mut ramwr_2x = ramwr_2x.unwrap();
    let bytes_2x: Vec<u8> = ramwr_2x.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels * 3 bytes = 192 bytes
    assert_eq!(bytes_1x.len(), 192);
    // Scaled: 16x16 = 256 pixels * 3 bytes = 768 bytes
    assert_eq!(bytes_2x.len(), 768);
    
    // Verify a few sample positions with 18-bit (3 bytes per pixel)
    for orig_y in [0, 3, 7].iter() {
      for orig_x in [0, 3, 7].iter() {
        let orig_idx = (orig_y * 8 + orig_x) * 3;
        let orig_pixel = [bytes_1x[orig_idx], bytes_1x[orig_idx + 1], bytes_1x[orig_idx + 2]];
        
        // Check the 2x2 block
        for dy in 0..2 {
          for dx in 0..2 {
            let scaled_x = orig_x * 2 + dx;
            let scaled_y = orig_y * 2 + dy;
            let scaled_idx = (scaled_y * 16 + scaled_x) * 3;
            let scaled_pixel = [bytes_2x[scaled_idx], bytes_2x[scaled_idx + 1], bytes_2x[scaled_idx + 2]];
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "18-bit: Pixel mismatch at original ({}, {}) -> scaled ({}, {})",
              orig_x, orig_y, scaled_x, scaled_y
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_draw_char_scaled_12bit_pattern_verification() {
    // Verify scaling works correctly for 12-bit color format
    // 12-bit is tricky: 2 pixels = 3 bytes [R1G1, B1R2, G2B2]
    let ramwr_1x = draw_char('4', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    let ramwr_2x = draw_char_scaled('4', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK, 2, 2);
    assert!(ramwr_2x.is_some());
    let mut ramwr_2x = ramwr_2x.unwrap();
    let bytes_2x: Vec<u8> = ramwr_2x.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels, 2 pixels = 3 bytes, so 64 pixels = 96 bytes
    assert_eq!(bytes_1x.len(), 96);
    // Scaled: 16x16 = 256 pixels, 2 pixels = 3 bytes, so 256 pixels = 384 bytes
    assert_eq!(bytes_2x.len(), 384);
    
    // Helper function to extract a pixel from 12-bit data
    let get_12bit_pixel = |bytes: &[u8], pixel_idx: usize| -> (u8, u8, u8) {
      let byte_idx = (pixel_idx / 2) * 3;
      if pixel_idx % 2 == 0 {
        // Even pixel: [R1G1, B1R2, G2B2] -> R1, G1, B1
        let r = (bytes[byte_idx] >> 4) & 0x0F;
        let g = bytes[byte_idx] & 0x0F;
        let b = (bytes[byte_idx + 1] >> 4) & 0x0F;
        (r, g, b)
      } else {
        // Odd pixel: [R1G1, B1R2, G2B2] -> R2, G2, B2
        let r = bytes[byte_idx + 1] & 0x0F;
        let g = (bytes[byte_idx + 2] >> 4) & 0x0F;
        let b = bytes[byte_idx + 2] & 0x0F;
        (r, g, b)
      }
    };
    
    // Verify that each original pixel becomes a 2x2 block in the scaled version
    for orig_y in [0, 3, 7].iter() {
      for orig_x in [0, 3, 7].iter() {
        let orig_pixel_idx = orig_y * 8 + orig_x;
        let orig_pixel = get_12bit_pixel(&bytes_1x, orig_pixel_idx);
        
        // Check the 2x2 block
        for dy in 0..2 {
          for dx in 0..2 {
            let scaled_x = orig_x * 2 + dx;
            let scaled_y = orig_y * 2 + dy;
            let scaled_pixel_idx = scaled_y * 16 + scaled_x;
            let scaled_pixel = get_12bit_pixel(&bytes_2x, scaled_pixel_idx);
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "12-bit: Pixel mismatch at original ({}, {}) -> scaled ({}, {}). Original RGB: {:?}, Scaled RGB: {:?}",
              orig_x, orig_y, scaled_x, scaled_y, orig_pixel, scaled_pixel
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_draw_char_scaled_12bit_asymmetric_pattern_verification() {
    // Test asymmetric scaling with 12-bit mode
    let ramwr_1x = draw_char('5', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK);
    assert!(ramwr_1x.is_some());
    let mut ramwr_1x = ramwr_1x.unwrap();
    let bytes_1x: Vec<u8> = ramwr_1x.parm_bytes().into_iter().collect();
    
    // Use 4x2 scale to ensure even pixel count (32x16 = 512 pixels, which is even)
    let ramwr_4x2 = draw_char_scaled('5', Pixel::<Pixel12>::WHITE, Pixel::<Pixel12>::BLACK, 4, 2);
    assert!(ramwr_4x2.is_some());
    let mut ramwr_4x2 = ramwr_4x2.unwrap();
    let bytes_4x2: Vec<u8> = ramwr_4x2.parm_bytes().into_iter().collect();
    
    // Original: 8x8 = 64 pixels = 96 bytes
    assert_eq!(bytes_1x.len(), 96);
    // Scaled: 32x16 = 512 pixels = 768 bytes
    assert_eq!(bytes_4x2.len(), 768);
    
    // Helper function to extract a pixel from 12-bit data
    let get_12bit_pixel = |bytes: &[u8], pixel_idx: usize| -> (u8, u8, u8) {
      let byte_idx = (pixel_idx / 2) * 3;
      if pixel_idx % 2 == 0 {
        let r = (bytes[byte_idx] >> 4) & 0x0F;
        let g = bytes[byte_idx] & 0x0F;
        let b = (bytes[byte_idx + 1] >> 4) & 0x0F;
        (r, g, b)
      } else {
        let r = bytes[byte_idx + 1] & 0x0F;
        let g = (bytes[byte_idx + 2] >> 4) & 0x0F;
        let b = bytes[byte_idx + 2] & 0x0F;
        (r, g, b)
      }
    };
    
    // Verify that each original pixel becomes a 4x2 block
    for orig_y in [0, 4, 7].iter() {
      for orig_x in [0, 4, 7].iter() {
        let orig_pixel_idx = orig_y * 8 + orig_x;
        let orig_pixel = get_12bit_pixel(&bytes_1x, orig_pixel_idx);
        
        // Check the 4x2 block (4 wide, 2 tall)
        for dy in 0..2 {
          for dx in 0..4 {
            let scaled_x = orig_x * 4 + dx;
            let scaled_y = orig_y * 2 + dy;
            let scaled_pixel_idx = scaled_y * 32 + scaled_x;
            let scaled_pixel = get_12bit_pixel(&bytes_4x2, scaled_pixel_idx);
            
            assert_eq!(
              orig_pixel, scaled_pixel,
              "12-bit 4x2: Pixel mismatch at original ({}, {}) -> scaled ({}, {})",
              orig_x, orig_y, scaled_x, scaled_y
            );
          }
        }
      }
    }
  }
}

// Made with Bob
