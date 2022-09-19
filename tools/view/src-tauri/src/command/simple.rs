use tauri::State;

use super::super::react_flow::macros::{edge, node};
use super::super::react_flow::{ReactFlowEdge, ReactFlowNode, ReactFlowNodeKind};
use sysdc_parser::structure::{SysDCAnnotation, SysDCFunction, SysDCSpawnDetail, SysDCSystem};

#[tauri::command]
pub fn get_flow(system: State<'_, SysDCSystem>) -> (Vec<ReactFlowNode>, Vec<ReactFlowEdge>) {
    let mut nodes = vec![];
    let mut edges = vec![];

    for unit in &system.units {
        nodes.push(node!(ReactFlowNodeKind::Unit, unit.name));
        for module in &unit.modules {
            nodes.push(node!(ReactFlowNodeKind::Module, module.name));
            for func in &module.functions {
                nodes.push(node!(ReactFlowNodeKind::Function, func.name));
                let (fnodes, fedges) = gen_func_flow(func);
                nodes.extend(fnodes);
                edges.extend(fedges);
            }
        }
    }

    (nodes, edges)
}

fn gen_func_flow(func: &SysDCFunction) -> (Vec<ReactFlowNode>, Vec<ReactFlowEdge>) {
    let mut nodes = vec![];
    let mut edges = vec![];

    // Node
    for (name, _) in &func.args {
        nodes.push(node!(name));
    }
    for annotation in &func.annotations {
        match annotation {
            SysDCAnnotation::Spawn { result, details } => {
                nodes.push(node!(result.0));
                for detail in details {
                    match detail {
                        SysDCSpawnDetail::Use(name, _) => nodes.push(node!(name)),
                        SysDCSpawnDetail::LetTo { name, .. } => nodes.push(node!(name)),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // Edge
    for annotation in &func.annotations {
        match annotation {
            SysDCAnnotation::Modify { target, uses } => {
                for (name, _) in uses {
                    edges.push(edge!(name, target.0));
                }
            }
            SysDCAnnotation::Spawn { result, details } => {
                for detail in details {
                    match detail {
                        SysDCSpawnDetail::Use(name, _) => {
                            edges.push(edge!(name, result.0));
                        }
                        SysDCSpawnDetail::LetTo {
                            name: var, args, ..
                        } => {
                            edges.extend(
                                args.iter()
                                    .map(|(name, _)| edge!(name, var))
                                    .collect::<Vec<ReactFlowEdge>>(),
                            );
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    (nodes, edges)
}
