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
        self.text = wrap_sentence(text, self.n, true, false);
        self.vocabulary = vocabulary.to_vec();
    }

    fn generate_word_(&self, sentence: &Vec<Word>) -> Word {
        let mut words_frequency: HashMap<Word, f32> = HashMap::new();

        let context: Vec<Word> = self.text[
            (sentence.len()-self.n as usize+1)..(sentence.len())
        ].to_vec();

        for word in self.vocabulary.iter() {
            let mut full_sentence: Vec<Word> = context.to_owned();
            full_sentence.push(word.to_owned());

            let frequency = self.sequence_probability(&full_sentence);
            words_frequency.insert(word.to_owned(), frequency);
        }

        println!("{:?}", words_frequency);

        words_frequency
            .iter()
            .max_by(|a, b| f32::total_cmp(a.1, b.1))
            .unwrap()
            .0.to_owned()
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
        // println!("sequence({:?}), count: {}", sequence, count);

        count
    }

    fn ngram_probability(&self, context: &Vec<Word>, ngram: &Vec<Word>) -> f32 {
        self.count_of_sequence(&ngram) as f32 / self.count_of_sequence(&context) as f32
    }

    // use chain rule of probability to compute the whole probability of a sequence
    // using its ngram.
    fn sequence_probability(&self, sequence: &Vec<Word>) -> f32 {
        let mut probability: f32 = 0.0;

        for i in 0..sequence.len() {
            let start_index: usize = if i < self.n as usize {0} else {i as usize - self.n as usize + 1};
            let word_index: usize = i as usize;
            // println!("start: ({}), end: ({}), test_set: ({:?})", start_index, word_index, sequence);

            let context: Vec<Word> = sequence[start_index..word_index as usize].to_owned();
            let ngram: Vec<Word> = sequence[start_index..word_index+1 as usize].to_owned();
            // println!("context: {:?}, ngram: {:?}", context, ngram);

            probability += f32::log2(self.ngram_probability(&context, &ngram));
        }

        probability
    }

    fn entropy(&self, test_set: &Vec<Word>) -> f32 {
        // return -1 * _mean(
        //     [self.logscore(ngram[-1], ngram[:-1]) for ngram in text_ngrams]
        // )
        (1.0 / self.n as f32) * self.sequence_probability(test_set)
    }

    fn perplexity_(&self, test_set: &Vec<Word>) -> f32 {
        f32::powf(2.0, self.entropy(test_set))
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

    fn perplexity(&self, test_set: Vec<Word>) -> f32 {
        MLE::perplexity_(&self, &test_set)
    }
}
