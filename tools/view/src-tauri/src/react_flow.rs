use serde::Serialize;

use sysdc_parser::name::Name;

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
}

impl ReactFlowNode {
    pub fn new(kind: ReactFlowNodeKind, name: &Name) -> ReactFlowNode {
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

        ReactFlowNode {
            id: name.get_full_name(),
            kind,
            parent,
        }
    }

    pub fn new_with_full(
        id: String,
        kind: ReactFlowNodeKind,
        parent: Option<String>,
    ) -> ReactFlowNode {
        ReactFlowNode { id, kind, parent }
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

    use super::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeKind};

    #[test]
    fn node_serialize_1() {
        let has_parent_node = ReactFlowNode {
            id: ".0.test".to_string(),
            kind: ReactFlowNodeKind::Var,
            parent: Some(".0".to_string()),
        };
        compare(
            has_parent_node,
            "{\"id\":\".0.test\",\"type\":\"Var\",\"parentNode\":\".0\"}",
        );
    }

    #[test]
    fn node_serialize_2() {
        let hasnt_parent_node = ReactFlowNode {
            id: ".0.test".to_string(),
            kind: ReactFlowNodeKind::Var,
            parent: None,
        };
        compare(hasnt_parent_node, "{\"id\":\".0.test\",\"type\":\"Var\"}");
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
