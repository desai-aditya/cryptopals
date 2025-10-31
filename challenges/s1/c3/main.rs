use cryptopals::analysis::*;
use cryptopals::hex::*;
use std::fs;
use std::path::PathBuf;

fn solve(input: String, corpus: String) {
    // for all 255 different u8 bytes - do an xor with the string
    // check the loss against the corpus
    // take the one with minimum loss

    let corpus_freq = count_character_frequencies(corpus);
    let hex_input = string_to_hex(input);

    let mut minval = f64::MAX;
    let mut minidx = 0;
    let mut minstring = String::new();
    let _ = (0..=255)
        .map(|i| {
            let test_hex = hex_input.clone();
            let test_hex = hex_xor_byte(&test_hex, i);
            let test_string = String::from_utf8_lossy(&test_hex);
            let string_freq = count_character_frequencies(test_string.to_string());
            let t = score(&string_freq, &corpus_freq);
            if t < minval {
                minval = t;
                minidx = i;
                minstring = test_string.to_string();
                //                println!("{} {} {}", minidx, minval, minstring);
            }
        })
        .collect::<Vec<_>>();
    println!("Final string is {}", minstring);
    println!("Final string is at index {}", minidx);
    println!("Final score is {}", minval);
}

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

    let corpus = fs::read_to_string(path).expect("failed to read file");

    let corpus = corpus.trim().to_string();

    solve(input, corpus);

    println!("Yay success!");
}
