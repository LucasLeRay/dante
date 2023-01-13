use crate::tokenization::{
    special_tokens,
    token::Word
};

pub fn wrap_sentence(sentence: &Vec<Word>, n: u32, start: bool, end: bool) -> Vec<Word> {
    let mut wrapped: Vec<Word> = Vec::new();

    if start {
        for _ in 0..n {
            wrapped.push(special_tokens::SOS.to_string());
        }
    }
    for word in sentence.iter() {
        wrapped.push(word.to_string());
    }
    if end {
        for _ in 0..n {
            wrapped.push(special_tokens::EOS.to_string());
        }
    }

    wrapped
}
