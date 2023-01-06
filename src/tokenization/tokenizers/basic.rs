use pyo3::{pyclass, pymethods, PyResult};

use crate::tokenization::{
    tokenizers::tokenizer::Tokenizer,
    token::{
        Word, Token
    },
    pre_tokenizers::PreTokenizerKind
};

#[pyclass]
pub struct BasicTokenizer {
    pub vocabulary: Vec<Word>,
    pre_tokenizers: Vec<PreTokenizerKind>,
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
    fn new_(vocabulary: Option<&Vec<Word>>, pre_tokenizers: Option<&Vec<PreTokenizerKind>>) -> Self {
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

    fn fit_(&mut self, corpus: &str) {
        let tokens: Vec<Token> = self.pre_process(corpus);
        self.vocabulary = self.extract_vocabulary(&tokens);
    }

    fn transform_(&self, corpus: &str) -> Vec<Token> {
        let tokens: Vec<Token> = self.pre_process(corpus);
        self.tokenize(&self.vocabulary, &tokens)
    }
}

#[pymethods]
impl BasicTokenizer {
    #[new]
    fn new(vocabulary: Vec<Word>, pre_tokenizers: Vec<PreTokenizerKind>) -> Self {
        BasicTokenizer::new_(Some(&vocabulary), Some(&pre_tokenizers))
    }

    fn fit(&mut self, corpus: &str) {
        BasicTokenizer::fit_(self, corpus)
    }

    fn transform(&self, corpus: &str) -> Vec<Token> {
        BasicTokenizer::transform_(self, corpus)
    }

    fn fit_transform(&mut self, corpus: &str) -> Vec<Token> {
        BasicTokenizer::fit_transform_(self, corpus)
    }

    #[getter]
    fn vocabulary(&self) -> PyResult<Vec<Word>> {
        Ok(self.vocabulary.to_vec())
    }
}
