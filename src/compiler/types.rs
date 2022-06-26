use std::rc::Rc;

pub trait SysDCType {
    fn get_name(&self) -> String;
    fn get_full_name(&self) -> String;
}

pub struct SysDCDefaultType {
    name: String
}

impl SysDCType for SysDCDefaultType {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_full_name(&self) -> String {
        ".0.global.".to_string() + &self.name
    }
}

impl SysDCDefaultType {
    fn new(name: &str) -> SysDCDefaultType {
        SysDCDefaultType { name: name.to_string() }
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
