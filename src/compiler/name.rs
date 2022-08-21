use std::fmt::{ Debug, Formatter };

use serde::{ Serialize, Deserialize };

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub namespace: String
}

impl Name {
    pub fn new_root() -> Name {
        Name {
            name: "0".to_string(),
            namespace: "".to_string()
        }
    }

    pub fn new_on_global_namespace(name: String) -> Name {
        Name::from(&Name::from(&Name::new_root(), "global".to_string()), name)
    }

    pub fn from(base: &Name, name: String) -> Name {
        Name {
            name: name.clone(),
            namespace: base.get_global_name()
        }
    }

    pub fn get_global_name(&self) -> String {
        self.namespace.clone() + "." + &self.name
    }

    pub fn get_par_name(&self, ignore_underscore: bool) -> Name {
        let name = self.get_global_name();
        let splitted_name = name.split(".").filter(|x| !ignore_underscore || x != &"_").collect::<Vec<&str>>();
        let par_name = splitted_name[splitted_name.len()-2];
        let par_namespace = splitted_name[0..splitted_name.len()-2].join(".");
        Name { name: par_name.to_string(), namespace: par_namespace }
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_global_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn create_name() {
        let root = Name::new_root();
        let name = Name::from(&root, "aaa".to_string());
        assert_eq!(name.get_global_name(), ".0.aaa".to_string());

        let name = Name::new_root();
        assert_eq!(name.get_global_name(), ".0".to_string());
    }
}
