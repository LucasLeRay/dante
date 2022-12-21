use pyo3::prelude::*;

mod tokenizers;

#[pymodule]
fn dante(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<tokenizers::bpe::BPE>()?;
    Ok(())
}
