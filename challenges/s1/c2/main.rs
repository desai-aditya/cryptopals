use cryptopals::hex::*;
use cryptopals::base64::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("challenges")
        .join("s1")
        .join("c2");
    let input = path.join("input");
    let expected = path.join("expected");

    let input = fs::read_to_string(input).expect("failed to read file");
    let expected = fs::read_to_string(expected).expect("failed to read file");

    let lines: Vec<&str> = input.lines().collect();

    let mut input = input.split_whitespace();

    let i1 = input.next().unwrap();
    let i2 = input.next().unwrap();

    let i1 = i1.trim().to_string();
    let i2 = i2.trim().to_string();
    let expected = expected.trim().to_string();

    let i1 = string_to_hex(i1);
    let i2 = string_to_hex(i2);

    let xor = hex_xor(&i1, &i2);
    let xor = hex_to_string(&xor);

    assert_eq!(xor, expected);
    println!("Yay success!");
}
