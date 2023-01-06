use pyo3::prelude::*;

use tokenization::{
    tokenizers::basic::BasicTokenizer,
    pre_tokenizers::PreTokenizerKind
};

mod tokenization;

#[pymodule]
fn dante(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BasicTokenizer>()?;
    m.add_class::<PreTokenizerKind>()?;

    Ok(())
}
