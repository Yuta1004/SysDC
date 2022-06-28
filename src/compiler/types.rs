use std::fmt::{ Debug, Formatter };

use super::name::Name;

#[derive(PartialEq)]
pub enum SysDCType {
    /* Default */
    Int32,
    Float32,
    StringType,
    NoneType,
   
    /* User defined */
    Solved(Name),
    Unsolved(Name)
}

impl SysDCType {
    pub fn from(namespace: &Name, name: &String) -> SysDCType {
        match name.as_str() {
            "int32" => SysDCType::Int32,
            "float32" => SysDCType::Float32,
            "string" => SysDCType::StringType,
            "none" => SysDCType::NoneType,
            _ => panic!("[ERROR] Type {} is not found in this scope({:?})", name, namespace)
        }
    }

    pub fn from_allow_unsolved(namespace: &Name, name: &String) -> SysDCType {
        match name.as_str() {
            "int32" => SysDCType::Int32,
            "float32" => SysDCType::Float32,
            "string" => SysDCType::StringType,
            "none" => SysDCType::NoneType,
            _ => SysDCType::Unsolved(Name::new(namespace, name))
        }
    }

    pub fn get_name(&self) -> Name {
        match self {
            SysDCType::Int32 => Name::new_on_global_namespace(&"int32".to_string()),
            SysDCType::Float32 => Name::new_on_global_namespace(&"float32".to_string()),
            SysDCType::StringType => Name::new_on_global_namespace(&"string".to_string()),
            SysDCType::NoneType => Name::new_on_global_namespace(&"none".to_string()),
            SysDCType::Solved(name) => name.clone(),
            SysDCType::Unsolved(name) => name.clone()
        }
    }
}

impl Debug for SysDCType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_name().get_global_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use super::SysDCType;

    #[test]
    fn from_all_ok() {
        assert_eq!(SysDCType::from_allow_unsolved(&Name::new_root(), &"int32".to_string()), SysDCType::Int32);
        assert_eq!(SysDCType::from_allow_unsolved(&Name::new_root(), &"float32".to_string()), SysDCType::Float32);
        assert_eq!(SysDCType::from_allow_unsolved(&Name::new_root(), &"string".to_string()), SysDCType::StringType);
        assert_eq!(SysDCType::from_allow_unsolved(&Name::new_root(), &"none".to_string()), SysDCType::NoneType);
        assert_eq!(SysDCType::from_allow_unsolved(&Name::new_root(), &"cocoa".to_string()), SysDCType::Unsolved(Name::new(&Name::new_root(), &"cocoa".to_string())));
    }

    #[test]
    #[should_panic]
    fn from_ng() {
        assert_eq!(SysDCType::from(&Name::new_root(), &"cocoa".to_string()), SysDCType::Unsolved(Name::new(&Name::new_root(), &"cocoa".to_string())));
    }
}
