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
    source: Name,
    target: Name,
    animated: bool,
}

impl Edge {
    pub fn new(source: Name, target: Name, animated: bool) -> Edge {
        Edge {
            source,
            target,
            animated,
        }
    }
}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Edge", 4)?;
        s.serialize_field("id", "a")?;
        s.serialize_field("source", &self.source.get_full_name())?;
        s.serialize_field("target", &self.target.get_full_name())?;
        s.serialize_field("animated", &self.animated)?;
        s.end()
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use sysdc_parser::name::Name;

    use super::{Edge, Node};

    #[test]
    fn node_serialize() {
        let node = Node::new(Name::new_root(), "test".to_string());
        compare(node, "{\"id\":\".0\",\"data\":{\"label\":\"test\"}}");
    }

    #[test]
    fn edge_serialize() {
        let source = Name::new(&Name::new_root(), "A".to_string());
        let target = Name::new(&Name::new_root(), "B".to_string());
        let edge = Edge::new(source, target, false);
        compare(
            edge,
            "{\"id\":\"a\",\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
    }

    fn compare<T>(node: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&node).unwrap(), json_str);
    }
}
