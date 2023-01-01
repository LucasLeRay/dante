use regex::Regex;

use crate::tokenization::token::{Token, Word};
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;

pub struct Punctuation;

fn split_token(token: &Token, re: &Regex) -> Vec<Token> {
    let mut words: Vec<Word> = Vec::new();
    let word: Word = token.word.to_string();
    let mut cursor = 0;

    for m in re.find_iter(&word) {
        if cursor != m.start() {
            words.push(word[cursor..m.start()].to_string());
        }
        words.push(m.as_str().to_string());
        cursor = m.end();
    }
    if cursor < word.len() {
        words.push(word[cursor..].to_string())
    }

    words.iter().map(|w| Token{word: w.to_string()}).collect()
}

impl PreTokenizer for Punctuation {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        let mut new_tokens: Vec<Token> = Vec::new();
        for token in tokens.iter() {
            let re = Regex::new(r"[.,:;\-!?]+").unwrap();
            new_tokens.extend(split_token(token, &re));
        }

        new_tokens
    }
}
