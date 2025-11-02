use crate::corpus::*;
use crate::hex::*;
use std::collections::HashMap;

fn count_characters(corpus: String) -> HashMap<char, u64> {
    let mut m = HashMap::new();
    for c in corpus.chars() {
        let e = m.entry(c).or_insert(0);
        *e += 1;
    }
    m
}

fn sanitize_string(s: String) -> String {
    s.chars().filter(|c| c.is_digit(10)  || *c == ' ' || c.is_ascii_lowercase()).collect()
}

pub fn calculate_character_frequencies(corpus: String) -> HashMap<char, f64> {
    let corpus = sanitize_string(corpus);
    let n = corpus.len() as f64;
    let count_m = count_characters(corpus);
    let mut freq_m = HashMap::new();

    for (k, v) in count_m {
        freq_m.insert(k, (v as f64) / n);
    }

    freq_m
}

pub fn score_english(m1: &HashMap<char, f64>, m2: &HashMap<char, f64>) -> f64 {
    let mut sum: f64 = 0.0;

    // we need an invalid variable otherwise when m1 and m2 are disjoint,
    // the score is 0 which is wrong
    let mut invalid = true;

    for (k, v) in m1 {
        if let Some(val) = m2.get(&k) {
            sum += (v - val).abs();
            invalid = false;
        }
    }

    if invalid {
        sum = f64::MAX;
    }
    sum
}

pub fn break_xor_cipher(input: String, corpus: &Corpus) -> (String, u8, f64) {
    // for all 255 different u8 bytes - do an xor with the string
    // check the loss against the corpus
    // take the one with minimum loss

    let hex_input = string_to_hex(input);

    let mut min_score = f64::MAX;
    let mut xor_byte = 0;
    let mut xor_string = String::new();
    let _ = (0..=255)
        .map(|i| {
            let test_hex = hex_input.clone();
            let test_hex = hex_xor_byte(&test_hex, i);
            let test_string = String::from_utf8_lossy(&test_hex);
            let string_freq = calculate_character_frequencies(test_string.to_string());
            let t = score_english(&string_freq, &corpus.get_character_frequencies());
            if t < min_score {
                min_score = t;
                xor_byte = i;
                xor_string = test_string.to_string();
                //                println!("{} {} {}", minidx, min_score, minstring);
            }
        })
        .collect::<Vec<_>>();
    // println!("Final string is {}", xor_string);
    // println!("Final string is at index {}", xor_byte);
    // println!("Final score is {}", min_score);
    (xor_string, xor_byte, min_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_frequencies() {
        let s = String::from("Hello world");
        let out = count_characters(s);
        let expected = HashMap::from([
            ('d', 1),
            ('H', 1),
            ('l', 3),
            ('e', 1),
            ('r', 1),
            ('o', 2),
            ('w', 1),
            (' ', 1),
        ]);
        println!("{:?} ", out);
        assert_eq!(expected, out);
    }
}
