use crate::tokenization::token::{Word, Token};
use crate::tokenization::splitters::splitter::Splitter;
use crate::tokenization::tokenizers::tokenizer::Tokenizer;
use crate::tokenization::pre_tokenizers::pre_tokenizer::PreTokenizer;

use super::tokenizers::noop::NoopTokenizer;


// most certainly not... vocabulary can be get from the fit
// pub struct TokenizationResults {
//     vocabulary: Vec<Word>,
//     tokens: Vec<Token>
// }

pub struct TokenizationPipeline<'a> {
    pub vocabulary: Vec<Word>,
    splitter: &'a (dyn Splitter + 'a),
    pre_tokenizers: Vec<&'a (dyn PreTokenizer + 'a)>,
    tokenizer: &'a (dyn Tokenizer + 'a),
}

impl<'a> TokenizationPipeline<'a> {
    pub fn new(
        splitter: &'a dyn Splitter,
        pre_tokenizers: Option<Vec<&'a dyn PreTokenizer>>,
        tokenizer: Option<&'a dyn Tokenizer>
    ) -> TokenizationPipeline<'a> {
        TokenizationPipeline {
            splitter,
            pre_tokenizers: match pre_tokenizers {
                None => Vec::new(),
                Some(transformers) => transformers
            },
            tokenizer: match tokenizer {
                None => &NoopTokenizer{},
                Some(t) => t
            },
            vocabulary: Vec::new()
        }
    }

    fn pre_process(&self, corpus: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = self.splitter.split(&corpus);
        for pre_tokenizer in self.pre_tokenizers.iter() {
            tokens = pre_tokenizer.pre_tokenize(&tokens);
        }
        tokens
    }

    pub fn fit(&mut self, corpus: &str) {
        let mut tokens: Vec<Token> = self.pre_process(corpus);
        self.vocabulary = self.tokenizer.extract_vocabulary(&tokens);
    }

    pub fn transform(&self, corpus: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = self.pre_process(corpus);
        self.tokenizer.tokenize(&self.vocabulary, &tokens)
    }

    pub fn fit_transform(&mut self, corpus: &str) -> Vec<Token> {
        self.fit(corpus);
        self.transform(corpus)
    }
}


// tokenizer = Tokenizer(corpus).split(Splitter).pre_tokenize(pre_tokenizers).add_model()
// pipeline = TokenizationPipeline(
//     splitter=Splitter,
//     pre_tokenizers=Vec<PreTokenizer>
//     tokenizer=Tokenizer
// ).fit(train_corpus)
//
// tokens = pipeline.transform(test_corpus)
