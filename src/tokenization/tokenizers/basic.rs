use pyo3::{pyclass, pymethods};

use crate::tokenization::{
    tokenizers::tokenizer::Tokenizer,
    token::{
        Word, Token
    },
    pre_tokenizers::PreTokenizerKind
};

#[pyclass]
pub struct BasicTokenizer {
    pre_tokenizers: Vec<PreTokenizerKind>,
    pub vocabulary: Vec<Word>,
}

impl BasicTokenizer {
    fn pre_process(&self, corpus: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = self.split(&corpus);
        for pre_tokenizer in self.pre_tokenizers.iter() {
            tokens = pre_tokenizer.get_processor().pre_tokenize(&tokens);
        }
        tokens
    }
}

impl Tokenizer for BasicTokenizer {
    fn new(vocabulary: Option<&Vec<Word>>, pre_tokenizers: Option<&Vec<PreTokenizerKind>>) -> Self {
        let vocabulary = match vocabulary {
            Some(v) => v.to_vec(),
            None => vec![]
        };

        let pre_tokenizers = match pre_tokenizers {
            Some(t) => t.to_vec(),
            None => vec![]
        };

        BasicTokenizer {
            pre_tokenizers,
            vocabulary
        }
    }

    fn fit(&mut self, corpus: &str) {
        let tokens: Vec<Token> = self.pre_process(corpus);
        self.vocabulary = self.extract_vocabulary(&tokens);
    }

    fn transform(&self, corpus: &str) -> Vec<Token> {
        let tokens: Vec<Token> = self.pre_process(corpus);
        self.tokenize(&self.vocabulary, &tokens)
    }
}

#[pymethods]
impl BasicTokenizer {
    #[new]
    fn new_py(vocabulary: Vec<Word>, pre_tokenizers: Vec<PreTokenizerKind>) -> Self {
        println!("coucou");
        BasicTokenizer::new(Some(&vocabulary), Some(&pre_tokenizers))
    }

    fn fit_py(&mut self, corpus: &str) {
        BasicTokenizer::fit(self, corpus)
    }

    fn transform_py(&self, corpus: &str) -> Vec<Token> {
        BasicTokenizer::transform(self, corpus)
    }

    fn fit_transform_py(&mut self, corpus: &str) -> Vec<Token> {
        BasicTokenizer::fit_transform(self, corpus)
    }
}
