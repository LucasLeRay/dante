use std::fmt;

use pyo3::pyclass;

pub type Word = String;

// struct Offset {
//     start: u32,
//     end: u32
// }

// #[derive(Eq, Clone)]
#[derive(Clone, Debug)]
#[pyclass]
pub struct Token {
    pub word: Word,
    // offset: Offset,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}
