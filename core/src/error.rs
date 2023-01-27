use std::fmt;
use std::fmt::{Display, Formatter};

use thiserror::Error;

use super::location::Location;
use super::token::TokenKind;
use super::types::Type;

#[derive(Debug, Error)]
pub enum PErrorKind {
    /* トークン分割時に発生したエラー */
    #[error("Token \"{0:?}\" is requested, but not found")]
    RequestedTokenNotFound(TokenKind),
    #[error("Found unregistered symbol")]
    FoundUnregisteredSymbol,

    /* パース時に発生したエラー */
    #[error("Unit name is not specified")]
    UnitNameNotSpecified,
    #[error("From namespace is not specified")]
    FromNamespaceNotSpecified,
    #[error("Expected Data or Module definition, but not found")]
    DataOrModuleNotFound,
    #[error("Unexpected EOF found")]
    UnexpectedEOF,
    #[error("Annotation \"return\" exists multiple")]
    ReturnExistsMultiple,
    #[error("Annotation \"return\" exists on procedure")]
    ReturnExistsOnProcedure,
    #[error("Annotation \"return\" not exists")]
    ReturnNotExists,
    #[error("Missing to specify the result os spawn")]
    ResultOfSpawnNotSpecified,
    #[error("Function name is requested, but not found")]
    FunctionNameNotFound,
    #[error("Unknown annotation \"{0}\" found")]
    UnknownAnnotationFound(String),

    /* 検査時に発生したエラー */
    #[error("\"{0}\" is already defiend")]
    AlreadyDefined(String),
    #[error("\"{0:?}\" is defined, but type is mismatch")]
    TypeUnmatch1(Type),
    #[error("\"{0:?}\" is required, but \"{1:?}\" found")]
    TypeUnmatch2(Type, Type),
    #[error("Argument length not match")]
    ArgumentsLengthNotMatch,
    #[error("Cannot find \"{0}\"")]
    NotFound(String),
    #[error("\"{0}\" is not defined")]
    NotDefined(String),
    #[error("Member \"{0}\" is not defined in Data \"{1}\"")]
    MemberNotDefinedInData(String, String),
    #[error("Function \"{0}\" is not defiend in Module \"{1}\"")]
    FuncNotDefinedInModule(String, String),
    #[error("Missing to specify the function")]
    MissingFunctionName,
    #[error("Found illegal access")]
    IllegalAccess,
}

#[derive(Debug, Error)]
pub struct PError {
    kind: PErrorKind,
    happen_at: Location,
}

impl From<PErrorKind> for PError {
    fn from(kind: PErrorKind) -> PError {
        PError {
            kind,
            happen_at: Location::new(),
        }
    }
}

impl Display for PError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (at {})", self.kind, self.happen_at)
    }
}

impl PError {
    pub fn with_loc(mut self, location: Location) -> PError {
        self.happen_at = location;
        self
    }
}
