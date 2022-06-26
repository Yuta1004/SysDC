use std::fmt::{ Debug, Formatter };

#[derive(Clone)]
pub struct Name {
    name: String,
    namespace: String
}

impl Name {
    pub fn new(base: &Name, name: &String) -> Name {
        Name {
            name: name.clone(),
            namespace: base.get_full_name()
        }
    }

    pub fn new_root() -> Name {
        Name {
            name: "0".to_string(),
            namespace: "".to_string()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_full_name(&self) -> String {
        self.namespace.clone() + "." + &self.name
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.get_full_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn create_name() {
        let root = Name::new_root();
        let name = Name::new(&root, &"aaa".to_string());
        assert_eq!(name.get_name(), "aaa".to_string());
        assert_eq!(name.get_full_name(), ".0.aaa".to_string());

        let name = Name::new_root();
        assert_eq!(name.get_name(), "0".to_string());
        assert_eq!(name.get_full_name(), ".0".to_string());
    }
}
