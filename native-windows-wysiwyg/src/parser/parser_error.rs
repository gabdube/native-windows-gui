use std::io::Error as IoError;
use proc_macro2::LexError;

#[derive(Debug)]
pub enum ParserError {
    Io(IoError),
    Pm2(LexError),
    StructParsing(String)
}

impl From<IoError> for ParserError {
    fn from(err: IoError) -> Self {
        ParserError::Io(err)
    }
}

impl From<LexError> for ParserError {
    fn from(err: LexError) -> Self {
        ParserError::Pm2(err)
    }
}
