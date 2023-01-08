use crate::tokenization::{
    special_tokens,
    token::Word
};

pub fn wrap_sentence(sentence: &Vec<Word>, n: u32) -> Vec<Word> {
    let mut wrapped: Vec<Word> = Vec::new();

    for _ in 0..n {
        wrapped.push(special_tokens::SOS.to_string());
    }
    for word in sentence.iter() {
        wrapped.push(word.to_string());
    }

    wrapped
}
