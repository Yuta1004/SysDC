use tauri::State;

use super::super::react_flow;
use super::super::react_flow::{ReactFlowDesign, ReactFlowNode, ReactFlowNodeKind};
use sysdc_parser::structure::{
    SysDCAnnotation, SysDCFunction, SysDCModule, SysDCSpawnDetail, SysDCSystem, SysDCUnit,
};
use sysdc_parser::types::{Type, TypeKind};

#[tauri::command]
pub fn get_flow(system: State<'_, SysDCSystem>) -> ReactFlowDesign {
    system.units.iter().map(|unit| gen_unit_flow(unit)).fold(
        (vec![], vec![]),
        |(mut nodes, mut edges), (_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
            (nodes, edges)
        },
    )
}

fn gen_unit_flow(unit: &SysDCUnit) -> ReactFlowDesign {
    unit.modules
        .iter()
        .map(|module| gen_module_flow(module))
        .fold(
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
    module
        .functions
        .iter()
        .map(|func| gen_func_flow(func))
        .fold(
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
    let mut nodes = vec![];
    let mut edges = vec![];

    if let SysDCAnnotation::Affect { func: afunc, args } = annotation {
        nodes.extend(react_flow::node_affect(&func.name, &afunc.0));
        edges.extend(react_flow::edge_affect(&func.name, &afunc.0, args));
    }

    if let SysDCAnnotation::Spawn { details, .. } = annotation {
        for detail in details {
            if let SysDCSpawnDetail::LetTo { name, func, args } = detail {
                nodes.extend(react_flow::node_spawn(name));
                edges.extend(react_flow::edge_spawn(name, &func.0, args))
            }
        }
    }

    if let SysDCAnnotation::Modify { target, uses } = annotation {
        // unimplemented!();
    }

    (nodes, edges)
}
