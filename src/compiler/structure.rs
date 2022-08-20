use serde::{ Serialize, Deserialize };

use super::name::Name;
use super::types::Type;

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCSystem {
    pub units: Vec<SysDCUnit>
}

impl SysDCSystem {
    pub fn new(units: Vec<SysDCUnit>) -> SysDCSystem {
        SysDCSystem { units }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCUnit {
    pub name: Name,
    pub data: Vec<SysDCData>,
    pub modules: Vec<SysDCModule>
}

impl SysDCUnit {
    pub fn new(name: Name, data: Vec<SysDCData>, modules: Vec<SysDCModule>) -> SysDCUnit {
        SysDCUnit { name, data, modules }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCData {
    pub name: Name,
    pub member: Vec<(Name, Type)> 
}

impl SysDCData {
    pub fn new(name: Name, member: Vec<(Name, Type)>) -> SysDCData {
        SysDCData { name, member }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCModule {
    pub name: Name,
    pub functions: Vec<SysDCFunction>
}

impl SysDCModule {
    pub fn new(name: Name, functions: Vec<SysDCFunction>) -> SysDCModule {
        SysDCModule { name, functions }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCFunction {
    pub name: Name,
    pub args: Vec<(Name, Type)>,
    pub returns: Option<(Name, Type)>,
    pub spawns: Vec<SysDCSpawn>
}

impl SysDCFunction {
    pub fn new(name: Name, args: Vec<(Name, Type)>, returns: (Name, Type), spawns: Vec<SysDCSpawn>) -> SysDCFunction {
        SysDCFunction { name, args, returns: Some(returns), spawns }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SysDCAnnotation {
    Return(Name),
    Spawn(SysDCSpawn)
}

impl SysDCAnnotation {
    pub fn new_return(name: Name) -> SysDCAnnotation {
        SysDCAnnotation::Return(name)
    }

    pub fn new_spawn(result: (Name, Type), detail: Vec<SysDCSpawnChild>) -> SysDCAnnotation {
        SysDCAnnotation::Spawn(SysDCSpawn::new(result, detail))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCSpawn {
    pub result: (Name, Type),
    pub detail: Vec<SysDCSpawnChild>
}

impl SysDCSpawn {
    pub fn new(result: (Name, Type), detail: Vec<SysDCSpawnChild>) -> SysDCSpawn {
        SysDCSpawn { result, detail }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SysDCSpawnChild {
    Use(Name, Type),
    Return(Name, Type),
    LetTo { name: Name, func: (Name, Type), args: Vec<(Name, Type)> },
}

impl SysDCSpawnChild {
    pub fn new_use(name: Name, types: Type) -> SysDCSpawnChild {
        SysDCSpawnChild::Use(name, types)
    }

    pub fn new_return(name: Name, types: Type) -> SysDCSpawnChild {
        SysDCSpawnChild::Return(name, types)
    }

    pub fn new_let_to(name: Name, func: (Name, Type), args: Vec<(Name, Type)>) -> SysDCSpawnChild {
        SysDCSpawnChild::LetTo { name, func, args }
    }
}
