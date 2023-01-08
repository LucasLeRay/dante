use std::collections::HashMap;

use pyo3::{pyclass, pymethods};

use crate::tokenization::token::Word;
use crate::utils::wrappers::wrap_sentence;

// MLE stands for "Maximum Likelihood Estimation"
#[pyclass]
pub struct MLE {
    n: u32,
    text: Vec<Word>,
    vocabulary: Vec<Word>
}

impl MLE {
    fn new_(n: u32) -> Self {
        if n == 0 {
            panic!("number of grams should be at least 1.");
        }

        MLE {
            n,
            text: vec![],
            vocabulary: vec![]
        }
    }

    fn fit_(&mut self, text: &Vec<Word>, vocabulary: &Vec<Word>) {
        self.text = wrap_sentence(text, self.n);
        self.vocabulary = vocabulary.to_vec();
    }

    fn generate_word_(&self) -> Word {
        let mut word_frequency: HashMap<Word, f32> = HashMap::new();

        let context: Vec<Word> = self.text[
            (self.text.len()-self.n as usize+1)..(self.text.len())
        ].to_vec();
        let context_count: u32 = self.count_of_sequence(&context);

        for word in self.vocabulary.iter() {
            let mut full_sentence: Vec<Word> = context.to_owned();
            full_sentence.push(word.to_owned());

            let ngram_count: u32 = self.count_of_sequence(&full_sentence);
            let frequency: f32 = ngram_count as f32 / context_count as f32;

            word_frequency.insert(word.to_owned(), frequency);
        }

        println!("{:?}", word_frequency);

        word_frequency
            .iter()
            .max_by(|a, b| f32::total_cmp(a.1, b.1))
            .map(|(k, _v)| k.to_owned())
            .unwrap()
    }

    fn count_of_sequence(&self, sequence: &Vec<Word>) -> u32 {
        let mut count: u32 = 0;
        
        for (index, _word) in self.text.iter().enumerate() {
            if index > self.text.len() - self.n as usize {
                break;
            }
            if sequence.iter()
            .zip(&self.text[index..index+self.n as usize])
            .all(|(a,b)| a == b) {
                count += 1;
            }
        }
        println!("sequence({:?}), count: {}", sequence, count);

        count
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

    fn generate_word(&mut self) -> Word {
        MLE::generate_word_(&self)
    }
}
