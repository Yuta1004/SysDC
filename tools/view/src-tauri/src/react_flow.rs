use serde::ser::SerializeStruct;
use serde::Serialize;

use sysdc_parser::name::Name;

#[derive(Serialize)]
pub enum ReactFlowNodeKind {
    Unit,
    Module,
    Function,
    Var,
    SpawnInner,
    SpawnOuter,
    AffectInner,
    AffectOuter,
}

pub struct ReactFlowNode {
    pub(super) id: Name,
    pub(super) kind: ReactFlowNodeKind,
    pub(super) label: String,
}

impl Serialize for ReactFlowNode {
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
        s.serialize_field("type", &self.kind)?;
        s.serialize_field("data", &inner)?;
        s.end()
    }
}

pub struct ReactFlowEdge {
    pub(super) id: i32,
    pub(super) source: Name,
    pub(super) target: Name,
    pub(super) animated: bool,
}

impl Serialize for ReactFlowEdge {
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
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    pub(crate) static CREATED_EDGE_NUMS: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    macro_rules! node {
        ($name:expr) => {
            node!(ReactFlowNodeKind::Var, $name)
        };

        ($kind:expr, $name:expr) => {
            ReactFlowNode {
                id: $name.clone(),
                kind: $kind,
                label: format!("{}({})", $name.name.clone(), $name.get_full_name()),
            }
        };
    }

    macro_rules! edge {
        ($source:expr, $target:expr) => {{
            let mut id = crate::react_flow::macros::CREATED_EDGE_NUMS.lock().unwrap();
            *id += 1;

            ReactFlowEdge {
                id: *id,
                source: $source.clone(),
                target: $target.clone(),
                animated: false,
            }
        }};
    }

    pub(crate) use edge;
    pub(crate) use node;
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use sysdc_parser::name::Name;

    use super::macros::{edge, node};
    use super::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeKind};

    #[test]
    fn node_serialize() {
        let name = Name::new(&Name::new_root(), "test".to_string());
        compare(
            node!(&name),
            "{\"id\":\".0.test\",\"type\":\"Var\",\"data\":{\"label\":\"test(.0.test)\"}}",
        );
        compare(
            node!(ReactFlowNodeKind::Var, &name),
            "{\"id\":\".0.test\",\"type\":\"Var\",\"data\":{\"label\":\"test(.0.test)\"}}",
        );
    }

    #[test]
    fn edge_serialize() {
        let source = Name::new(&Name::new_root(), "A".to_string());
        let target = Name::new(&Name::new_root(), "B".to_string());
        compare(
            edge!(&source, &target),
            "{\"id\":1,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
        compare(
            edge!(&source, &target),
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
