use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use super::types::Type;
use super::token::TokenKind;

#[derive(Debug)]
pub enum CompileErrorKind {
    /* トークン分割時に発生したエラー */
    RequestedTokenNotFound(TokenKind),
    FoundUnregisteredSymbol,

    /* パース時に発生したエラー */
    UnitNameNotSpecified,
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
}

impl Display for CompileErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompileErrorKind::RequestedTokenNotFound(kind) => write!(f, "Token \"{:?}\" is requested, but not found", kind),
            CompileErrorKind::FoundUnregisteredSymbol => write!(f, "Found unregistered symbol"),

            CompileErrorKind::UnitNameNotSpecified => write!(f, "Unit name is not specified"),
            CompileErrorKind::UnexpectedEOF => write!(f, "Expected Data or Module definition, but not found"),
            CompileErrorKind::ReturnExistsMultiple => write!(f, "Annotation \"return\" exists multiple"),
            CompileErrorKind::ReturnNotExists => write!(f, "Annotation \"return\" not existed"),
            CompileErrorKind::ResultOfSpawnNotSpecified => write!(f, "Missing to specify the result of spawn"),
            CompileErrorKind::FunctionNameNotFound => write!(f, "Function name is requested, but not found"),

            CompileErrorKind::TypeUnmatch1(actual) => write!(f, "\"{:?}\" is defined, but type is unmatch", actual),
            CompileErrorKind::TypeUnmatch2(required, actual) => write!(f, "\"{:?}\" is required, but \"{:?}\" exists", required, actual),
            CompileErrorKind::NotFound(name) => write!(f, "Cannot find \"{}\"", name),
            CompileErrorKind::NotDefined(name) => write!(f, "\"{}\" is not defined", name),
            CompileErrorKind::MemberNotDefinedInData(member, data) => write!(f, "Member \"{}\" is not defined in Data \"{}\"", member, data),
            CompileErrorKind::FuncNotDefinedInModule(func, module) => write!(f, "Function \"{}\" is not defined in Module \"{}\"", func, module),
            CompileErrorKind::MissingFunctionName => write!(f, "Missing to specify the function"),
            CompileErrorKind::IllegalAccess => write!(f, "Found illegal access"),
        }
    }
}

#[derive(Debug)]
pub struct CompileError {
    kind: CompileErrorKind,
    pos: Option<(i32, i32)>
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.pos {
            Some((row, col)) => write!(f, "{} (at {}:{})", self.kind, row, col),
            None => write!(f, "{}", self.kind)
        }
    }
}

impl CompileError {
    pub fn new<T>(kind: CompileErrorKind) -> Result<T, Box<dyn Error>> {
        Err(Box::new(CompileError { kind, pos: None }))
    }

    pub fn new_with_pos<T>(kind: CompileErrorKind, pos: (i32, i32)) -> Result<T, Box<dyn Error>> {
        Err(Box::new(CompileError { kind, pos: Some(pos) }))
    }
}
