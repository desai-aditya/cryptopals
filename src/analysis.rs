use std::collections::HashMap;

fn analyze_character_frequencies(corpus: String) -> HashMap<char, u64> {
    let mut m = HashMap::new();
    for c in corpus.chars() {
        let e = m.entry(c).or_insert(0);
        *e += 1;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_frequencies() {
        let s = String::from("hello world");
        let out = analyze_character_frequencies(s);
        let expected = HashMap::from([
            ('d', 1),
            ('l', 3),
            ('e', 1),
            (' ', 1),
            ('r', 1),
            ('o', 2),
            ('w', 1),
            ('h', 1),
        ]);
        println!("{:?} ", out);
        assert_eq!(expected, out);
    }
}
