use std::fmt::{ Debug, Formatter };

use serde::{ Serialize, Deserialize };
use serde::ser::Serializer;
use serde::de::Deserializer;

use super::name::Name;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub refs: Option<Name>
}

impl Type {
    pub fn new(kind: TypeKind, name: Option<Name>) -> Type {
        Type {
            kind,
            refs: name
        }
    }

    pub fn new_unsovled_nohint() -> Type {
        Type {
            kind: TypeKind::UnsolvedNoHint,
            refs: None
        }
    }

    pub fn from(name: String) -> Type {
        match name.as_str() {
            "i32" => Type { kind: TypeKind::Int32, refs: None },
            _ => Type { kind: TypeKind::Unsolved(name), refs: None }
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            TypeKind::Int32 => write!(f, "Int32"),
            TypeKind::Unsolved(hint) => write!(f, "{}", hint),
            TypeKind::UnsolvedNoHint => write!(f, "UnsolvedNoHint"),
            _ => {
                write!(f, "{:?}:{:?}", self.kind, self.refs)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    /* プリミティブ型 */
    Int32,

    /* ユーザ定義型 */
    Data,

    /* パーサ用 (解決後のSysDCSystemには含まれない) */
    Unsolved(String),
    UnsolvedNoHint
}

impl Serialize for TypeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            TypeKind::Unsolved(_) |
            TypeKind::UnsolvedNoHint => panic!("[ERROR] Cannot serialize object containing unsolved types."),
            _ => serializer.serialize_str(&format!("{:?}", self))
        }
    }
}

impl<'de> Deserialize<'de> for TypeKind {
    fn deserialize<D>(deserializer: D) -> Result<TypeKind, D::Error>
    where
        D: Deserializer<'de> 
    {
        let kind = String::deserialize(deserializer)?;
        Ok(match kind.as_str() {
            "Int32" => TypeKind::Int32,
            "Data" => TypeKind::Data,
            s => panic!("[ERROR] Found unknown type at deserializing => \"{}\"", s)
        })
    }
}
