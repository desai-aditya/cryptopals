use cryptopals::hex::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("challenges")
        .join("s1")
        .join("c5");
    let input = path.join("input");
    let input = fs::read_to_string(input).expect("failed to read file");

    let key = path.join("key");
    let key = fs::read_to_string(key).expect("failed to read file");

    let expected = path.join("expected");
    let expected = fs::read_to_string(expected).expect("failed to read file");

    let input = input.trim().as_bytes();
    let key = key.trim().as_bytes();
    let expected = expected.trim().to_owned();

    println!("{:?}", key);

    let plain_text = input;
    let encrypted_text = hex_xor_repeating(plain_text, key);
    let hex_encrypted_text = hex_to_string(&encrypted_text);
    println!("{:?}", hex_encrypted_text);

    assert_eq!(expected, hex_encrypted_text);

    println!("Yay success!");
}
