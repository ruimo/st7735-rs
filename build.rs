use core::panic;
use std::env;
use std::fs;
use font8x8::UnicodeFonts;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

/// Function to reverse the bits of a byte (left-right flip)
fn reverse_bits(byte: u8) -> u8 {
    let mut result = 0u8;
    for i in 0..8 {
        if byte & (1 << i) != 0 {
            result |= 1 << (7 - i);
        }
    }
    result
}

fn main() {
    let target_chars_string;
    let target_chars = if let Ok(file_path_str) = env::var("ST7735_FONT_PATH") {
        let path = Path::new(&file_path_str);

        // Check if it's an absolute path
        if !path.is_absolute() {
            panic!("ST7735_FONT_PATH must be an absolute path, but got: '{}'", file_path_str);
        }

        if path.exists() {
            // Trigger rebuild when the specified file changes
            println!("cargo:rerun-if-changed={}", file_path_str);

            // Read file contents
            target_chars_string = fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Cannot read the ST7735_FONT_PATH: '{}'", path.display()));
            target_chars_string.trim()
        } else {
            panic!("The file specified in MY_FONT_FILE_PATH does not exist: {}", file_path_str);
        }
    } else {
        // Default
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    };

    // Sort by character order (Unicode order) and remove duplicates for faster search
    let mut chars: Vec<char> = target_chars.chars().collect();
    chars.sort_unstable();
    chars.dedup();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("extracted_font.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    writeln!(f, "/// Auto-generated font data (character, bitmap)").unwrap();
    writeln!(f, "/// Bits are already reversed (left-right flipped)").unwrap();
    writeln!(f, "pub const FONT_DATA: &[(char, [u8; 8])] = &[").unwrap();
    for c in chars {
        if let Some(bitmap) = font8x8::BASIC_FONTS.get(c) {
            // Reverse bits of each byte (left-right flip)
            let reversed: [u8; 8] = [
                reverse_bits(bitmap[0]),
                reverse_bits(bitmap[1]),
                reverse_bits(bitmap[2]),
                reverse_bits(bitmap[3]),
                reverse_bits(bitmap[4]),
                reverse_bits(bitmap[5]),
                reverse_bits(bitmap[6]),
                reverse_bits(bitmap[7]),
            ];
            writeln!(f, "    ({:?}, {:?}),", c, reversed).unwrap();
        }
    }
    writeln!(f, "];").unwrap();

    // Re-run when build.rs itself or features change
    println!("cargo:rerun-if-changed=build.rs");
}
