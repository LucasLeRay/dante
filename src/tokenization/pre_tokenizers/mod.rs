use pyo3::pyclass;

use self::pre_tokenizer::PreTokenizer;

pub mod pre_tokenizer;
pub mod case_fold;
pub mod punctuation;

#[derive(Clone)]
#[pyclass]
pub enum PreTokenizerKind {
    CaseFold,
    Punctuation
}

impl PreTokenizerKind {
    pub fn get_processor(&self) -> &dyn PreTokenizer {
        match self {
            PreTokenizerKind::CaseFold => &case_fold::CaseFold,
            PreTokenizerKind::Punctuation => &punctuation::Punctuation
        }
    }
}
