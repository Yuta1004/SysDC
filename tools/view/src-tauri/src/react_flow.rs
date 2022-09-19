use serde::ser::SerializeStruct;
use serde::Serialize;

use sysdc_parser::name::Name;

#[derive(Serialize)]
pub enum ReactFlowNodeKind {
    Unit,
    Module,
    Function,
    Procedure,
    Argument,
    Var,
    ReturnVar,
    SpawnInner,
    SpawnOuter,
    AffectInner,
    AffectOuter,
}

#[derive(Serialize)]
pub struct ReactFlowNode {
    pub id: String,

    #[serde(rename(serialize = "type"))]
    pub kind: ReactFlowNodeKind,

    #[serde(
        rename(serialize = "parentNode"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<String>,

    pub data: ReactFlowNodeData,
}

#[derive(Serialize)]
pub struct ReactFlowNodeData {
    pub label: String,
}

#[derive(Serialize)]
pub struct ReactFlowEdge {
    pub id: i32,
    pub source: String,
    pub target: String,
    pub animated: bool,
}

pub mod macros {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    pub(crate) static CREATED_EDGE_NUMS: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    macro_rules! node {
        ($kind:expr, $name:expr) => {
            match $kind {
                ReactFlowNodeKind::Module
                | ReactFlowNodeKind::Function
                | ReactFlowNodeKind::Procedure
                | ReactFlowNodeKind::Argument
                | ReactFlowNodeKind::Var
                | ReactFlowNodeKind::ReturnVar => ReactFlowNode {
                    id: $name.get_full_name().replace("._", ""),
                    kind: $kind,
                    parent: Some($name.get_par_name(true).get_full_name()),
                    data: ReactFlowNodeData {
                        label: format!("{}({})", $name.name.clone(), $name.get_full_name()),
                    },
                },
                _ => ReactFlowNode {
                    id: $name.get_full_name().replace("._", ""),
                    kind: $kind,
                    parent: None,
                    data: ReactFlowNodeData {
                        label: format!("{}({})", $name.name.clone(), $name.get_full_name()),
                    },
                },
            }
        };
    }

    macro_rules! edge {
        ($source:expr, $target:expr) => {{
            let mut id = crate::react_flow::macros::CREATED_EDGE_NUMS.lock().unwrap();
            *id += 1;

            ReactFlowEdge {
                id: *id,
                source: $source.get_full_name().replace("._", ""),
                target: $target.get_full_name().replace("._", ""),
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
    use super::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeData, ReactFlowNodeKind};

    #[test]
    fn node_serialize() {
        let name = Name::new(&Name::new_root(), "test".to_string());
        compare(
            node!(ReactFlowNodeKind::Var, &name),
            "{\"id\":\".0.test\",\"type\":\"Var\",\"parentNode\":\".0\",\"data\":{\"label\":\"test(.0.test)\"}}",
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
