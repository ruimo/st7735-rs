use st7735_rs::color_format::{Pixel, Pixel16};

fn main() {
    println!("=== Predefined Colors ===");
    let red = Pixel::<Pixel16>::RED;
    println!("RED: R={}, G={}, B={}", red.r, red.g, red.b);
    
    let green = Pixel::<Pixel16>::GREEN;
    println!("GREEN: R={}, G={}, B={}", green.r, green.g, green.b);
    
    let blue = Pixel::<Pixel16>::BLUE;
    println!("BLUE: R={}, G={}, B={}", blue.r, blue.g, blue.b);
    
    let white = Pixel::<Pixel16>::WHITE;
    println!("WHITE: R={}, G={}, B={}", white.r, white.g, white.b);
    
    let black = Pixel::<Pixel16>::BLACK;
    println!("BLACK: R={}, G={}, B={}", black.r, black.g, black.b);
    
    println!("\n=== Custom Color ===");
    // Custom color (16-bit: R=20, G=40, B=15)
    let custom = Pixel::<Pixel16>::new(20, 40, 15);
    println!("CUSTOM: R={}, G={}, B={}", custom.r, custom.g, custom.b);
    
    println!("\nCustom colors example completed successfully!");
}

// Made with Bob
