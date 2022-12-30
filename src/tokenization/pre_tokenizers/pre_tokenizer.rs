use crate::tokenization::token::Token;

pub trait PreTokenizer {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token>;
}
