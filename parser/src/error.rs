use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use super::types::Type;
use super::token::TokenKind;

pub type PResult<T> = Result<T, PError>;

#[derive(Debug)]
pub enum PErrorKind {
    /* トークン分割時に発生したエラー */
    RequestedTokenNotFound(TokenKind),
    FoundUnregisteredSymbol,

    /* パース時に発生したエラー */
    UnitNameNotSpecified,
    FromNamespaceNotSpecified,
    UnexpectedEOF,
    ReturnExistsMultiple,
    ReturnNotExists,
    ResultOfSpawnNotSpecified,
    FunctionNameNotFound,

    /* 検査時に発生したエラー */
    AlreadyDefined(String),
    TypeUnmatch1(Type),
    TypeUnmatch2(Type, Type),
    NotFound(String),
    NotDefined(String),
    MemberNotDefinedInData(String, String),
    FuncNotDefinedInModule(String, String),
    MissingFunctionName,
    IllegalAccess,
}

impl Display for PErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PErrorKind::RequestedTokenNotFound(kind) => write!(f, "Token \"{:?}\" is requested, but not found", kind),
            PErrorKind::FoundUnregisteredSymbol => write!(f, "Found unregistered symbol"),

            PErrorKind::UnitNameNotSpecified => write!(f, "Unit name is not specified"),
            PErrorKind::FromNamespaceNotSpecified => write!(f, "From namespace is not specified"),
            PErrorKind::UnexpectedEOF => write!(f, "Expected Data or Module definition, but not found"),
            PErrorKind::ReturnExistsMultiple => write!(f, "Annotation \"return\" exists multiple"),
            PErrorKind::ReturnNotExists => write!(f, "Annotation \"return\" not existed"),
            PErrorKind::ResultOfSpawnNotSpecified => write!(f, "Missing to specify the result of spawn"),
            PErrorKind::FunctionNameNotFound => write!(f, "Function name is requested, but not found"),

            PErrorKind::AlreadyDefined(name) => write!(f, "\"{}\" is already defined", name),
            PErrorKind::TypeUnmatch1(actual) => write!(f, "\"{:?}\" is defined, but type is unmatch", actual),
            PErrorKind::TypeUnmatch2(required, actual) => write!(f, "\"{:?}\" is required, but \"{:?}\" exists", required, actual),
            PErrorKind::NotFound(name) => write!(f, "Cannot find \"{}\"", name),
            PErrorKind::NotDefined(name) => write!(f, "\"{}\" is not defined", name),
            PErrorKind::MemberNotDefinedInData(member, data) => write!(f, "Member \"{}\" is not defined in Data \"{}\"", member, data),
            PErrorKind::FuncNotDefinedInModule(func, module) => write!(f, "Function \"{}\" is not defined in Module \"{}\"", func, module),
            PErrorKind::MissingFunctionName => write!(f, "Missing to specify the function"),
            PErrorKind::IllegalAccess => write!(f, "Found illegal access"),
        }
    }
}

#[derive(Debug)]
pub struct PError {
    kind: PErrorKind,
    filename: Option<String>,
    pos: Option<(i32, i32)>
}

impl Error for PError {}

impl Display for PError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (&self.filename, self.pos) {
            (Some(filename), Some((row, col))) => {
                write!(f, "{} (at {}:{}:{})", self.kind, filename, row, col)
            },
            (None, Some((row, col))) => {
                write!(f, "{} (at {}:{})", self.kind, row, col)
            },
            (Some(filename), None) => {
                write!(f, "{} (at {})", self.kind, filename)
            },
            _ => write!(f, "{}", self.kind)
        }
    }
}

impl PError {
    pub fn new<T>(kind: PErrorKind) -> PResult<T> {
        Err(PError { kind, filename: None, pos: None })
    }

    pub fn new_with_pos<T>(kind: PErrorKind, pos: (i32, i32)) -> PResult<T> {
        Err(PError { kind, filename: None, pos: Some(pos) })
    }

    pub fn append_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    pub fn upgrade<T>(self) -> Result<T, Box<dyn Error>> {
        Err(Box::new(self))
    }
}
