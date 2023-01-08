use crate::tokenization::token::Token;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;

pub struct CaseFold;

impl PreTokenizer for CaseFold {
    fn pre_tokenize(&self, tokens: &Vec<Token>) -> Vec<Token> {
        tokens.iter().map(|token| Token{word: token.word.to_lowercase()}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let case_fold: CaseFold = CaseFold{};

        let input: Vec<Token> = vec![Token{word: "HeLlO".to_string()}];
        let expected: Vec<Token> = vec![Token{word: "hello".to_string()}];

        let result: Vec<Token> = case_fold.pre_tokenize(&input);
        assert_eq!(result, expected);
    }
}
