use pyo3::prelude::*;

use tokenization::{
    tokenizers::basic::BasicTokenizer,
    pre_tokenizers::PreTokenizerKind
};
use models::language::mle::MLE;

mod models;
mod tokenization;
mod utils;

#[pymodule]
fn dante(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BasicTokenizer>()?;
    m.add_class::<PreTokenizerKind>()?;

    m.add_class::<MLE>()?;

    Ok(())
}
