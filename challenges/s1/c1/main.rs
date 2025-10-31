use cryptopals::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("challenges")
        .join("s1")
        .join("c1");
    let input = path.join("input");
    let expected = path.join("expected");

    let input = fs::read_to_string(input).expect("failed to read file");
    let expected = fs::read_to_string(expected).expect("failed to read file");

    let input = input.trim().to_string();
    let expected = expected.trim().to_string();

    let hex = string_to_hex(input);
    let b64 = hex_to_b64(&hex);

    assert_eq!(b64, expected);
    println!("Yay success!");
}
