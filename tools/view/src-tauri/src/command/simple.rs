use tauri::State;

use super::super::react_flow::{ReactFlowDesign, ReactFlowEdge, ReactFlowNode, ReactFlowNodeKind};
use sysdc_parser::name::Name;
use sysdc_parser::structure::{
    SysDCAnnotation, SysDCFunction, SysDCModule, SysDCSpawnDetail, SysDCSystem, SysDCUnit,
};
use sysdc_parser::types::{Type, TypeKind};

#[tauri::command]
pub fn get_flow(system: State<'_, SysDCSystem>) -> ReactFlowDesign {
    system.units.iter().map(gen_unit_flow).fold(
        (vec![], vec![]),
        |(mut nodes, mut edges), (_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
            (nodes, edges)
        },
    )
}

fn gen_unit_flow(unit: &SysDCUnit) -> ReactFlowDesign {
    unit.modules.iter().map(gen_module_flow).fold(
        (
            vec![ReactFlowNode::new(ReactFlowNodeKind::Unit, &unit.name)],
            vec![],
        ),
        |(mut nodes, mut edges), (_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
            (nodes, edges)
        },
    )
}

fn gen_module_flow(module: &SysDCModule) -> ReactFlowDesign {
    module.functions.iter().map(gen_func_flow).fold(
        (
            vec![ReactFlowNode::new(ReactFlowNodeKind::Module, &module.name)],
            vec![],
        ),
        |(mut nodes, mut edges), (_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
            (nodes, edges)
        },
    )
}

fn gen_func_flow(func: &SysDCFunction) -> ReactFlowDesign {
    let mut nodes = vec![];
    let mut edges = vec![];

    if let (
        _,
        Type {
            kind: TypeKind::Void,
            ..
        },
    ) = func.returns
    {
        nodes.push(ReactFlowNode::new(ReactFlowNodeKind::Procedure, &func.name));
    } else {
        nodes.push(ReactFlowNode::new(ReactFlowNodeKind::Function, &func.name));
        nodes.push(ReactFlowNode::new(
            ReactFlowNodeKind::ReturnVar,
            &func.returns.0,
        ));
    }

    func.args
        .iter()
        .for_each(|(name, _)| nodes.push(ReactFlowNode::new(ReactFlowNodeKind::Argument, name)));

    func.annotations
        .iter()
        .map(|annotation| gen_annotation_flow(func, annotation))
        .for_each(|(_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
        });

    (nodes, edges)
}

fn gen_annotation_flow(func: &SysDCFunction, annotation: &SysDCAnnotation) -> ReactFlowDesign {
    match annotation {
        SysDCAnnotation::Affect { func: afunc, args } => {
            gen_annotation_affect_flow(&func.name, &afunc.0, args)
        }
        SysDCAnnotation::Spawn { details, .. } => details
            .iter()
            .filter_map(|detail| {
                if let SysDCSpawnDetail::LetTo { name, func, args } = detail {
                    Some(gen_annotation_spawn_flow(name, &func.0, args))
                } else {
                    None
                }
            })
            .fold(
                (vec![], vec![]),
                |(mut nodes, mut edges), (_nodes, _edges)| {
                    nodes.extend(_nodes);
                    edges.extend(_edges);
                    (nodes, edges)
                },
            ),
        _ => (vec![], vec![]),
    }
}

pub fn gen_annotation_affect_flow(
    func: &Name,
    afunc: &Name,
    args: &Vec<(Name, Type)>,
) -> ReactFlowDesign {
    let mut nodes = vec![];
    let mut edges = vec![];

    {
        let name_par = func.get_full_name();
        let name = format!("{}:{}:affect", func.get_full_name(), afunc.get_full_name());

        // N: inner
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:inner", name),
            ReactFlowNodeKind::AffectInner,
            Some(format!("{}:outer", name)),
        ));

        // N: outer
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:outer", name),
            ReactFlowNodeKind::AffectOuter,
            Some(name_par),
        ));
    }

    {
        let name = format!("{}:{}:affect", func.get_full_name(), afunc.get_full_name());
        let afunc = afunc.get_full_name();

        // E: uses -> outer
        for (aname, _) in args {
            edges.push(ReactFlowEdge::new(
                aname.get_full_name(),
                format!("{}:outer", name),
            ));
        }

        // E: inner -> func
        edges.push(ReactFlowEdge::new(format!("{}:inner", name), afunc));
    };

    (nodes, edges)
}

pub fn gen_annotation_spawn_flow(
    result: &Name,
    func: &Name,
    args: &Vec<(Name, Type)>,
) -> ReactFlowDesign {
    let mut nodes = vec![];
    let mut edges = vec![];

    {
        let result_par = result.get_par_name(true).get_full_name();
        let result = result.get_full_name();

        // N: inner
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:inner", result),
            ReactFlowNodeKind::SpawnInner,
            Some(format!("{}:outer", result)),
        ));

        // N: outer
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:outer", result),
            ReactFlowNodeKind::SpawnOuter,
            Some(result_par.clone()),
        ));

        // N: result
        nodes.push(ReactFlowNode::new_with_full(
            result,
            ReactFlowNodeKind::Var,
            Some(result_par),
        ));
    }

    {
        let result = result.get_full_name();
        let func = func.get_full_name();

        // E: uses -> outer
        for (aname, _) in args {
            edges.push(ReactFlowEdge::new(
                aname.get_full_name(),
                format!("{}:outer", result),
            ));
        }

        // E: inner -> func, func -> inner
        edges.push(ReactFlowEdge::new(
            format!("{}:inner", result),
            func.clone(),
        ));
        edges.push(ReactFlowEdge::new(func, format!("{}:inner", result)));

        // E: outer -> result
        edges.push(ReactFlowEdge::new(format!("{}:outer", result), result));
    }

    (nodes, edges)
}
