use super::name::Name;
use super::types::SysDCType;

#[derive(Debug)]
pub struct SysDCSystemWrapper {
    pub system: SysDCSystem,
}

impl SysDCSystemWrapper {
    pub fn create(system: SysDCSystem) {
        panic!("[ERROR] Cannot check the connections inside given SysDCSystem.")
    }
}

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

// #[derive(Debug)]
// pub struct SysDCSystem {
//     pub name: Name,
//     pub layers: Vec<SysDCLayer>
// }

// impl SysDCSystem {
//     pub fn new() -> SysDCSystem {
//         SysDCSystem {
//             name: Name::new_root(),
//             layers: vec!()
//         }
//     }

//     pub fn push_layer(&mut self, layer: SysDCLayer) {
//         self.layers.push(layer);
//     }
// }

// #[derive(Debug)]
// pub struct SysDCLayer {
//     pub name: Name,
//     pub units: Vec<SysDCUnit>
// }

// impl SysDCLayer {
//     pub fn new(namespace: &Name, layer_num: i32) -> SysDCLayer {
//         SysDCLayer {
//             name: Name::new(namespace, format!("layer{}", layer_num)),
//             units: vec!()
//         }
//     }

//     pub fn push_units(&mut self, unit: SysDCUnit) {
//         self.units.push(unit);
//     }
// }

// #[derive(Debug)]
// pub struct SysDCUnit {
//     pub name: Name,
//     pub data: Vec<Rc<RefCell<SysDCData>>>,
//     pub modules: Vec<Rc<RefCell<SysDCModule>>>
// }

// impl SysDCUnit {
//     pub fn new(namespace: &Name, name: String) -> SysDCUnit {
//         SysDCUnit {
//             name: Name::new(namespace, name),
//             data: vec!(),
//             modules: vec!()
//         }
//     }

//     pub fn push_data(&mut self, data: Rc<RefCell<SysDCData>>) {
//         self.data.push(data);
//     }

//     pub fn push_module(&mut self, module: Rc<RefCell<SysDCModule>>) {
//         self.modules.push(module);
//     }
// }

// #[derive(Debug)]
// pub struct SysDCData {
//     pub name: Name,
//     pub variables: Vec<Rc<RefCell<SysDCVariable>>>
// }

// impl SysDCData {
//     pub fn new(namespace: &Name, name: String) -> Rc<RefCell<SysDCData>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCData {
//                     name: Name::new(namespace, name),
//                     variables: vec!()
//                 }
//             )
//         )
//     }

//     pub fn push_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
//         self.variables.push(variable);
//     }
// }

// #[derive(Debug)]
// pub struct SysDCVariable {
//     pub name: Name,
//     pub var_type: SysDCType
// }

// impl SysDCVariable {
//     pub fn new(namespace: &Name, name: String, var_type: SysDCType) -> Rc<RefCell<SysDCVariable>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCVariable {
//                     name: Name::new(namespace, name),
//                     var_type
//                 }
//             )
//         )
//     }
// }

// #[derive(Debug)]
// pub struct SysDCModule {
//     pub name: Name,
//     pub functions: Vec<Rc<RefCell<SysDCFunction>>>
// }

// impl SysDCModule {
//     pub fn new(namespace: &Name, name: String) -> Rc<RefCell<SysDCModule>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCModule {
//                     name: Name::new(namespace, name),
//                     functions: vec!()
//                 }
//             )
//         )
//     }

//     pub fn push_function(&mut self, function: Rc<RefCell<SysDCFunction>>) {
//         self.functions.push(function);
//     }
// }

// #[derive(Debug)]
// pub struct SysDCFunction {
//     pub name: Name,
//     pub return_type: SysDCType,
//     pub args: Vec<Rc<RefCell<SysDCVariable>>>,
//     pub uses: Vec<Rc<RefCell<SysDCVariable>>>,
//     pub modifies: Vec<Rc<RefCell<SysDCVariable>>>,
//     pub link: Option<Rc<RefCell<SysDCLink>>>
// }

// impl SysDCFunction {
//     pub fn new(namespace: &Name, name: String) -> Rc<RefCell<SysDCFunction>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCFunction {
//                     name: Name::new(namespace, name),
//                     return_type: SysDCType::NoneType,
//                     args: vec!(),
//                     uses: vec!(),
//                     modifies: vec!(),
//                     link: None
//                 }
//             )
//         )
//     }

//     pub fn set_return_type(&mut self, return_type: SysDCType) {
//         self.return_type = return_type;
//     }

//     pub fn push_arg(&mut self, arg: Rc<RefCell<SysDCVariable>>) {
//         self.args.push(arg);
//     }

//     pub fn push_using_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
//         self.uses.push(variable)
//     }

//     pub fn push_modifying_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
//         self.modifies.push(variable)
//     }

//     pub fn set_link(&mut self, link: Rc<RefCell<SysDCLink>>) {
//         match self.link {
//             Some(_) => panic!("[ERROR] SysDCFunction.link is already setted"),
//             None => self.link = Some(link)
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum SysDCLinkType {
//     Branch,
//     Chain,
//     InstanceOfFunction
// }

// #[derive(Debug)]
// pub struct SysDCLink {
//     pub name: Name,
//     pub link_type: SysDCLinkType,
//     pub links: Option<Vec<Rc<RefCell<SysDCLink>>>>,         // Valid for link_type is Branch/Chain
//     pub func: Option<Rc<RefCell<SysDCFunction>>>,     // Valid for link_type is InstanceOfFunction
//     pub args: Option<Vec<Rc<RefCell<SysDCVariable>>>>       // Valid for link_type is InstanceOfFunction
// }

// impl SysDCLink {
//     pub fn new_branch(namespace: &Name, name: String) -> Rc<RefCell<SysDCLink>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCLink {
//                     name: Name::new(namespace, name),
//                     link_type: SysDCLinkType::Branch,
//                     links: Some(vec!()),
//                     func: None,
//                     args: None
//                 }
//             )
//         )
//     }

//     pub fn new_chain(namespace: &Name, name: String) -> Rc<RefCell<SysDCLink>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCLink {
//                     name: Name::new(namespace, name),
//                     link_type: SysDCLinkType::Chain,
//                     links: Some(vec!()),
//                     func: None,
//                     args: None
//                 }
//             )
//         )
//     }

//     pub fn new_instance_of_function(namespace: &Name, name: String) -> Rc<RefCell<SysDCLink>> {
//         Rc::new(
//             RefCell::new(
//                 SysDCLink {
//                     name: Name::new(namespace, name),
//                     link_type: SysDCLinkType::InstanceOfFunction,
//                     links: None,
//                     func: None,
//                     args: Some(vec!())
//                 }
//             )
//         )
//     }

//     pub fn push_link(&mut self, link: Rc<RefCell<SysDCLink>>) {
//         if self.link_type != SysDCLinkType::InstanceOfFunction {
//             self.links.as_mut().unwrap().push(link);
//         } else {
//             panic!("[ERROR] SysDCLink.link_type is InstanceOfFunction, but push_link called")
//         }
//     }

//     pub fn set_function(&mut self, function: Rc<RefCell<SysDCFunction>>) {
//         if self.link_type == SysDCLinkType::InstanceOfFunction {
//             match self.func {
//                 Some(_) => panic!("[ERROR] SysDCLink.function is already setted"),
//                 None => self.func = Some(function)
//             }
//         } else {
//             panic!("[ERROR] SysDCLink.link_type is Branch/Chain, but set_function called")
//         }
//     }

//     pub fn push_arg(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
//         if self.link_type == SysDCLinkType::InstanceOfFunction {
//             self.args.as_mut().unwrap().push(variable);
//         } else {
//             panic!("[ERROR] SysDCLink.link_type is Branch/Chain, but push_arg_mapping called")
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::rc::Rc;

//     use super::super::types::SysDCType;
//     use super::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable, SysDCModule, SysDCFunction };

//     #[test]
//     fn create_sysdc_tree() {
//         /* [file: user.def]
//             data User {
//                 id: i32,
//                 age: i32,
//                 name: i32
//             }

//             module UserModule binds User as this {
//                 greet() -> none {
//                     use = this.age;
//                     link = chain {
//                         Printer::print(text: this.id),
//                         Printer::print(text: this.name)
//                     };
//                 }
//             }
//         */
//         /* [file: printer.def]
//             module Printer {
//                 print(text: i32) -> none {
//                     use text; 
//                 }
//             }
//         */

//         let mut system = SysDCSystem::new();
//         {
//             let mut layer_1 = SysDCLayer::new(&system.name, 1);
//             {
//                 let mut printer_unit = SysDCUnit::new(&layer_1.name, "printer".to_string());
//                 {
//                     let printer_module = SysDCModule::new(&printer_unit.name, "Printer".to_string());
//                     {
//                         let print_function = SysDCFunction::new(&printer_module.borrow().name, "print".to_string());
//                         let print_function_text = SysDCVariable::new(&print_function.borrow().name, "text".to_string(), SysDCType::Int32);
//                         print_function.borrow_mut().push_arg(Rc::clone(&print_function_text));
//                         print_function.borrow_mut().push_using_variable(Rc::clone(&print_function_text));
//                         printer_module.borrow_mut().push_function(print_function);
//                     }
//                     printer_unit.push_module(printer_module);
//                 }
//                 layer_1.push_units(printer_unit);
//             }
//             system.push_layer(layer_1);

//             let mut layer_0 = SysDCLayer::new(&system.name, 0);
//             {
//                 let mut user_unit = SysDCUnit::new(&layer_0.name, "user".to_string());
//                 {
//                     let user_data = SysDCData::new(&user_unit.name, "User".to_string());
//                     let user_data_name = &user_data.borrow().name.clone();
//                     user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, "id".to_string(), SysDCType::Int32));
//                     user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, "age".to_string(), SysDCType::Int32));
//                     user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, "name".to_string(), SysDCType::Int32));
//                     user_unit.push_data(user_data);

//                     let user_module = SysDCModule::new(&user_unit.name, "UserModule".to_string());
//                     {
//                         let greet_function = SysDCFunction::new(&user_module.borrow().name, "greet".to_string());
//                         // New Job(Connector): use = this.age;
//                         // New Job(Connector): link = chain { Printer::print(text: this.id), Printer::print(text: this.name) }
//                         user_module.borrow_mut().push_function(greet_function);
//                     }
//                     user_unit.push_module(user_module);
//                 }
//                 layer_0.push_units(user_unit);
//             }
//             system.push_layer(layer_0);
//         }
//     }
// }
