use std::sync::Mutex;

use once_cell::sync::Lazy;
use serde::Serialize;

use sysdc_parser::name::Name;
use sysdc_parser::types::Type;

pub type ReactFlowDesign = (Vec<ReactFlowNode>, Vec<ReactFlowEdge>);

#[derive(Serialize)]
pub enum ReactFlowNodeKind {
    Unit,
    Module,
    Function,
    Procedure,
    Argument,
    Var,
    ReturnVar,
    SpawnOuter,
    SpawnInner,
    AffectOuter,
    AffectInner,
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
    id: String,
    source: String,
    target: String,
    animated: bool,
}

pub fn node(kind: ReactFlowNodeKind, name: &Name) -> ReactFlowNode {
    let parent = match kind {
        ReactFlowNodeKind::Unit => None,
        ReactFlowNodeKind::Module
        | ReactFlowNodeKind::Function
        | ReactFlowNodeKind::Procedure
        | ReactFlowNodeKind::Argument
        | ReactFlowNodeKind::Var
        | ReactFlowNodeKind::ReturnVar => Some(name.get_par_name(true).get_full_name()),
        _ => panic!("Internal error"),
    };

    ReactFlowNode {
        id: name.get_full_name(),
        kind,
        parent,
        data: ReactFlowNodeData {
            label: format!("{}({})", name.name, name.get_full_name()),
        },
    }
}

pub fn node_spawn(result: &Name) -> Vec<ReactFlowNode> {
    let result_par = result.get_par_name(true).get_full_name();
    let result = result.get_full_name();

    let inner = ReactFlowNode {
        id: format!("{}:s:inner", result),
        kind: ReactFlowNodeKind::SpawnInner,
        parent: Some(format!("{}:s:outer", result)),
        data: ReactFlowNodeData {
            label: "".to_string(),
        },
    };
    let outer = ReactFlowNode {
        id: format!("{}:s:outer", result),
        kind: ReactFlowNodeKind::SpawnOuter,
        parent: Some(result_par.clone()),
        data: ReactFlowNodeData {
            label: "".to_string(),
        },
    };
    let resultn = ReactFlowNode {
        id: result,
        kind: ReactFlowNodeKind::Var,
        parent: Some(result_par),
        data: ReactFlowNodeData {
            label: "test".to_string()
        }
    };

    vec![inner, outer, resultn]
}

pub fn edge_spawn(name: &Name, func: &Name, args: &Vec<(Name, Type)>) -> Vec<ReactFlowEdge> {
    let name = name.get_full_name();
    let func = func.get_full_name();

    let mut edges = vec![];

    // E: uses -> outer
    for (aname, _) in args {
        edges.push(ReactFlowEdge {
            id: format!("{}/{}:s:outer", aname.get_full_name(), name),
            source: aname.get_full_name(),
            target: format!("{}:s:outer", name),
            animated: false,
        });
    }

    // E: inner -> func
    edges.push(ReactFlowEdge {
        id: format!("{}:s:inner/{}", name, func),
        source: format!("{}:s:inner", name),
        target: func.clone(),
        animated: false,
    });
    edges.push(ReactFlowEdge {
        id: format!("{}/{}:s:inner", func, name),
        source: func,
        target: format!("{}:s:inner", name),
        animated: false,
    });

    // E: outer -> result
    edges.push(ReactFlowEdge {
        id: format!("{}:s:outer/{}", name, name),
        source: format!("{}:s:outer", name),
        target: name,
        animated: false,
    });

    edges
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use super::{ReactFlowNode, ReactFlowNodeData, ReactFlowNodeKind, ReactFlowEdge};

    #[test]
    fn node_serialize_1() {
        let has_parent_node = ReactFlowNode {
            id: ".0.test".to_string(),
            kind: ReactFlowNodeKind::Var,
            parent: Some(".0".to_string()),
            data: ReactFlowNodeData {
                label: "test".to_string(),
            },
        };
        compare(
            has_parent_node,
            "{\"id\":\".0.test\",\"type\":\"Var\",\"parentNode\":\".0\",\"data\":{\"label\":\"test\"}}",
        );
    }

    #[test]
    fn node_serialize_2() {
        let hasnt_parent_node = ReactFlowNode {
            id: ".0.test".to_string(),
            kind: ReactFlowNodeKind::Var,
            parent: None,
            data: ReactFlowNodeData {
                label: "test".to_string(),
            },
        };
        compare(
            hasnt_parent_node,
            "{\"id\":\".0.test\",\"type\":\"Var\",\"data\":{\"label\":\"test\"}}",
        );
    }

    #[test]
    fn edge_serialize() {
        let edge = ReactFlowEdge {
            id: "test".to_string(),
            source: ".0.A".to_string(),
            target: ".0.B".to_string(),
            animated: false
        };
        compare(
            edge,
            "{\"id\":\"test\",\"source\":\".0.A\",\"target\":\".0.B\",\"animated\":false}",
        );
    }

    fn compare<T>(elem: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&elem).unwrap(), json_str);
    }
}
