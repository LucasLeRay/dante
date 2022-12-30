use std::collections::HashMap;

use pyo3::{pyclass, pymethods};

use crate::tokenization::special_tokens;

#[pyclass]
pub struct BPE {
    vocabulary: HashMap<String, u32>,
    bpe_codes: HashMap<Vec<String>, u32>,
    k: u32
}

#[pymethods]
impl BPE {
    #[new]
    pub fn new(k: u32) -> BPE {
        BPE {
            vocabulary: HashMap::new(),
            bpe_codes: HashMap::new(),
            k
        }
    }

    pub fn fit(&mut self, corpus: &str) {
        self.vocabulary = self.get_words_count(corpus);
        
        for index in 0..self.k {
            let pair_stats: HashMap<Vec<String>, u32> = self.get_pair_stats();
            if pair_stats.is_empty() {
                break;
            }
            let best_pair: &Vec<String> = pair_stats.iter().max_by_key(|x| x.1).unwrap().0;
            self.bpe_codes.insert(best_pair.to_vec(), index);
            self.vocabulary = self.get_new_vocabulary(best_pair);
        }
    }

    pub fn transform(&self, corpus: &str) -> Vec<Vec<String>> {
        corpus
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|word| self.tokenize(word))
            .collect()
    }

    pub fn tokenize(&self, word: &str) -> Vec<String> {
        let mut word: Vec<String> = wrap_word(
            word
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|c| c.to_string())
                .collect()
        );
        
        loop {
            let pairs = self.get_pairs(&word);
            let bpe_pairs: Vec<(&Vec<String>, &u32)> = pairs
            .iter()
            .filter(|pair| self.bpe_codes.contains_key(pair.to_owned()))
            .map(|pair| self.bpe_codes.get_key_value(pair).unwrap())
            .collect();
            
            if bpe_pairs.is_empty() {
                break;
            }
            
            let pair_to_merge = bpe_pairs.iter().min_by_key(|pair| pair.1).unwrap().0;
            word = self.create_new_word(word, pair_to_merge);
        }

        word
    }
}

impl BPE {
    fn get_pairs(&self, word: &Vec<String>) -> Vec<Vec<String>> {
        let mut pairs: Vec<Vec<String>> = Vec::new();
        
        for pair in word.windows(2) {
            pairs.push(vec![pair[0].to_string(), pair[1].to_string()]);
        }

        pairs
    }

    fn create_new_word(&self, word: Vec<String>, pair_to_merge: &Vec<String>) -> Vec<String> {
        let mut new_word: Vec<String> = Vec::new();
        let (left, right) = (&pair_to_merge[0], &pair_to_merge[1]);

        let mut i = 0;
        while i < word.len() {
            if &word[i] == left && &word[i+1] == right {
                new_word.push(left.to_owned() + right);
                i += 1;
            } else {
                new_word.push(word[i].to_owned())
            }
            i += 1;
        }

        new_word
    }

    fn get_words_count(&self, text: &str) -> HashMap<String, u32> {
        text
            .split(" ")
            .fold(HashMap::new(), |mut words, word| {
                let word: Vec<String> = wrap_word(
                    word
                        .chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                );
                *words.entry(word.join(" ")).or_insert(0) += 1;
                words
            })
    }

    fn get_pair_stats(&self) -> HashMap<Vec<String>, u32> {
        let mut stats: HashMap<Vec<String>, u32> = HashMap::new();
        for (word, frequency) in self.vocabulary.iter() {
            for pair in word.split(" ").map(|x| x.to_owned()).collect::<Vec<String>>().windows(2) {
                *stats.entry(Vec::from(pair)).or_insert(0) += frequency;
            }
        }

        stats
    }

    fn get_new_vocabulary(&self, best_pair: &Vec<String>) -> HashMap<String, u32> {
        let mut new_vocabulary: HashMap<String, u32> = HashMap::new();

        let pattern = regex::escape(&best_pair.join(" "));
        let replacement = best_pair.join("");
        for (word, frequency) in self.vocabulary.iter() {
            let re = regex::Regex::new(&pattern).unwrap();
            let new_word = re.replace(&word, &replacement).to_string();
            new_vocabulary.insert(new_word, *frequency);
        }

        new_vocabulary
    }
}

pub fn wrap_word(word: Vec<String>) -> Vec<String> {
    let mut wrapped: Vec<String> = Vec::new();

    wrapped.push(special_tokens::SOW.to_string());
    for char in word.iter() {
        wrapped.push(char.to_owned());
    }
    wrapped.push(special_tokens::EOW.to_string());

    wrapped
}
