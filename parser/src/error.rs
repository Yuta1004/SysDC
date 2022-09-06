use std::fmt;
use std::fmt::{ Display, Formatter };
use std::error::Error;

use super::util::Location;
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
    DataOrModuleNotFound,
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
            PErrorKind::DataOrModuleNotFound => write!(f, "Expected Data or Module definition, but not found"),
            PErrorKind::UnexpectedEOF => write!(f, "Unexpected EOF found"),
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
    happen_at: Location
}

impl Error for PError {}

impl Display for PError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (at {})", self.kind, self.happen_at)
    }
}

impl PError {
    pub fn new<T>(kind: PErrorKind) -> PResult<T> {
        Err(PError { kind, happen_at: Location::new() })
    }

    pub fn new_with_loc<T>(kind: PErrorKind, happen_at: Location) -> PResult<T> {
        Err(PError { kind, happen_at })
    }

    pub fn set_filename(&mut self, filename: String) {
        self.happen_at.set_filename(filename);
    }

    pub fn upgrade<T>(self) -> Result<T, Box<dyn Error>> {
        Err(Box::new(self))
    }
}
