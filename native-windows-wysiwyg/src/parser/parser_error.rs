use std::io::Error as IoError;
use proc_macro2::LexError;

pub enum ParserError {
    Io(IoError),
    Pm2(LexError),
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
