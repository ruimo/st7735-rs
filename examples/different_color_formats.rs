use st7735_rs::color_format::{Pixel, Pixel12, Pixel16, Pixel18};

fn main() {
    println!("=== 12-bit Color Format (RGB 4:4:4) ===");
    let red12 = Pixel::<Pixel12>::RED;
    println!("RED: R={}, G={}, B={} (max: 15)", red12.r, red12.g, red12.b);
    
    let green12 = Pixel::<Pixel12>::GREEN;
    println!("GREEN: R={}, G={}, B={} (max: 15)", green12.r, green12.g, green12.b);
    
    let blue12 = Pixel::<Pixel12>::BLUE;
    println!("BLUE: R={}, G={}, B={} (max: 15)", blue12.r, blue12.g, blue12.b);
    
    println!("\n=== 16-bit Color Format (RGB 5:6:5) ===");
    let red16 = Pixel::<Pixel16>::RED;
    println!("RED: R={}, G={}, B={} (R,B max: 31, G max: 63)", red16.r, red16.g, red16.b);
    
    let green16 = Pixel::<Pixel16>::GREEN;
    println!("GREEN: R={}, G={}, B={} (R,B max: 31, G max: 63)", green16.r, green16.g, green16.b);
    
    let blue16 = Pixel::<Pixel16>::BLUE;
    println!("BLUE: R={}, G={}, B={} (R,B max: 31, G max: 63)", blue16.r, blue16.g, blue16.b);
    
    println!("\n=== 18-bit Color Format (RGB 6:6:6) ===");
    let red18 = Pixel::<Pixel18>::RED;
    println!("RED: R={}, G={}, B={} (max: 63)", red18.r, red18.g, red18.b);
    
    let green18 = Pixel::<Pixel18>::GREEN;
    println!("GREEN: R={}, G={}, B={} (max: 63)", green18.r, green18.g, green18.b);
    
    let blue18 = Pixel::<Pixel18>::BLUE;
    println!("BLUE: R={}, G={}, B={} (max: 63)", blue18.r, blue18.g, blue18.b);
    
    println!("\nDifferent color formats example completed successfully!");
}

// Made with Bob
