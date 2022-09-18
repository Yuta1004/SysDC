use tauri::State;

use sysdc_parser::structure::SysDCSystem;
use sysdc_parser::name::Name;

use super::SysDCSystemWrapper;
use super::react_flow::{Node, Edge};

#[tauri::command]
pub fn get_system(manager: State<'_, SysDCSystemWrapper>) -> SysDCSystem {
    (*manager.get()).clone()
}

#[tauri::command]
pub fn get_nodes(_: State<'_, SysDCSystemWrapper>) -> Vec<Node> {
    vec![
        Node::new(Name::new(&Name::new_root(), "A".to_string()), "TestA".to_string()),
        Node::new(Name::new(&Name::new_root(), "B".to_string()), "TestB".to_string())
    ]
}

#[tauri::command]
pub fn get_edges(_: State<'_, SysDCSystemWrapper>) -> Vec<Edge> {
    vec![
        Edge::new(
            Name::new(&Name::new_root(), "A".to_string()),
            Name::new(&Name::new_root(), "B".to_string()),
            true
        )
    ]
}
