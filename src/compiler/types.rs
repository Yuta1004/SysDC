use std::fmt::{ Debug, Formatter };

use super::name::Name;

#[derive(PartialEq)]
pub enum SysDCType {
    /* Default */
    Int32,

    /* User defined */
    Solved(Name),
    Unsolved(Name)
}

impl SysDCType {
    pub fn from(namespace: &Name, name: String) -> SysDCType {
        match name.as_str() {
            "i32" => SysDCType::Int32,
            _ => SysDCType::Unsolved(Name::new(namespace, name))
        }
    }

    pub fn get_name(&self) -> Name {
        match self {
            SysDCType::Int32 => Name::new_on_global_namespace("i32".to_string()),
            SysDCType::Solved(name) => name.clone(),
            SysDCType::Unsolved(name) => name.clone()
        }
    }
}

impl Debug for SysDCType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name().get_global_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use super::SysDCType;

    #[test]
    fn from_all_ok() {
        assert_eq!(SysDCType::from(&Name::new_root(), "i32".to_string()), SysDCType::Int32);
        assert_eq!(SysDCType::from(&Name::new_root(), "cocoa".to_string()), SysDCType::Unsolved(Name::new(&Name::new_root(), "cocoa".to_string())));
    }
}
