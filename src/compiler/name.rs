pub struct Name {
    name: String,
    namespace: String
}

impl Name {
    pub fn new(name: String, namespace: String) -> Name {
        let last_idx = namespace.len()-1;
        let namespace = match namespace.chars().nth(last_idx).unwrap() {
            '.' => namespace,
            _ => namespace + "."
        };

        Name { name, namespace }
    }

    pub fn new_root() -> Name {
        Name {
            name: "0".to_string(),
            namespace: ".".to_string()
        }
    }

    pub fn from(base: Name, name: String) -> Name {
        Name {
            name,
            namespace: base.get_full_name() + "."
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_full_name(&self) -> String {
        self.namespace.clone() + &self.name
    }
}

#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn create_name() {
        let name = Name::new("aaa".to_string(), ".".to_string());
        assert_eq!(name.get_name(), "aaa".to_string());
        assert_eq!(name.get_full_name(), ".aaa".to_string());

        let name = Name::new("aaa".to_string(), ".test".to_string());
        assert_eq!(name.get_name(), "aaa".to_string());
        assert_eq!(name.get_full_name(), ".test.aaa".to_string());

        let name = Name::new_root();
        assert_eq!(name.get_name(), "0".to_string());
        assert_eq!(name.get_full_name(), ".0".to_string());

        let name = Name::from(Name::new_root(), "aaa".to_string());
        assert_eq!(name.get_name(), "aaa".to_string());
        assert_eq!(name.get_full_name(), ".0.aaa".to_string());
    }
}
