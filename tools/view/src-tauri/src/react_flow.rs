use std::sync::Mutex;

use once_cell::sync::Lazy;
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
        s.serialize_field("id", &self.id.get_full_name().replace("._", ""))?;
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

impl Edge {
    pub fn new(source: Name, target: Name, animated: bool) -> Edge {
        static CREATED_EDGE_NUMS: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

        let mut id = CREATED_EDGE_NUMS.lock().unwrap();
        *id += 1;

        Edge {
            id: *id,
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
        s.serialize_field("id", &self.id)?;
        s.serialize_field("source", &self.source.get_full_name().replace("._", ""))?;
        s.serialize_field("target", &self.target.get_full_name().replace("._", ""))?;
        s.serialize_field("animated", &self.animated)?;
        s.end()
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
        ($source:expr, $target:expr) => {
            Edge::new($source.clone(), $target.clone(), false)
        };
    }

    pub(crate) use edge;
    pub(crate) use node;
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
        let edge_1 = Edge::new(source.clone(), target.clone(), false);
        let edge_2 = Edge::new(source, target, false);
        compare(
            edge_1,
            "{\"id\":1,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
        compare(
            edge_2,
            "{\"id\":2,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
    }

    fn compare<T>(node: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&node).unwrap(), json_str);
    }
}
