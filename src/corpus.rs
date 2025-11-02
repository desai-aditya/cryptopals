use crate::analysis::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Corpus {
    path: PathBuf,
    m: HashMap<char, f64>,
}

impl Corpus {
    pub fn new(path: PathBuf) -> Self {
        let mut c = Self {
            path: path,
            m: HashMap::new(),
        };
        c.calculate_character_frequencies();
        c
    }

    pub fn calculate_character_frequencies(&mut self) {
        let corpus = fs::read_to_string(&self.path).expect("failed to read file");
        let corpus = corpus.trim().to_string();
        self.m = calculate_character_frequencies(corpus);
    }

    pub fn get_character_frequencies(&self) -> HashMap<char, f64> {
        self.m.clone()
    }
}
