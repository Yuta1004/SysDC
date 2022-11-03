use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub namespace: String,
}

impl Name {
    pub fn new(base: &Name, name: String) -> Name {
        Name {
            name,
            namespace: base.get_full_name(),
        }
    }

    pub fn new_root() -> Name {
        Name {
            name: "0".to_string(),
            namespace: "".to_string(),
        }
    }

    pub fn get_full_name(&self) -> String {
        self.namespace.clone() + "." + &self.name
    }

    pub fn get_namespace(&self, ignore_underscore: bool) -> Name {
        let splitted_name = self
            .namespace
            .split('.')
            .filter(|x| !ignore_underscore || x != &"_")
            .collect::<Vec<&str>>();
        let new_name = splitted_name[splitted_name.len() - 2].to_string();
        let new_namespace = splitted_name[0..splitted_name.len() - 2].join(".");
        Name {
            name: new_name,
            namespace: new_namespace,
        }
    }

    pub fn get_par_name(&self, ignore_underscore: bool) -> Name {
        let name = self.get_full_name();
        let splitted_name = name
            .split('.')
            .filter(|x| !ignore_underscore || x != &"_")
            .collect::<Vec<&str>>();
        let par_name = splitted_name[splitted_name.len() - 2];
        let par_namespace = splitted_name[0..splitted_name.len() - 2].join(".");
        Name {
            name: par_name.to_string(),
            namespace: par_namespace,
        }
    }

    pub fn has_underscore(&self) -> bool {
        self.namespace.contains('_') || self.name.contains('_')
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_full_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn create_name() {
        let root = Name::new_root();
        let name = Name::new(&root, "aaa".to_string());
        assert_eq!(name.get_full_name(), ".0.aaa".to_string());

        let name = Name::new_root();
        assert_eq!(name.get_full_name(), ".0".to_string());
    }
}
