use std::fmt::{ Debug, Formatter };

use super::name::Name;

pub enum SysDCType {
    /* Default */
    Int32,
    Float32,
    StringType,
    NoneType,
   
    /* User defined */
    Solved(Name),               // name(full)
    Unsolved(Name, String)      // namespace, name(local)
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
            _ => SysDCType::Unsolved(namespace.clone(), name.clone())
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            SysDCType::Int32 => "int32".to_string(),
            SysDCType::Float32 => "float32".to_string(),
            SysDCType::StringType => "string".to_string(),
            SysDCType::NoneType => "none".to_string(),
            SysDCType::Solved(name) => name.get_name(),
            SysDCType::Unsolved(namespace, name) => Name::new(&namespace, &name).get_name()
        }
    }

    pub fn get_full_name(&self) -> String {
        match self {
            SysDCType::Int32
            | SysDCType::Float32
            | SysDCType::StringType
            | SysDCType::NoneType => Name::new(&Name::new(&Name::new_root(), &"global".to_string()), &self.get_name()).get_full_name(),
            SysDCType::Solved(name) => name.get_full_name(),
            SysDCType::Unsolved(namespace, name) => Name::new(&namespace, &name).get_full_name()
        }
    }
}

impl Debug for SysDCType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_full_name())
    }
}
