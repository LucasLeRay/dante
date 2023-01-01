use itertools::Itertools;
use regex::Regex;

use crate::tokenization::{token::{Token, Word}, special_tokens::UNK, pre_tokenizers::PreTokenizerKind};

pub trait Tokenizer {
    fn new_(vocabulary: Option<&Vec<Word>>, pre_tokenizers: Option<&Vec<PreTokenizerKind>>) -> Self;

    fn transform_(&self, corpus: &str) -> Vec<Token>;

    fn fit_(&mut self, _corpus: &str) {}

    fn fit_transform_(&mut self, corpus: &str) -> Vec<Token> {
        self.fit_(corpus);
        self.transform_(corpus)
    }

    fn split(&self, corpus: &str) -> Vec<Token> {
        let re = Regex::new(r"\S+").unwrap();
        re.find_iter(corpus).map(|mat| Token{word: mat.as_str().to_owned()}).collect()
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
