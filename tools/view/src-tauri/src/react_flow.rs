use serde::ser::SerializeStruct;
use serde::Serialize;

use sysdc_parser::name::Name;

pub struct Node {
    id: Name,
    label: String,
}

impl Node {
    pub fn new(id: Name, label: String) -> Node {
        Node { id, label }
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct NodeInner<'a> {
            label: &'a String,
        }

        let inner = NodeInner { label: &self.label };

        let mut s = serializer.serialize_struct("Node", 2)?;
        s.serialize_field("id", &self.id.get_full_name())?;
        s.serialize_field("data", &inner)?;
        s.end()
    }
}

pub struct Edge {
    id: i32,
    source: Name,
    target: Name,
    animated: bool,
}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Edge", 4)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("source", &self.source.get_full_name())?;
        s.serialize_field("target", &self.target.get_full_name())?;
        s.serialize_field("animated", &self.animated)?;
        s.end()
    }
}

pub struct EdgeGenerator {
    registed_edge_nums: i32,
}

impl EdgeGenerator {
    pub fn new() -> EdgeGenerator {
        EdgeGenerator {
            registed_edge_nums: 0,
        }
    }

    pub fn gen(&mut self, source: Name, target: Name, animated: bool) -> Edge {
        self.registed_edge_nums += 1;
        Edge {
            id: self.registed_edge_nums,
            source,
            target,
            animated,
        }
    }
}

#[macro_use]
pub mod macros {
    macro_rules! node {
        ($name:expr) => {
            Node::new(
                $name.clone(),
                format!("{}({})", $name.name.clone(), $name.get_full_name()),
            )
        };
    }

    macro_rules! edge {
        ($edge_generator:expr, $source:expr, $target:expr) => {
            $edge_generator.gen($source.clone(), $target.clone(), false)
        };
    }

    pub(crate) use edge;
    pub(crate) use node;
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use sysdc_parser::name::Name;

    use super::{EdgeGenerator, Node};

    #[test]
    fn node_serialize() {
        let node = Node::new(Name::new_root(), "test".to_string());
        compare(node, "{\"id\":\".0\",\"data\":{\"label\":\"test\"}}");
    }

    #[test]
    fn edge_serialize() {
        let source = Name::new(&Name::new_root(), "A".to_string());
        let target = Name::new(&Name::new_root(), "B".to_string());
        let edge = EdgeGenerator::new().gen(source, target, false);
        compare(
            edge,
            "{\"id\":1,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
    }

    fn compare<T>(node: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&node).unwrap(), json_str);
    }
}
