use std::collections::HashMap;

use itertools::Itertools;
use pyo3::{pyclass, pymethods};
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use crate::tokenization::special_tokens;
use crate::tokenization::token::Word;
use crate::utils::wrappers::wrap_sentence;

// MLE stands for "Maximum Likelihood Estimation"
#[pyclass]
pub struct MLE {
    n: u32,
    ngrams: Vec<Vec<Word>>,
    context_counter: HashMap<Vec<Word>, Vec<Word>>,
    vocabulary: Vec<Word>
}

impl MLE {
    fn new_(n: u32) -> Self {
        if n < 2 {
            panic!("number of grams should be at least 2.");
        }

        MLE {
            n,
            ngrams: vec![],
            context_counter: HashMap::new(),
            vocabulary: vec![]
        }
    }

    // transform text into n-grams and add the context counts in a lookup table
    fn fit_(&mut self, text: &Vec<Word>, vocabulary: &Vec<Word>) {
        self.ngrams = ngrams(text, self.n, true);
        for ngram in self.ngrams.iter() {
            let (word, context) = ngram.split_last().unwrap();
            self.context_counter.entry(context.to_vec()).or_insert(vec![]).push(word.to_owned());
        }
        self.vocabulary = vocabulary.to_vec();
    }

    // generate a word using the provided sentence.
    fn generate_word_(&self, sentence: &Vec<Word>) -> Word {
        let context: Vec<Word> = sentence[(sentence.len() + 1 - self.n as usize)..].to_vec();

        let possible_words: Vec<Word> = match self.context_counter.get(&context) {
            Some(w) => w.to_owned(),
            None => vec![]
        };

        if possible_words.len() == 0 {
            return special_tokens::UNK.to_owned();
        }

        let mut rng = thread_rng();
        let choices: Vec<(&Word, f32)> = possible_words.iter().map(|w| (w, self.score(&context, &w))).collect();
        let dist = WeightedIndex::new(choices.iter().map(|choice| choice.1)).unwrap();

        choices[dist.sample(&mut rng)].0.to_owned()
    }

    // get the frequency of a word after a specific context.
    fn score(&self, context: &Vec<Word>, word: &Word) -> f32 {
        let possible_words: Vec<Word> = match self.context_counter.get(context) {
            Some(w) => w.to_owned(),
            None => vec![]
        };
        let word_count: usize = possible_words.iter().filter(|possible_word| *possible_word == word).count();

        if possible_words.len() == 0 || word_count == 0 {
            return 0.0;
        }

        word_count as f32 / possible_words.len() as f32
    }

    // compute the entropy of the model given a test set
    fn entropy_(&self, test_set: &Vec<Word>) -> f32 {
        let ngrams: Vec<Vec<Word>> = ngrams(test_set, self.n, true);
        let mut total_score: f32 = 0.0;

        for ngram in ngrams.iter() {
            let (word, context) = ngram.split_last().unwrap();
            let score = self.score(&context.to_vec(), word);
            total_score += if score == 0.0 {0.0} else {f32::log2(score)};
        }
        
        -1.0 * (total_score / ngrams.len() as f32)
    }

    // compute the perplexity of the model given a test set
    fn perplexity_(&self, test_set: &Vec<Word>) -> f32 {
        f32::powf(2.0, self.entropy_(test_set))
    }
}

#[pymethods]
impl MLE {
    #[new]
    fn new(n: u32) -> Self {
        MLE::new_(n)
    }

    fn fit(&mut self, text: Vec<Word>, vocabulary: Vec<Word>) {
        MLE::fit_(self, &text, &vocabulary)
    }

    fn generate_word(&mut self, sentence: Vec<Word>) -> Word {
        MLE::generate_word_(&self, &sentence)
    }

    fn entropy(&self, test_set: Vec<Word>) -> f32 {
        MLE::entropy_(&self, &test_set)
    }

    fn perplexity(&self, test_set: Vec<Word>) -> f32 {
        MLE::perplexity_(&self, &test_set)
    }
}

// get ngrams from list of words
fn ngrams(words: &Vec<Word>, n: u32, padding: bool) -> Vec<Vec<Word>> {
    wrap_sentence(words, n - 1, padding, padding)
        .as_slice()
        .windows(n as usize)
        .map(|toto| toto.to_vec())
        .collect_vec()
}
