use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

#[derive(Debug)]
pub enum CompileError {
    /* トークン分割時に発生したエラー */
    TokenizeError(String),

    /* パース時に発生したエラー */
    ParseError(String),

    /* 検査時に発生したエラー */
    CheckError(String)
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompileError::TokenizeError(msg) => write!(f, "{} (CompileError::TokenizeError)", msg),
            CompileError::ParseError(msg) => write!(f, "{} (CompileError::ParseError)", msg),
            CompileError::CheckError(msg) => write!(f, "{} (CompileError::CheckError)", msg)
        }
    }
}
