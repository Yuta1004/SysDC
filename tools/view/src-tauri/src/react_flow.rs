use std::sync::Mutex;

use once_cell::sync::Lazy;
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
    id: String,

    #[serde(rename(serialize = "type"))]
    kind: ReactFlowNodeKind,

    #[serde(
        rename(serialize = "parentNode"),
        skip_serializing_if = "Option::is_none"
    )]
    parent: Option<String>,

    data: ReactFlowNodeData,
}

#[derive(Serialize)]
pub struct ReactFlowNodeData {
    label: String,
}

#[derive(Serialize)]
pub struct ReactFlowEdge {
    id: i32,
    source: String,
    target: String,
    animated: bool,
}

pub fn node(kind: ReactFlowNodeKind, name: &Name) -> ReactFlowNode {
    match kind {
        ReactFlowNodeKind::Module
        | ReactFlowNodeKind::Function
        | ReactFlowNodeKind::Procedure
        | ReactFlowNodeKind::Argument
        | ReactFlowNodeKind::Var
        | ReactFlowNodeKind::ReturnVar => ReactFlowNode {
            id: name.get_full_name().replace("._", ""),
            kind,
            parent: Some(name.get_par_name(true).get_full_name()),
            data: ReactFlowNodeData {
                label: format!("{}({})", name.name, name.get_full_name()),
            },
        },
        _ => ReactFlowNode {
            id: name.get_full_name().replace("._", ""),
            kind,
            parent: None,
            data: ReactFlowNodeData {
                label: format!("{}({})", name.name, name.get_full_name()),
            },
        },
    }
}

pub fn edge(source: &Name, target: &Name) -> ReactFlowEdge {
    static CREATED_EDGE_NUMS: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    let mut id = CREATED_EDGE_NUMS.lock().unwrap();
    *id += 1;

    ReactFlowEdge {
        id: *id,
        source: source.get_full_name().replace("._", ""),
        target: target.get_full_name().replace("._", ""),
        animated: false,
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use sysdc_parser::name::Name;

    use super::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeKind};

    #[test]
    fn node_serialize() {
        let name = Name::new(&Name::new_root(), "test".to_string());
        compare(
            super::node(ReactFlowNodeKind::Var, &name),
            "{\"id\":\".0.test\",\"type\":\"Var\",\"parentNode\":\".0\",\"data\":{\"label\":\"test(.0.test)\"}}",
        );
    }

    #[test]
    fn edge_serialize() {
        let source = Name::new(&Name::new_root(), "A".to_string());
        let target = Name::new(&Name::new_root(), "B".to_string());
        compare(
            super::edge(&source, &target),
            "{\"id\":1,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
        compare(
            super::edge(&source, &target),
            "{\"id\":2,\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
    }

    fn compare<T>(elem: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&elem).unwrap(), json_str);
    }
}
