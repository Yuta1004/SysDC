use tauri::State;

use sysdc_parser::name::Name;

use super::SysDCSystemWrapper;
use super::react_flow::{Node, Edge};

#[tauri::command]
pub fn get_flow(_: State<'_, SysDCSystemWrapper>) -> (Vec<Node>, Vec<Edge>) {
    let nodes = vec![
        Node::new(Name::new(&Name::new_root(), "A".to_string()), "TestA".to_string()),
        Node::new(Name::new(&Name::new_root(), "B".to_string()), "TestB".to_string())
    ];
    let edges = vec![
        Edge::new(
            Name::new(&Name::new_root(), "A".to_string()),
            Name::new(&Name::new_root(), "B".to_string()),
            true
        )
    ];
    (nodes, edges)
}
