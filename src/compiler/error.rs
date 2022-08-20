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
    UnexpectedEOF,
    ReturnExistsMultiple,
    ReturnNotExists,
    ResultOfSpawnNotSpecified,
    FunctionNameNotFound,

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

            CompileError::UnexpectedEOF => write!(f, "Expected Data or Module definition, but not found"),
            CompileError::ReturnExistsMultiple => write!(f, "Annotation \"return\" exists multiple"),
            CompileError::ReturnNotExists => write!(f, "Annotation \"return\" not existed"),
            CompileError::ResultOfSpawnNotSpecified => write!(f, "Missing to specify the result of spawn"),
            CompileError::FunctionNameNotFound => write!(f, "Function name is requested, but not found"),

            CompileError::CheckError(msg) => write!(f, "{} (CompileError::CheckError)", msg),

            CompileError::InternalError => write!(f, "Occur something in compiler")
        }
    }
}
