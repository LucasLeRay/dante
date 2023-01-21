use itertools::Itertools;

use crate::tokenization::token::Word;
use crate::utils::wrappers::wrap_sentence;

// get ngrams from list of words
pub fn ngrams(words: &Vec<Word>, n: u32, padding: bool) -> Vec<Vec<Word>> {
    wrap_sentence(words, n - 1, padding, padding)
        .as_slice()
        .windows(n as usize)
        .map(|toto| toto.to_vec())
        .collect_vec()
}
