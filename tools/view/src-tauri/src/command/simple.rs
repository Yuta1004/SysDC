use tauri::State;

use super::super::react_flow::macros::{edge, node};
use super::super::react_flow::{Edge, EdgeGenerator, Node};
use super::super::SysDCSystemWrapper;
use sysdc_parser::structure::{SysDCAnnotation, SysDCFunction, SysDCSpawnDetail};

#[tauri::command]
pub fn get_flow(system: State<'_, SysDCSystemWrapper>) -> (Vec<Node>, Vec<Edge>) {
    let mut nodes = vec![];
    let mut edges = vec![];
    let mut edge_generator = EdgeGenerator::new();

    let system = system.get();
    for unit in &system.units {
        for module in &unit.modules {
            for func in &module.functions {
                gen_func_flow(func, &mut nodes, &mut edges, &mut edge_generator);
            }
        }
    }

    (nodes, edges)
}

fn gen_func_flow(
    func: &SysDCFunction,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    edge_generator: &mut EdgeGenerator,
) {
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
                    edges.push(edge!(edge_generator, name, target.0));
                }
            }
            SysDCAnnotation::Spawn { result, details } => {
                for detail in details {
                    match detail {
                        SysDCSpawnDetail::Use(name, _) => {
                            edges.push(edge!(edge_generator, name, result.0));
                        }
                        SysDCSpawnDetail::LetTo {
                            name: var, args, ..
                        } => {
                            edges.extend(
                                args.iter()
                                    .map(|(name, _)| edge!(edge_generator, name, var))
                                    .collect::<Vec<Edge>>(),
                            );
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
