use serde::Serialize;

use super::name::Name;
use super::types::Type;

#[derive(Debug, Serialize)]
pub struct SysDCSystem {
    pub units: Vec<SysDCUnit>
}

impl SysDCSystem {
    pub fn new(units: Vec<SysDCUnit>) -> SysDCSystem {
        SysDCSystem { units }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct SysDCData {
    pub name: Name,
    pub member: Vec<(Name, Type)> 
}

impl SysDCData {
    pub fn new(name: Name, member: Vec<(Name, Type)>) -> SysDCData {
        SysDCData { name, member }
    }
}

#[derive(Debug, Serialize)]
pub struct SysDCModule {
    pub name: Name,
    pub functions: Vec<SysDCFunction>
}

impl SysDCModule {
    pub fn new(name: Name, functions: Vec<SysDCFunction>) -> SysDCModule {
        SysDCModule { name, functions }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct SysDCSpawn {
    pub result: (Name, Type),
    pub detail: Vec<SysDCSpawnChild>
}

impl SysDCSpawn {
    pub fn new(result: (Name, Type), detail: Vec<SysDCSpawnChild>) -> SysDCSpawn {
        SysDCSpawn { result, detail }
    }
}

#[derive(Debug, Serialize)]
pub enum SysDCSpawnChild {
    Use { name: Name, types: Type },
    // Process { name: Name, func: Name, args: Vec<(Name, Type)> }
}

impl SysDCSpawnChild {
    pub fn new_use(name: Name, types: Type) -> SysDCSpawnChild {
        SysDCSpawnChild::Use { name, types }
    }
}

#[cfg(test)]
mod test {
    use super::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };
    use super::super::name::Name;
    use super::super::types::Type;

    #[test]
    fn create_system() {
        //  box.def
        //  -------
        //  data Box {
        //      x: i32,
        //      y: i32
        //  }
        // 
        //  module BoxModule {
        //      move(box: Box, dx: i32, dy: i32) -> Box {
        //          @return movedBox
        // 
        //          +use box.x, box.y, dx, dy
        //          @spawn movedBox: Box
        //      }
        //  }

        let name = Name::new_root();
        let name = Name::from(&name, "box".to_string());
        let name_data = Name::from(&name, "Box".to_string());
        let name_data_x = Name::from(&name_data, "x".to_string());
        let name_data_y = Name::from(&name_data, "y".to_string());
        let name_module = Name::from(&name, "BoxModule".to_string());
        let name_func = Name::from(&name_module, "move".to_string());
        let name_func_arg_box = Name::from(&name_func, "box".to_string());
        let name_func_arg_dx = Name::from(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::from(&name_func, "dy".to_string());
        let name_func_ret = Name::from(&name_func, "movedBox".to_string());
        let name_spawn_use_box_x = Name::from(&name_func, "box.x".to_string());
        let name_spawn_use_box_y = Name::from(&name_func, "box.y".to_string());
        let name_spawn_use_dx = Name::from(&name_func, "dx".to_string());
        let name_spawn_use_dy = Name::from(&name_func, "dy".to_string());
        let name_spawn_ret = Name::from(&name_func, "movedBox".to_string());

        let spawn_use_box_x = SysDCSpawnChild::new_use(name_spawn_use_box_x, Type::Int32);
        let spawn_use_box_y = SysDCSpawnChild::new_use(name_spawn_use_box_y, Type::Int32);
        let spawn_use_dx = SysDCSpawnChild::new_use(name_spawn_use_dx, Type::Int32);
        let spawn_use_dy = SysDCSpawnChild::new_use(name_spawn_use_dy, Type::Int32);
        let spawn = SysDCSpawn::new(
            (name_spawn_ret, Type::from(&name_func, "Box".to_string())),
            vec!(spawn_use_box_x, spawn_use_box_y, spawn_use_dx, spawn_use_dy)
        );

        let func_args = vec!(
            (name_func_arg_box, Type::Int32),
            (name_func_arg_dx, Type::Int32),
            (name_func_arg_dy, Type::Int32)
        );
        let func_returns = (name_func_ret, Type::from(&name_func, "Box".to_string()));
        let func = SysDCFunction::new(name_func, func_args, func_returns, vec!(spawn));

        let module = SysDCModule::new(name_module, vec!(func));

        let data_members = vec!(
            (name_data_x, Type::Int32),
            (name_data_y, Type::Int32)
        );
        let data = SysDCData::new(name_data, data_members);

        let unit = SysDCUnit::new(name, vec!(data), vec!(module));

        SysDCSystem::new(vec!(unit));
    }
}
