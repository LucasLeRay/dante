use std::fmt;

use pyo3::{pyclass, pymethods, PyResult};

pub type Word = String;

#[derive(Clone, Debug, PartialEq)]
#[pyclass]
pub struct Token {
    pub word: Word,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

#[pymethods]
impl Token {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.word.to_string())
    }
}
