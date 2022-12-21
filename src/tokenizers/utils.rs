pub const EOW: &str = "</w>";
pub const SOW: &str = "<w>";

pub fn wrap_word(word: Vec<String>) -> Vec<String> {
    let mut wrapped: Vec<String> = Vec::new();

    wrapped.push(SOW.to_string());
    for char in word.iter() {
        wrapped.push(char.to_owned());
    }
    wrapped.push(EOW.to_string());

    wrapped
}
