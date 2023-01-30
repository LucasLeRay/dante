use std::collections::HashMap;

use itertools::Itertools;
use pyo3::{pyclass, pymethods};

use crate::tokenization::token::Word;

#[pyclass]
pub struct NaiveBayesClassifier {
    vocabulary: Vec<Word>,
    // Note that 'priors' and 'likelihood' are expressed as log
    priors: HashMap<String, f32>,
    likelihood: HashMap<String, HashMap<Word, f32>>
}

impl NaiveBayesClassifier {
    fn new_() -> Self {
        NaiveBayesClassifier {
            vocabulary: vec![],
            priors: HashMap::new(),
            likelihood: HashMap::new()
        }
    }

    // 'train_set' is a mapping of documents over the corrsponding class
    fn fit_(&mut self, train_set: &HashMap<String, Vec<Vec<Word>>>) {
        let all_docs: Vec<&Vec<String>> = train_set.values().flatten().collect();
        let n_doc: usize = all_docs.len();
        
        self.vocabulary = all_docs.into_iter()
            .flat_map(|v| v.to_owned())
            .unique()
            .collect();
 
        self.priors = train_set.iter()
            .fold(HashMap::new(), |mut priors, class| {
                let label = class.0.to_owned();
                let n_class = class.1.len();

                let log_priors = f32::log2(n_class as f32 / n_doc as f32);

                priors.insert(label, log_priors);
                priors
            });

        self.likelihood = train_set.iter()
            .fold(HashMap::new(), |mut likelihood, class| {
                let label = class.0.to_owned();
                let class_corpus: Vec<&Word> = class.1.iter().flatten().collect();

                let class_likelihood = self.vocabulary.iter().fold(HashMap::new(), |mut class_likelihood, word| {
                    let count: usize = class_corpus.iter().filter(|w| **w == word).count();

                    let log_likelihood = f32::log2((count as f32 + 1.0) / (class_corpus.len() + self.vocabulary.len()) as f32);
                    class_likelihood.insert(word.to_owned(), log_likelihood);

                    class_likelihood
                });

                likelihood.insert(label, class_likelihood);
                likelihood
            });
    }

    fn predict_(&self, test_set: &Vec<Word>) -> String {
        self.priors.iter()
        .map(|prior| {
            let class = prior.0;
            let class_prior = prior.1;
            let class_likelihood = self.likelihood.get(class).unwrap();

            let sum = test_set.iter().fold(0.0, |mut sum, word| {
                if self.vocabulary.contains(word) {
                    sum += class_likelihood.get(word).unwrap();
                }
                sum
            });

            (class, class_prior + sum)
        })
        .max_by(|x, y| x.1.abs().partial_cmp(&y.1.abs()).unwrap())
        .unwrap()
        .0.to_owned()
    }
}

#[pymethods]
impl NaiveBayesClassifier {
    #[new]
    fn new() -> Self {
        NaiveBayesClassifier::new_()
    }

    fn fit(&mut self, train_set: HashMap<String, Vec<Vec<Word>>>) {
        NaiveBayesClassifier::fit_(self, &train_set)
    }

    fn predict(&self, test_set: Vec<Word>) -> String {
        NaiveBayesClassifier::predict_(self, &test_set)
    }
}
