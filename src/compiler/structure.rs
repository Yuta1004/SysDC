use super::name::Name;
use super::types::SysDCType;

#[derive(Debug)]
pub struct SysDCSystem {
    pub units: Vec<SysDCUnit>
}

impl SysDCSystem {
    pub fn new(units: Vec<SysDCUnit>) -> SysDCSystem {
        SysDCSystem { units }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct SysDCData {
    pub name: Name,
    pub member: Vec<(Name, SysDCType)> 
}

impl SysDCData {
    pub fn new(name: Name, member: Vec<(Name, SysDCType)>) -> SysDCData {
        SysDCData { name, member }
    }
}

#[derive(Debug)]
pub struct SysDCModule {
    pub name: Name,
    pub functions: Vec<SysDCFunction>
}

impl SysDCModule {
    pub fn new(name: Name, functions: Vec<SysDCFunction>) -> SysDCModule {
        SysDCModule { name, functions }
    }
}

#[derive(Debug)]
pub struct SysDCFunction {
    pub name: Name,
    pub args: Vec<(Name, SysDCType)>,
    pub returns: Option<(Name, SysDCType)>,
    pub spawns: Vec<SysDCSpawn>
}

impl SysDCFunction {
    pub fn new(name: Name, args: Vec<(Name, SysDCType)>, returns: (Name, SysDCType), spawns: Vec<SysDCSpawn>) -> SysDCFunction {
        SysDCFunction { name, args, returns: Some(returns), spawns }
    }
}

#[derive(Debug)]
pub enum SysDCAnnotation {
    Return(Name),
    Spawn(SysDCSpawn)
}

impl SysDCAnnotation {
    pub fn new_return(name: Name) -> SysDCAnnotation {
        SysDCAnnotation::Return(name)
    }

    pub fn new_spawn(result: (Name, SysDCType), detail: Vec<SysDCSpawnChild>) -> SysDCAnnotation {
        SysDCAnnotation::Spawn(SysDCSpawn::new(result, detail))
    }
}

#[derive(Debug)]
pub struct SysDCSpawn {
    pub result: (Name, SysDCType),
    pub detail: Vec<SysDCSpawnChild>
}

impl SysDCSpawn {
    pub fn new(result: (Name, SysDCType), detail: Vec<SysDCSpawnChild>) -> SysDCSpawn {
        SysDCSpawn { result, detail }
    }
}

#[derive(Debug)]
pub enum SysDCSpawnChild {
    Use { name: Name, types: SysDCType },
    // Process { name: Name, func: Name, args: Vec<(Name, SysDCType)> }
}

impl SysDCSpawnChild {
    pub fn new_use(name: Name, types: SysDCType) -> SysDCSpawnChild {
        SysDCSpawnChild::Use { name, types }
    }
}

#[cfg(test)]
mod test {
    use super::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };
    use super::super::name::Name;
    use super::super::types::SysDCType;

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
        let name = Name::new(&name, "box".to_string());
        let name_data = Name::new(&name, "Box".to_string());
        let name_data_x = Name::new(&name_data, "x".to_string());
        let name_data_y = Name::new(&name_data, "y".to_string());
        let name_module = Name::new(&name, "BoxModule".to_string());
        let name_func = Name::new(&name_module, "move".to_string());
        let name_func_arg_box = Name::new(&name_func, "box".to_string());
        let name_func_arg_dx = Name::new(&name_func, "dx".to_string());
        let name_func_arg_dy = Name::new(&name_func, "dy".to_string());
        let name_func_ret = Name::new(&name_func, "movedBox".to_string());
        let name_spawn_use_box_x = Name::new(&name_func, "box.x".to_string());
        let name_spawn_use_box_y = Name::new(&name_func, "box.y".to_string());
        let name_spawn_use_dx = Name::new(&name_func, "dx".to_string());
        let name_spawn_use_dy = Name::new(&name_func, "dy".to_string());
        let name_spawn_ret = Name::new(&name_func, "movedBox".to_string());

        let spawn_use_box_x = SysDCSpawnChild::new_use(name_spawn_use_box_x, SysDCType::Int32);
        let spawn_use_box_y = SysDCSpawnChild::new_use(name_spawn_use_box_y, SysDCType::Int32);
        let spawn_use_dx = SysDCSpawnChild::new_use(name_spawn_use_dx, SysDCType::Int32);
        let spawn_use_dy = SysDCSpawnChild::new_use(name_spawn_use_dy, SysDCType::Int32);
        let spawn = SysDCSpawn::new(
            (name_spawn_ret, SysDCType::from(&name_func, "Box".to_string())),
            vec!(spawn_use_box_x, spawn_use_box_y, spawn_use_dx, spawn_use_dy)
        );

        let func_args = vec!(
            (name_func_arg_box, SysDCType::Int32),
            (name_func_arg_dx, SysDCType::Int32),
            (name_func_arg_dy, SysDCType::Int32)
        );
        let func_returns = (name_func_ret, SysDCType::from(&name_func, "Box".to_string()));
        let func = SysDCFunction::new(name_func, func_args, func_returns, vec!(spawn));

        let module = SysDCModule::new(name_module, vec!(func));

        let data_members = vec!(
            (name_data_x, SysDCType::Int32),
            (name_data_y, SysDCType::Int32)
        );
        let data = SysDCData::new(name_data, data_members);

        let unit = SysDCUnit::new(name, vec!(data), vec!(module));

        SysDCSystem::new(vec!(unit));
    }
}
