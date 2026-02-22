//! Address range setting commands
//!
//! This module contains commands for setting the column and row address ranges
//! for memory write operations.

use core::time::Duration;
use core::ops::RangeBounds;
use super::Command;

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
pub(crate) fn range_start<R: RangeBounds<u16>>(range: &R) -> u16 {
  use core::ops::Bound;
  match range.start_bound() {
    Bound::Included(&n) => n,
    Bound::Excluded(&n) => n.saturating_add(1),
    Bound::Unbounded => 0,
  }
}

/// Helper function to extract the end value from a range
pub(crate) fn range_end<R: RangeBounds<u16>>(range: &R) -> u16 {
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
/// use st7735_rs::command::{Command, range::Caset};
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
/// use st7735_rs::command::{Command, range::Raset};
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

#[cfg(test)]
mod tests {
  use super::*;

  // CASET tests
  #[test]
  fn test_caset_cmd_byte() {
    let caset = Caset::new(0..=127);
    assert_eq!(caset.cmd_byte(), 0x2A);
  }

  #[test]
  fn test_caset_parm_bytes_simple() {
    let mut caset = Caset::new(0..=127);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x7F]);
  }

  #[test]
  fn test_caset_parm_bytes_with_offset() {
    let mut caset = Caset::new(10..=50);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x32]);
  }

  #[test]
  fn test_caset_parm_bytes_large_values() {
    let mut caset = Caset::new(256..=512);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x01, 0x00, 0x02, 0x00]);
  }

  #[test]
  fn test_caset_post_delay() {
    let caset = Caset::new(0..=127);
    assert_eq!(caset.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_caset_exclusive_range() {
    let mut caset = Caset::new(10..50);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x31]);
  }

  #[test]
  fn test_caset_range_from() {
    let mut caset = Caset::new(100..);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x64, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_range_to_inclusive() {
    let mut caset = Caset::new(..=200);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC8]);
  }

  #[test]
  fn test_caset_range_to() {
    let mut caset = Caset::new(..200);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC7]);
  }

  #[test]
  fn test_caset_full_range() {
    let mut caset = Caset::new(..);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_single_point() {
    let mut caset = Caset::new(42..=42);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x2A, 0x00, 0x2A]);
  }

  #[test]
  fn test_caset_max_value() {
    let mut caset = Caset::new(65535..=65535);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0xFF, 0xFF, 0xFF, 0xFF]);
  }

  #[test]
  fn test_caset_high_values() {
    let mut caset = Caset::new(1000..=2000);
    let params: Vec<u8> = caset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x03, 0xE8, 0x07, 0xD0]);
  }

  // RASET tests
  #[test]
  fn test_raset_cmd_byte() {
    let raset = Raset::new(0..=159);
    assert_eq!(raset.cmd_byte(), 0x2B);
  }

  #[test]
  fn test_raset_parm_bytes_simple() {
    let mut raset = Raset::new(0..=159);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0x9F]);
  }

  #[test]
  fn test_raset_parm_bytes_with_offset() {
    let mut raset = Raset::new(10..=50);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x32]);
  }

  #[test]
  fn test_raset_parm_bytes_large_values() {
    let mut raset = Raset::new(256..=512);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x01, 0x00, 0x02, 0x00]);
  }

  #[test]
  fn test_raset_post_delay() {
    let raset = Raset::new(0..=159);
    assert_eq!(raset.post_delay(), Duration::from_millis(0));
  }

  #[test]
  fn test_raset_exclusive_range() {
    let mut raset = Raset::new(10..50);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x0A, 0x00, 0x31]);
  }

  #[test]
  fn test_raset_range_from() {
    let mut raset = Raset::new(100..);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x64, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_range_to_inclusive() {
    let mut raset = Raset::new(..=200);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC8]);
  }

  #[test]
  fn test_raset_range_to() {
    let mut raset = Raset::new(..200);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0x00, 0xC7]);
  }

  #[test]
  fn test_raset_full_range() {
    let mut raset = Raset::new(..);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x00, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_single_point() {
    let mut raset = Raset::new(42..=42);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x00, 0x2A, 0x00, 0x2A]);
  }

  #[test]
  fn test_raset_max_value() {
    let mut raset = Raset::new(65535..=65535);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0xFF, 0xFF, 0xFF, 0xFF]);
  }

  #[test]
  fn test_raset_high_values() {
    let mut raset = Raset::new(1000..=2000);
    let params: Vec<u8> = raset.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0x03, 0xE8, 0x07, 0xD0]);
  }
}

// Made with Bob
