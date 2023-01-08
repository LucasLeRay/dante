use regex::Regex;

use crate::tokenization::token::Token;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;
use crate::tokenization::pre_tokenizers::utils::split_token;

pub struct WhiteSpace;

impl PreTokenizer for WhiteSpace {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        let mut new_tokens: Vec<Token> = Vec::new();
        for token in tokens.iter() {
            let re = Regex::new(r"\s+").unwrap();
            new_tokens.extend(split_token(token, &re, false));
        }

        new_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace() {
        let whitespace: WhiteSpace = WhiteSpace{};

        let input: Vec<Token> = vec![Token{word: "Hello World!".to_string()}];
        let expected: Vec<Token> = vec![
            Token{word: "Hello".to_string()},
            Token{word: "World!".to_string()}
        ];

        let result: Vec<Token> = whitespace.pre_tokenize(&input);
        assert_eq!(result, expected);
    }
}
