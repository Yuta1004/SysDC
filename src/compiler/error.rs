use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use super::types::Type;
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
    TypeUnmatch1(Type),
    TypeUnmatch2(Type, Type),
    NotFound(String),
    NotDefined(String),
    MemberNotDefinedInData(String, String),
    FuncNotDefinedInModule(String, String),
    MissingFunctionName,
    IllegalAccess,

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

            CompileError::TypeUnmatch1(actual) => write!(f, "\"{:?}\" is defined, but type is unmatch", actual),
            CompileError::TypeUnmatch2(required, actual) => write!(f, "\"{:?}\" is required, but \"{:?}\" exists", required, actual),
            CompileError::NotFound(name) => write!(f, "Cannot find \"{}\"", name),
            CompileError::NotDefined(name) => write!(f, "\"{}\" is not defined", name),
            CompileError::MemberNotDefinedInData(member, data) => write!(f, "Member \"{}\" is not defined in Data \"{}\"", member, data),
            CompileError::FuncNotDefinedInModule(func, module) => write!(f, "Function \"{}\" is not defined in Module \"{}\"", func, module),
            CompileError::MissingFunctionName => write!(f, "Missing to specify the function"),
            CompileError::IllegalAccess => write!(f, "Found illegal access"),

            CompileError::InternalError => write!(f, "Occur something in compiler")
        }
    }
}
