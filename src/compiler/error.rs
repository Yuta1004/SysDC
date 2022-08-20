use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use super::token::TokenKind;

#[derive(Debug)]
pub enum CompileError {
    /* トークン分割時に発生したエラー */
    RequestedTokenNotFound(TokenKind),
    FoundUnregisteredSymbol,

    /* パース時に発生したエラー */
    ParseError(String),

    /* 検査時に発生したエラー */
    CheckError(String),

    /* 内部エラー */
    InternalError
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompileError::RequestedTokenNotFound(kind) => write!(f, "Token \"{:?}\" is requested, but not found", kind),
            CompileError::FoundUnregisteredSymbol => write!(f, "Found unregistered symbol"),

            CompileError::ParseError(msg) => write!(f, "{} (CompileError::ParseError)", msg),
            CompileError::CheckError(msg) => write!(f, "{} (CompileError::CheckError)", msg),

            CompileError::InternalError => write!(f, "Occur something in compiler")
        }
    }
}
