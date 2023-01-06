use regex::Regex;

use crate::tokenization::token::Token;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;
use crate::tokenization::pre_tokenizers::utils::split_token;

pub struct Punctuation;

impl PreTokenizer for Punctuation {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        let mut new_tokens: Vec<Token> = Vec::new();
        for token in tokens.iter() {
            let re = Regex::new(r"[.,:;\-!?]+").unwrap();
            new_tokens.extend(split_token(token, &re, true));
        }

        new_tokens
    }
}
