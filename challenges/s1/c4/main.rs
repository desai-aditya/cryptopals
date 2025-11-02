use cryptopals::analysis::*;
use cryptopals::corpus::*;
use cryptopals::hex::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("challenges")
        .join("s1")
        .join("c4");
    let input = path.join("input");

    let input = fs::read_to_string(input).expect("failed to read file");

    let input = input.trim().to_string();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("pride_and_prejudice.txt");
    let corpus = Corpus::new(path);

    let mut min_score = f64::MAX;
    let mut decrypted_text = String::new();
    for l in input.lines() {
        let (s, b, score) = break_xor_cipher(l.to_owned(), &corpus);
        if score < min_score {
            min_score = score;
            decrypted_text = s;
        }
    }
    println!("Final string is {}", decrypted_text);
    println!("Final score is {}", min_score);

    println!("Yay success!");
}
