//! Display control commands
//!
//! This module contains commands for controlling the display state,
//! such as turning the display on and waking from sleep mode.

use core::time::Duration;
use super::Command;

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
/// use st7735_rs::command::{Command, display::Dispon};
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
/// use st7735_rs::command::{Command, display::Slpout};
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dispon_cmd_byte() {
    let dispon = Dispon;
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
    let dispon = Dispon;
    assert_eq!(dispon.post_delay(), Duration::from_millis(120));
  }

  #[test]
  fn test_slpout_cmd_byte() {
    let slpout = Slpout;
    assert_eq!(slpout.cmd_byte(), 0x11);
  }

  #[test]
  fn test_slpout_parm_bytes() {
    let mut slpout = Slpout;
    let params: Vec<u8> = slpout.parm_bytes().into_iter().collect();
    assert_eq!(params, Vec::<u8>::new());
  }

  #[test]
  fn test_slpout_post_delay() {
    let slpout = Slpout;
    assert_eq!(slpout.post_delay(), Duration::from_millis(120));
  }
}

// Made with Bob
