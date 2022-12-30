use regex::Regex;
use crate::tokenization::token::Token;
use crate::tokenization::splitters::splitter::Splitter;

pub struct Whitespace;

impl Splitter for Whitespace {
    fn split(&self, corpus: &str) -> Vec<Token> {
        let re = Regex::new(r"\S+").unwrap();
        re.find_iter(corpus).map(|mat| Token{word: mat.as_str().to_owned()}).collect()
    }
}
