use regex::Regex;

use crate::tokenization::token::Token;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;
use crate::tokenization::pre_tokenizers::utils::split_token;

pub struct Whitespace;

impl PreTokenizer for Whitespace {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        let mut new_tokens: Vec<Token> = Vec::new();
        for token in tokens.iter() {
            let re = Regex::new(r"\s+").unwrap();
            new_tokens.extend(split_token(token, &re, false));
        }

        new_tokens
    }
}

