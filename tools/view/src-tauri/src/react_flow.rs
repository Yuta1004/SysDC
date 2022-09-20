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
    DeadVar,
    ReturnVar,
    AffectOuter,
    AffectInner,
    SpawnOuter,
    SpawnInner,
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

impl ReactFlowNode {
    pub fn new(kind: ReactFlowNodeKind, name: &Name, types: Option<&Type>) -> ReactFlowNode {
        let parent = match kind {
            ReactFlowNodeKind::Unit => None,
            ReactFlowNodeKind::Module
            | ReactFlowNodeKind::Function
            | ReactFlowNodeKind::Procedure
            | ReactFlowNodeKind::Argument
            | ReactFlowNodeKind::Var
            | ReactFlowNodeKind::DeadVar
            | ReactFlowNodeKind::ReturnVar => Some(name.get_par_name(true).get_full_name()),
            _ => panic!("Internal error"),
        };
        let data = if let Some(types) = types {
            (name.clone(), types.clone())
        } else {
            (name.clone(), Type::from("void".to_string()))
        };

        ReactFlowNode {
            id: name.get_full_name(),
            kind,
            parent,
            data: ReactFlowNodeData::new(Some(data)),
        }
    }

    pub fn new_with_full(
        id: String,
        kind: ReactFlowNodeKind,
        parent: Option<String>,
        data: ReactFlowNodeData,
    ) -> ReactFlowNode {
        ReactFlowNode {
            id,
            kind,
            parent,
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ReactFlowNodeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Name>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    types: Option<Type>,
}

impl ReactFlowNodeData {
    pub fn new(info: Option<(Name, Type)>) -> ReactFlowNodeData {
        if let Some((name, types)) = info {
            ReactFlowNodeData {
                name: Some(name),
                types: Some(types),
            }
        } else {
            ReactFlowNodeData {
                name: None,
                types: None,
            }
        }
    }
}

#[derive(Serialize)]
pub struct ReactFlowEdge {
    id: String,
    source: String,
    target: String,
}

impl ReactFlowEdge {
    pub fn new(source: String, target: String) -> ReactFlowEdge {
        ReactFlowEdge {
            id: format!("{}/{}", source, target),
            source,
            target,
        }
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use super::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeData, ReactFlowNodeKind};
    use sysdc_parser::name::Name;
    use sysdc_parser::types::Type;

    #[test]
    fn node_serialize_1() {
        let has_parent_node = ReactFlowNode::new_with_full(
            ".0.test".to_string(),
            ReactFlowNodeKind::Var,
            Some(".0".to_string()),
            ReactFlowNodeData::new(Some((
                Name::new(&Name::new_root(), "test".to_string()),
                Type::from("void".to_string()),
            ))),
        );
        compare(
            has_parent_node,
            "{\"id\":\".0.test\",\"type\":\"Var\",\"parentNode\":\".0\",\"data\":{\"name\":{\"name\":\"test\",\"namespace\":\".0\"},\"type\":{\"kind\":\"void\",\"refs\":null}}}",
        );
    }

    #[test]
    fn node_serialize_2() {
        let hasnt_parent_node = ReactFlowNode::new_with_full(
            ".0.test".to_string(),
            ReactFlowNodeKind::Var,
            None,
            ReactFlowNodeData::new(Some((
                Name::new(&Name::new_root(), "test".to_string()),
                Type::from("void".to_string()),
            ))),
        );
        compare(
            hasnt_parent_node,
        "{\"id\":\".0.test\",\"type\":\"Var\",\"data\":{\"name\":{\"name\":\"test\",\"namespace\":\".0\"},\"type\":{\"kind\":\"void\",\"refs\":null}}}",
        );
    }

    #[test]
    fn edge_serialize() {
        let edge = ReactFlowEdge {
            id: "test".to_string(),
            source: ".0.A".to_string(),
            target: ".0.B".to_string(),
        };
        compare(
            edge,
            "{\"id\":\"test\",\"source\":\".0.A\",\"target\":\".0.B\"}",
        );
    }

    fn compare<T>(elem: T, json_str: &str)
    where
        T: Serialize,
    {
        assert_eq!(serde_json::to_string(&elem).unwrap(), json_str);
    }
}
