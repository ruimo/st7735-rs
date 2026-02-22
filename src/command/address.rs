//! Memory address control commands
//!
//! This module contains commands for controlling display orientation,
//! refresh order, and color format.

use core::time::Duration;
use super::Command;
use modular_bitfield::{Specifier, bitfield};

/// Address order for row/column addressing
#[derive(Specifier)]
pub enum AddressOrder {
  /// Normal order (left-to-right or top-to-bottom)
  Normal,
  /// Reverse order (right-to-left or bottom-to-top)
  Reverse,
}

/// Vertical refresh order
#[derive(Specifier)]
pub enum VerticalRefreshOrder {
  /// Refresh from top to bottom
  TopToBottom,
  /// Refresh from bottom to top
  BottomToTop,
}

/// Horizontal refresh order
#[derive(Specifier)]
pub enum HorizontalRefreshOrder {
  /// Refresh from left to right
  LeftToRight,
  /// Refresh from right to left
  RightToLeft,
}

/// RGB/BGR color order
#[derive(Specifier)]
pub enum RgbBgb {
  /// RGB color order
  Rgb,
  /// BGR color order
  Bgr,
}

/// Memory Data Access Control command (0x36)
///
/// Controls the display orientation, refresh order, and color format.
/// This command is essential for setting up portrait/landscape modes
/// and display rotation.
///
/// # Fields
///
/// * `my` - Row address order (Y-axis mirroring)
/// * `mx` - Column address order (X-axis mirroring)
/// * `exchange_row_col` - Row/column exchange (for rotation)
/// * `ml` - Vertical refresh order
/// * `rgb_bgr` - RGB/BGR color order
/// * `mh` - Horizontal refresh order
///
/// # Example
///
/// ```
/// use st7735_rs::command::{address::Madctl, AddressOrder, VerticalRefreshOrder, HorizontalRefreshOrder, RgbBgb};
///
/// // Portrait mode (0° rotation)
/// let madctl = Madctl::new()
///     .with_my(AddressOrder::Normal)
///     .with_mx(AddressOrder::Normal)
///     .with_exchange_row_col(false)
///     .with_ml(VerticalRefreshOrder::TopToBottom)
///     .with_rgb_bgr(RgbBgb::Rgb)
///     .with_mh(HorizontalRefreshOrder::LeftToRight);
///
/// // Landscape mode (90° clockwise rotation)
/// let madctl = Madctl::new()
///     .with_my(AddressOrder::Normal)
///     .with_mx(AddressOrder::Reverse)
///     .with_exchange_row_col(true);
/// ```
#[bitfield]
#[derive(Copy, Clone)]
pub struct Madctl {
  #[skip]
  reserved: modular_bitfield::prelude::B2,
  #[bits = 1]
  pub mh: HorizontalRefreshOrder,
  #[bits = 1]
  pub rgb_bgr: RgbBgb,
  #[bits = 1]
  pub ml: VerticalRefreshOrder,
  pub exchange_row_col: bool,
  #[bits = 1]
  pub mx: AddressOrder,
  #[bits = 1]
  pub my: AddressOrder,
}

impl Command for Madctl {
  fn cmd_byte(&self) -> u8 {
    0x36
  }

  fn parm_bytes(&mut self) -> impl IntoIterator<Item = u8> {
    (*self).into_bytes().into_iter()
  }
  
  fn post_delay(&self) -> Duration {
    Duration::ZERO
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_madctl_cmd_byte() {
    let madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    assert_eq!(madctl.cmd_byte(), 0x36);
  }

  #[test]
  fn test_madctl_parm_bytes_my_reverse() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Reverse)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b1000_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_mx_reverse() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Reverse)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0100_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_exchange_row_col() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(true)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0010_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_ml_bottom_to_top() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::BottomToTop)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0001_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_bgr() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Bgr)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_1000]);
  }

  #[test]
  fn test_madctl_parm_bytes_mh_right_to_left() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::RightToLeft);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0100]);
  }

  #[test]
  fn test_madctl_parm_bytes_all_set() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Reverse)
      .with_mx(AddressOrder::Reverse)
      .with_exchange_row_col(true)
      .with_ml(VerticalRefreshOrder::BottomToTop)
      .with_rgb_bgr(RgbBgb::Bgr)
      .with_mh(HorizontalRefreshOrder::RightToLeft);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b1111_1100]);
  }

  #[test]
  fn test_madctl_parm_bytes_portrait_mode() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_landscape_mode() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Reverse)
      .with_exchange_row_col(true)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0110_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_rotation_0() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0000_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_rotation_90() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Reverse)
      .with_exchange_row_col(true)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b0110_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_rotation_180() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Reverse)
      .with_mx(AddressOrder::Reverse)
      .with_exchange_row_col(false)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b1100_0000]);
  }

  #[test]
  fn test_madctl_parm_bytes_rotation_270() {
    let mut madctl = Madctl::new()
      .with_my(AddressOrder::Reverse)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(true)
      .with_ml(VerticalRefreshOrder::TopToBottom)
      .with_rgb_bgr(RgbBgb::Rgb)
      .with_mh(HorizontalRefreshOrder::LeftToRight);
    let params: Vec<u8> = madctl.parm_bytes().into_iter().collect();
    assert_eq!(params, vec![0b1010_0000]);
  }

  #[test]
  fn test_madctl_post_delay() {
    let madctl = Madctl::new()
      .with_my(AddressOrder::Normal)
      .with_mx(AddressOrder::Normal)
      .with_exchange_row_col(false);
    assert_eq!(madctl.post_delay(), Duration::ZERO);
  }
}

// Made with Bob
