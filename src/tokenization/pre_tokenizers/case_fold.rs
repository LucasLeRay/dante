use crate::tokenization::token::Token;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;

pub struct CaseFold;

impl PreTokenizer for CaseFold {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        tokens.iter().map(|token| Token{word: token.word.to_lowercase()}).collect()
    }
}
