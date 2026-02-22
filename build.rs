use std::env;
use std::fs;
use font8x8::UnicodeFonts;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

/// バイトのビットを左右反転する関数
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
    // 1. 利用者のプロジェクトの Cargo.toml を読み込む
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let manifest_content = fs::read_to_string(manifest_path).unwrap();
    
    // 2. [package.metadata.my_font_lib] セクションを探す
    let manifest: toml::Value = toml::from_str(&manifest_content).unwrap();
    let target_chars = manifest
        .get("package")
        .and_then(|pkg| pkg.get("metadata"))
        .and_then(|meta| meta.get("my_font_lib"))
        .and_then(|lib| lib.get("include_chars"))
        .and_then(|chars| chars.as_str())
        .unwrap_or("0123456789"); // デフォルト値

    // 検索を高速にするため、文字順（Unicode順）にソートして重複を除く
    let mut chars: Vec<char> = target_chars.chars().collect();
    chars.sort_unstable();
    chars.dedup();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("extracted_font.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    writeln!(f, "/// 自動生成されたフォントデータ（文字, ビットマップ）").unwrap();
    writeln!(f, "/// ビットを左右反転済み").unwrap();
    writeln!(f, "pub const FONT_DATA: &[(char, [u8; 8])] = &[").unwrap();
    for c in chars {
        if let Some(bitmap) = font8x8::BASIC_FONTS.get(c) {
            // 各バイトのビットを左右反転
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

    // build.rs 自体や Feature が変わった時に再実行させる
    println!("cargo:rerun-if-changed=build.rs");
}
