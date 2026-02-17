use st7735_rs::color_format::ColorFormat;
use st7735_rs::command::{Command, Slpout, Dispon, Colmod};

fn main() {
    // 1. Sleep out
    let slpout = Slpout;
    println!("Slpout command byte: 0x{:02X}", slpout.cmd_byte());
    println!("Slpout post delay: {:?}", slpout.post_delay());
    
    // 2. Set color mode (16-bit)
    let mut colmod = Colmod::new(ColorFormat::Bit16);
    println!("Colmod command byte: 0x{:02X}", colmod.cmd_byte());
    println!("Colmod parameter bytes: {:?}", colmod.parm_bytes().into_iter().collect::<Vec<_>>());
    
    // 3. Display ON
    let dispon = Dispon;
    println!("Dispon command byte: 0x{:02X}", dispon.cmd_byte());
    println!("Dispon post delay: {:?}", dispon.post_delay());
    
    println!("\nBasic initialization example completed successfully!");
}

// Made with Bob
