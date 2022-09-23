use std::fmt::{Debug, Formatter};

use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use super::name::Name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub refs: Option<Name>,
}

impl Type {
    pub fn new(kind: TypeKind, name: Option<Name>) -> Type {
        Type { kind, refs: name }
    }

    pub fn new_unsovled_nohint() -> Type {
        Type {
            kind: TypeKind::UnsolvedNoHint,
            refs: None,
        }
    }
}

impl From<String> for Type {
    fn from(name: String) -> Type {
        Type {
            kind: TypeKind::from(name),
            refs: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TypeKind {
    /* 無 */
    Void,

    /* プリミティブ型 */
    Int32,
    UInt32,
    Float32,
    Boolean,
    Char,

    /* ユーザ定義型 */
    Data,

    /* パーサ用 (解決後のSysDCSystemには含まれない) */
    Unsolved(String),
    UnsolvedNoHint,
}

impl TypeKind {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            TypeKind::Void
                | TypeKind::Int32
                | TypeKind::UInt32
                | TypeKind::Float32
                | TypeKind::Boolean
                | TypeKind::Char
        )
    }
}

impl From<String> for TypeKind {
    fn from(name: String) -> TypeKind {
        match name.as_str() {
            "void" => TypeKind::Void,
            "i32" => TypeKind::Int32,
            "u32" => TypeKind::UInt32,
            "f32" => TypeKind::Float32,
            "bool" => TypeKind::Boolean,
            "char" => TypeKind::Char,
            _ => TypeKind::Unsolved(name),
        }
    }
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TypeKind::Void => write!(f, "void"),
            TypeKind::Int32 => write!(f, "i32"),
            TypeKind::UInt32 => write!(f, "u32"),
            TypeKind::Float32 => write!(f, "f32"),
            TypeKind::Boolean => write!(f, "bool"),
            TypeKind::Char => write!(f, "char"),
            TypeKind::Data => write!(f, "Data"),
            TypeKind::Unsolved(hint) => write!(f, "{}", hint),
            TypeKind::UnsolvedNoHint => write!(f, "UnsolvedNoHint"),
        }
    }
}

impl Serialize for TypeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TypeKind::Unsolved(_) | TypeKind::UnsolvedNoHint => {
                panic!("[ERROR] Cannot serialize object containing unsolved types.")
            }
            _ => serializer.serialize_str(&format!("{:?}", self)),
        }
    }
}

impl<'de> Deserialize<'de> for TypeKind {
    fn deserialize<D>(deserializer: D) -> Result<TypeKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let skind = String::deserialize(deserializer)?;
        match TypeKind::from(skind) {
            TypeKind::Unsolved(_) => Ok(TypeKind::Data),
            kind => Ok(kind),
        }
    }
}

#[cfg(test)]
mod test {
    use rmp_serde;
    use rmp_serde::Serializer;
    use serde::Serialize;

    use super::TypeKind;

    macro_rules! check_serialize {
        ($target:ty, $obj:expr) => {
            let mut serialized = vec![];
            $obj.serialize(&mut Serializer::new(&mut serialized))
                .unwrap();
            let deserialized = rmp_serde::from_slice::<$target>(&serialized[..]).unwrap();
            assert_eq!(deserialized, $obj);
        };
    }

    #[test]
    fn default() {
        check_serialize!(TypeKind, TypeKind::Void);
        check_serialize!(TypeKind, TypeKind::Int32);
        check_serialize!(TypeKind, TypeKind::UInt32);
        check_serialize!(TypeKind, TypeKind::Float32);
        check_serialize!(TypeKind, TypeKind::Boolean);
        check_serialize!(TypeKind, TypeKind::Char);
        check_serialize!(TypeKind, TypeKind::Data);
    }

    #[test]
    #[should_panic]
    fn primitive_unsolved_1() {
        check_serialize!(TypeKind, TypeKind::Unsolved("aaa".to_string()));
    }

    #[test]
    #[should_panic]
    fn primitive_unsolved_2() {
        check_serialize!(TypeKind, TypeKind::UnsolvedNoHint);
    }
}
