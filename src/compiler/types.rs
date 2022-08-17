use std::fmt::{ Debug, Formatter };

use super::name::Name;

#[derive(PartialEq)]
pub enum Type {
    /* Default */
    Int32,

    /* User defined */
    Solved(Name),
    Unsolved(Name),
    UnsolvedNoHint
}

impl Type {
    pub fn from(namespace: &Name, name: String) -> Type {
        match name.as_str() {
            "i32" => Type::Int32,
            _ => Type::Unsolved(Name::from(namespace, name))
        }
    }

    pub fn get_name(&self) -> Name {
        match self {
            Type::Int32 => Name::new_on_global_namespace("i32".to_string()),
            Type::Solved(name) => name.clone(),
            Type::Unsolved(name) => name.clone(),
            Type::UnsolvedNoHint => Name::new_on_global_namespace("NoHintType".to_string())
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name().get_global_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use super::Type;

    #[test]
    fn from_all_ok() {
        assert_eq!(Type::from(&Name::new_root(), "i32".to_string()), Type::Int32);
        assert_eq!(Type::from(&Name::new_root(), "cocoa".to_string()), Type::Unsolved(Name::from(&Name::new_root(), "cocoa".to_string())));
    }
}
