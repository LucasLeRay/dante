use crate::tokenization::token::Token;

pub trait Splitter {
    fn split(&self, corpus: &str) -> Vec<Token>;
}
