use std::fmt;

pub type Word = String;

// struct Offset {
//     start: u32,
//     end: u32
// }

// #[derive(Eq, Clone)]
#[derive(Clone, Debug)]
pub struct Token {
    pub word: Word,
    // offset: Offset,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

// impl Hash for Token {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.word.hash(state);
//     }
// }

// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         self.word == other.word
//     }
// }
