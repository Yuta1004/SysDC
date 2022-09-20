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

#[derive(Serialize)]
pub struct ReactFlowEdge {
    id: String,
    source: String,
    target: String,
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
    }
}

pub fn node_affect(func: &Name, afunc: &Name) -> Vec<ReactFlowNode> {
    let name_par = func.get_full_name();
    let name = format!("{}:{}:affect", func.get_full_name(), afunc.get_full_name());

    let inner = ReactFlowNode {
        id: format!("{}:inner", name),
        kind: ReactFlowNodeKind::AffectInner,
        parent: Some(format!("{}:outer", name)),
    };
    let outer = ReactFlowNode {
        id: format!("{}:outer", name),
        kind: ReactFlowNodeKind::AffectOuter,
        parent: Some(name_par),
    };

    vec![inner, outer]
}

pub fn edge_affect(func: &Name, afunc: &Name, args: &Vec<(Name, Type)>) -> Vec<ReactFlowEdge> {
    let name = format!("{}:{}:affect", func.get_full_name(), afunc.get_full_name());
    let afunc = afunc.get_full_name();

    let mut edges = vec![];

    // E: uses -> outer
    for (aname, _) in args {
        edges.push(ReactFlowEdge {
            id: format!("{}/{}:outer", aname.get_full_name(), name),
            source: aname.get_full_name(),
            target: format!("{}:outer", name),
        });
    }

    // E: inner -> func
    edges.push(ReactFlowEdge {
        id: format!("{}:inner/{}", name, afunc),
        source: format!("{}:inner", name),
        target: afunc.clone(),
    });

    edges
}

pub fn node_spawn(result: &Name) -> Vec<ReactFlowNode> {
    let result_par = result.get_par_name(true).get_full_name();
    let result = result.get_full_name();

    let inner = ReactFlowNode {
        id: format!("{}:inner", result),
        kind: ReactFlowNodeKind::SpawnInner,
        parent: Some(format!("{}:outer", result)),
    };
    let outer = ReactFlowNode {
        id: format!("{}:outer", result),
        kind: ReactFlowNodeKind::SpawnOuter,
        parent: Some(result_par.clone()),
    };
    let resultn = ReactFlowNode {
        id: result,
        kind: ReactFlowNodeKind::Var,
        parent: Some(result_par),
    };

    vec![inner, outer, resultn]
}

pub fn edge_spawn(name: &Name, func: &Name, args: &Vec<(Name, Type)>) -> Vec<ReactFlowEdge> {
    let mut edges = vec![];

    let name = name.get_full_name();
    let func = func.get_full_name();

    // E: uses -> outer
    for (aname, _) in args {
        edges.push(ReactFlowEdge {
            id: format!("{}/{}:outer", aname.get_full_name(), name),
            source: aname.get_full_name(),
            target: format!("{}:outer", name),
        });
    }

    // E: inner -> func, func -> inner
    edges.push(ReactFlowEdge {
        id: format!("{}:inner/{}", name, func),
        source: format!("{}:inner", name),
        target: func.clone(),
    });
    edges.push(ReactFlowEdge {
        id: format!("{}/{}:inner", func, name),
        source: func,
        target: format!("{}:inner", name),
    });

    // E: outer -> result
    edges.push(ReactFlowEdge {
        id: format!("{}:outer/{}", name, name),
        source: format!("{}:outer", name),
        target: name,
    });

    edges
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
