use itertools::Itertools;

use crate::tokenization::{token::{Token, Word}, special_tokens::UNK};

pub trait Tokenizer {
    fn transform_(&self, corpus: &str) -> Vec<Token>;

    fn fit_(&mut self, _corpus: &str) {}

    fn fit_transform_(&mut self, corpus: &str) -> Vec<Token> {
        self.fit_(corpus);
        self.transform_(corpus)
    }

    fn extract_vocabulary(&self, tokens: &Vec<Token>) -> Vec<Word> {
        tokens.into_iter().map(|t| t.word.to_string()).unique().collect()
    }

    fn tokenize(&self, vocabulary: &Vec<Word>, tokens: &Vec<Token>) -> Vec<Token> {
        tokens.iter().map(|t| {
            Token {
                word: 
                if vocabulary.contains(&t.word) {t.word.to_string()}
                else {UNK.to_string()}
            }
        }).collect()
    }
}
