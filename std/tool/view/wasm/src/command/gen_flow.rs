use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use sysdc_core::name::Name;
use sysdc_core::structure::{
    SysDCAnnotation, SysDCFunction, SysDCModule, SysDCSpawnDetail, SysDCSystem, SysDCUnit,
};
use sysdc_core::types::{Type, TypeKind};
use super::super::react_flow::{
    ReactFlowDesign, ReactFlowEdge, ReactFlowNode, ReactFlowNodeData, ReactFlowNodeKind,
};

#[wasm_bindgen]
pub fn gen_flow(system: &str) -> Result<JsValue, String> {
    match serde_json::from_str::<SysDCSystem>(system) {
        Ok(system) => {
            let design = system.units.iter().map(gen_unit_flow).fold(
                (vec![], vec![]),
                |(mut nodes, mut edges), (_nodes, _edges)| {
                    nodes.extend(_nodes);
                    edges.extend(_edges);
                    (nodes, edges)
                },
            );
            Ok(serde_wasm_bindgen::to_value(&design).unwrap())
        }
        Err(err) => Err(err.to_string())
    }
}

fn gen_unit_flow(unit: &SysDCUnit) -> ReactFlowDesign {
    unit.modules.iter().map(gen_module_flow).fold(
        (
            vec![ReactFlowNode::new(
                ReactFlowNodeKind::Unit,
                &unit.name,
                None,
            )],
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
            vec![ReactFlowNode::new(
                ReactFlowNodeKind::Module,
                &module.name,
                None,
            )],
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

    let is_procedure = func.returns.1.kind == TypeKind::Void;

    if is_procedure {
        nodes.push(ReactFlowNode::new(
            ReactFlowNodeKind::Procedure,
            &func.name,
            None,
        ));
    } else {
        nodes.push(ReactFlowNode::new(
            ReactFlowNodeKind::Function,
            &func.name,
            None,
        ));
    }

    func.args.iter().for_each(|(name, types)| {
        nodes.push(ReactFlowNode::new(
            ReactFlowNodeKind::Argument,
            name,
            Some(types),
        ))
    });

    func.annotations
        .iter()
        .map(|annotation| gen_annotation_flow(func, annotation))
        .for_each(|(_nodes, _edges)| {
            nodes.extend(_nodes);
            edges.extend(_edges);
        });

    if !is_procedure {
        nodes.push(ReactFlowNode::new(
            ReactFlowNodeKind::ReturnVar,
            &func.returns.0,
            Some(&func.returns.1),
        ));
    }

    (nodes, edges)
}

fn gen_annotation_flow(func: &SysDCFunction, annotation: &SysDCAnnotation) -> ReactFlowDesign {
    if let SysDCAnnotation::Affect { func: afunc, args } = annotation {
        return gen_annotation_affect_flow(&func.name, &afunc.0, args);
    }

    if let SysDCAnnotation::Spawn { result, details } = annotation {
        let uses = details
            .iter()
            .filter_map(|detail| {
                if let SysDCSpawnDetail::Use(name, types) = detail {
                    Some((name.clone(), types.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(Name, Type)>>();

        return if uses.len() == details.len() {
            gen_annotation_spawn_flow(result, &Name::new_root(), &uses)
        } else {
            let (mut nodes, mut edges) = details
                .iter()
                .filter_map(|detail| {
                    if let SysDCSpawnDetail::LetTo { name, func, args } = detail {
                        Some(gen_annotation_spawn_flow(
                            &(name.clone(), func.1.clone()),
                            &func.0,
                            args,
                        ))
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
                );
            if let SysDCSpawnDetail::Return(name, _) = &details[details.len() - 1] {
                nodes.push(ReactFlowNode::new(
                    ReactFlowNodeKind::Var,
                    &result.0,
                    Some(&result.1),
                ));
                edges.push(ReactFlowEdge::new(
                    name.get_full_name(),
                    result.0.get_full_name(),
                ));
            }
            (nodes, edges)
        };
    }

    if let SysDCAnnotation::Modify { target, uses } = annotation {
        let dead_var_node = ReactFlowNode::new_with_full(
            format!("{}:dead", target.0.get_full_name()),
            ReactFlowNodeKind::DeadVar,
            Some(func.name.get_full_name()),
            ReactFlowNodeData::new(Some(target.clone())),
        );

        let mut uses = uses.clone();
        uses.push((
            Name::new(
                &target.0.get_par_name(true),
                format!("{}:dead", target.0.name),
            ),
            target.1.clone(),
        ));

        let (mut nodes, edges) = gen_annotation_spawn_flow(target, &Name::new_root(), &uses);
        nodes.push(dead_var_node);

        return (nodes, edges);
    }

    (vec![], vec![])
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
            ReactFlowNodeData::new(None),
        ));

        // N: outer
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:outer", name),
            ReactFlowNodeKind::AffectOuter,
            Some(name_par),
            ReactFlowNodeData::new(None),
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
    result: &(Name, Type),
    func: &Name,
    args: &Vec<(Name, Type)>,
) -> ReactFlowDesign {
    let mut nodes = vec![];
    let mut edges = vec![];

    {
        let result_par_fn = result.0.get_par_name(true).get_full_name();
        let result_fn = result.0.get_full_name();

        // N: inner
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:inner", result_fn),
            ReactFlowNodeKind::SpawnInner,
            Some(format!("{}:outer", result_fn)),
            ReactFlowNodeData::new(None),
        ));

        // N: outer
        nodes.push(ReactFlowNode::new_with_full(
            format!("{}:outer", result_fn),
            ReactFlowNodeKind::SpawnOuter,
            Some(result_par_fn.clone()),
            ReactFlowNodeData::new(None),
        ));

        // N: result
        nodes.push(ReactFlowNode::new_with_full(
            result_fn,
            ReactFlowNodeKind::Var,
            Some(result_par_fn),
            ReactFlowNodeData::new(Some(result.clone())),
        ));
    }

    {
        let result = result.0.get_full_name();
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
