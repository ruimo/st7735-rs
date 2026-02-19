//! Example demonstrating the use of Ramwr::draw_rect with a function
//!
//! This example shows how to create dynamic pixel patterns using a function
//! that computes pixel colors based on (x, y) coordinates.

use st7735_rs::color_format::{Pixel, Pixel16};
use st7735_rs::command::{Command, Ramwr};

fn main() {
    println!("=== draw_rect Function Example ===\n");

    // Example 1: Gradient pattern
    println!("1. Gradient Pattern (2x2):");
    let mut gradient = Ramwr::draw_rect(0..=1, 0..=1, |x, y| {
        let intensity = ((x + y) * 8) as u8;
        Pixel::<Pixel16>::new(intensity, intensity, intensity)
    });
    
    let bytes: Vec<u8> = gradient.parm_bytes().into_iter().collect();
    println!("   Generated {} bytes", bytes.len());
    println!("   Bytes: {:02X?}\n", bytes);

    // Example 2: Checkerboard pattern
    println!("2. Checkerboard Pattern (4x4):");
    let mut checkerboard = Ramwr::draw_rect(0..=3, 0..=3, |x, y| {
        if (x + y) % 2 == 0 {
            Pixel::<Pixel16>::WHITE
        } else {
            Pixel::<Pixel16>::BLACK
        }
    });
    
    let bytes: Vec<u8> = checkerboard.parm_bytes().into_iter().collect();
    println!("   Generated {} bytes for 16 pixels", bytes.len());

    // Example 3: Horizontal gradient
    println!("\n3. Horizontal Gradient (8x1):");
    let mut h_gradient = Ramwr::draw_rect(0..=7, 0..=0, |x, _y| {
        let red = (x * 4) as u8;
        Pixel::<Pixel16>::new(red, 0, 0)
    });
    
    let bytes: Vec<u8> = h_gradient.parm_bytes().into_iter().collect();
    println!("   Generated {} bytes", bytes.len());

    // Example 4: Circular pattern (distance from center)
    println!("\n4. Circular Pattern (10x10):");
    let mut circular = Ramwr::draw_rect(0..=9, 0..=9, |x, y| {
        let cx = 5.0;
        let cy = 5.0;
        let dx = (x as f32 - cx).abs();
        let dy = (y as f32 - cy).abs();
        let distance = (dx * dx + dy * dy).sqrt();
        let intensity = ((5.0 - distance.min(5.0)) * 6.0) as u8;
        Pixel::<Pixel16>::new(intensity, intensity, intensity)
    });
    
    let bytes: Vec<u8> = circular.parm_bytes().into_iter().collect();
    println!("   Generated {} bytes for 100 pixels", bytes.len());

    println!("\n=== Key Benefits ===");
    println!("✓ No memory allocation for pixel data");
    println!("✓ Pixels computed on-demand as bytes are consumed");
    println!("✓ Efficient for large displays or complex patterns");
    println!("✓ Function composition enables flexible pixel generation");
}

// Made with Bob
