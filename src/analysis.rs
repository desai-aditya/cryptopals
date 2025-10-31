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
    s.chars().filter(|c| c.is_ascii_lowercase()).collect()
}

pub fn count_character_frequencies(corpus: String) -> HashMap<char, f64> {
    let corpus = sanitize_string(corpus);
    let n = corpus.len() as f64;
    let count_m = count_characters(corpus);
    let mut freq_m = HashMap::new();

    for (k, v) in count_m {
        freq_m.insert(k, (v as f64) / n);
    }

    freq_m
}

pub fn score(m1: &HashMap<char, f64>, m2: &HashMap<char, f64>) -> f64 {
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
