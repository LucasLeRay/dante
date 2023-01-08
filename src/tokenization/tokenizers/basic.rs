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
    fn new_(
        vocabulary: Option<&Vec<Word>>,
        pre_tokenizers: Option<&Vec<PreTokenizerKind>>
    ) -> Self {
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

    fn pre_process(&self, corpus: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token{word: corpus.to_string()}];
        for pre_tokenizer in self.pre_tokenizers.iter() {
            tokens = pre_tokenizer.get_processor().pre_tokenize(&tokens);
        }

        tokens
    }
}

impl Tokenizer for BasicTokenizer {
    fn fit_(&mut self, corpus: &str) {
        let tokens: Vec<Token> = self.pre_process(corpus);
        self.vocabulary.extend(self.extract_vocabulary(&tokens));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenization::pre_tokenizers::PreTokenizerKind;

    #[test]
    fn fitted_vocabulary() {
        let mut tokenizer = BasicTokenizer::new_(
            None, None
        );

        tokenizer.fit("Hello World!");
        assert_eq!(tokenizer.vocabulary, vec!["Hello World!"]);
    }

    #[test]
    fn custom_vocabulary() {
        let mut tokenizer = BasicTokenizer::new_(
            Some(&vec!["Lucas".to_string()]), None
        );

        tokenizer.fit("Hello World!");
        assert_eq!(tokenizer.vocabulary, vec!["Lucas", "Hello World!"]);
    }

    #[test]
    fn pre_tokenizers_are_applied() {
        let mut tokenizer = BasicTokenizer::new_(
            None,
            Some(&vec![PreTokenizerKind::WhiteSpace])
        );

        tokenizer.fit("Hello World!");
        assert_eq!(tokenizer.vocabulary, vec!["Hello", "World!"]);
    }

    #[test]
    fn transformation() {
        use crate::tokenization::special_tokens::UNK;

        let mut tokenizer = BasicTokenizer::new_(
            None,
            Some(&vec![
                PreTokenizerKind::WhiteSpace,
                PreTokenizerKind::Punctuation,
            ])
        );

        tokenizer.fit("Hello World!");
        let result: Vec<Token> = tokenizer.transform("Hello Lucas!");
        assert_eq!(
            result,
            vec![
                Token{word: "Hello".to_string()},
                Token{word: UNK.to_string()},
                Token{word: "!".to_string()},
            ]
        );
    }
}
