#[derive(Debug, PartialEq)]
pub enum ParseFenError {
    UnexpectedChar,
    EmptyString,
    BadFenFormat(&'static str),
}
