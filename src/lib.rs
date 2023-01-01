use pyo3::prelude::*;
use tokenization::tokenizers::basic::BasicTokenizer;

mod tokenization;

#[pymodule]
fn dante(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BasicTokenizer>()?;

    Ok(())
}
