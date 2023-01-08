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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization_exclamation() {
        let punctuation: Punctuation = Punctuation{};

        let input: Vec<Token> = vec![Token{word: "Hello!".to_string()}];
        let expected: Vec<Token> = vec![
            Token{word: "Hello".to_string()}, Token{word: "!".to_string()}
        ];

        let result: Vec<Token> = punctuation.pre_tokenize(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenization_ellipsis() {
        let punctuation: Punctuation = Punctuation{};

        let input: Vec<Token> = vec![Token{word: "Hello...".to_string()}];
        let expected: Vec<Token> = vec![
            Token{word: "Hello".to_string()}, Token{word: "...".to_string()}
        ];

        let result: Vec<Token> = punctuation.pre_tokenize(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenization_between_words() {
        let punctuation: Punctuation = Punctuation{};

        let input: Vec<Token> = vec![Token{word: "Hello,Lucas".to_string()}];
        let expected: Vec<Token> = vec![
            Token{word: "Hello".to_string()},
            Token{word: ",".to_string()},
            Token{word: "Lucas".to_string()}
        ];

        let result: Vec<Token> = punctuation.pre_tokenize(&input);
        assert_eq!(result, expected);
    }
}
