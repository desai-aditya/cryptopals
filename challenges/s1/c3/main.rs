use cryptopals::analysis::*;
use cryptopals::corpus::*;
use cryptopals::hex::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("challenges")
        .join("s1")
        .join("c3");
    let input = path.join("input");

    let input = fs::read_to_string(input).expect("failed to read file");

    let input = input.trim().to_string();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("pride_and_prejudice.txt");
    let corpus = Corpus::new(path);

    let (decrypted_text, _, min_score) = break_xor_cipher(input, &corpus);
    println!("Final string is {}", decrypted_text);
    println!("Final score is {}", min_score);
    println!("Yay success!");
}
