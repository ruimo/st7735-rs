use st7735_rs::color_format::{Pixel, Pixel16};
use st7735_rs::command::{Command, Caset, Raset, Ramwr};

fn main() {
    // Set drawing area (X: 0-9, Y: 0-9)
    let mut caset = Caset::new(0..10);
    println!("=== Caset (Column Address Set) ===");
    println!("Command byte: 0x{:02X}", caset.cmd_byte());
    println!("Parameter bytes: {:?}", caset.parm_bytes().into_iter().collect::<Vec<_>>());
    println!("Post delay: {:?}", caset.post_delay());
    println!();
    
    let mut raset = Raset::new(0..10);
    println!("=== Raset (Row Address Set) ===");
    println!("Command byte: 0x{:02X}", raset.cmd_byte());
    println!("Parameter bytes: {:?}", raset.parm_bytes().into_iter().collect::<Vec<_>>());
    println!("Post delay: {:?}", raset.post_delay());
    println!();
    
    // Fill a 10x10 rectangle with red
    let mut ramwr = Ramwr::fill_rect(0..=9, 0..=9, Pixel::<Pixel16>::RED);
    println!("=== Ramwr (Memory Write) ===");
    println!("Command byte: 0x{:02X}", ramwr.cmd_byte());
    let bytes: Vec<u8> = ramwr.parm_bytes().into_iter().collect();
    println!("Parameter bytes count: {} bytes", bytes.len());
    println!("First 20 bytes: {:02X?}", &bytes[..20.min(bytes.len())]);
    println!("Post delay: {:?}", ramwr.post_delay());
    
    println!("\nDrawing rectangle example completed successfully!");
}

// Made with Bob
