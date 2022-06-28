use std::rc::Rc;
use std::fmt::{ Debug, Formatter };

use super::name::Name;

pub trait SysDCType {
    fn get_name(&self) -> String;
    fn get_full_name(&self) -> String;
}

impl Debug for dyn SysDCType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_full_name())
    }
}

#[derive(Debug)]
pub struct SysDCDefaultType {
    name: Name
}

impl SysDCDefaultType {
    fn new(name: &str) -> SysDCDefaultType {
        SysDCDefaultType {
            name: Name::new(&Name::new(&Name::new_root(), &"global".to_string()), &name.to_string())
        }
    }

    pub fn get_all() -> Vec<Rc<dyn SysDCType>> {
        vec!(
            Rc::new(SysDCDefaultType::new("int32")),
            Rc::new(SysDCDefaultType::new("float32")),
            Rc::new(SysDCDefaultType::new("string")),
            Rc::new(SysDCDefaultType::new("none")),
        )
    }
}

impl SysDCType for SysDCDefaultType {
    fn get_name(&self) -> String {
        self.name.get_name()
    }

    fn get_full_name(&self) -> String {
        self.name.get_full_name()
    }
}

#[derive(Debug)]
pub struct TmpType {
    name: String
}

impl TmpType {
    pub fn new(name: &String) -> Rc<TmpType> {
        Rc::new(
            TmpType { name: name.to_string() }
        )
    }
}

impl SysDCType for TmpType {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_full_name(&self) -> String {
        self.name.clone()
    }
}
