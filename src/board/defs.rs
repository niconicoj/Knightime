#[derive(Debug, PartialEq)]
pub enum ParseFenError {
    UnexpectedChar(&'static str),
}
