use regex::Regex;

use crate::tokenization::token::{Token, Word};

pub fn split_token(token: &Token, re: &Regex, keep_sep: bool) -> Vec<Token> {
    let mut words: Vec<Word> = Vec::new();
    let word: Word = token.word.to_string();
    let mut cursor = 0;

    for m in re.find_iter(&word) {
        if cursor != m.start() {
            words.push(word[cursor..m.start()].to_string());
        }
        if keep_sep {
            words.push(m.as_str().to_string());
        }
        cursor = m.end();
    }
    if cursor < word.len() {
        words.push(word[cursor..].to_string())
    }

    words.iter().map(|w| Token{word: w.to_string()}).collect()
}
