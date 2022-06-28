use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::name::Name;
use super::types::SysDCType;

#[derive(Debug)]
pub struct SysDCSystem {
    pub name: Name,
    pub layers: Vec<SysDCLayer>
}

impl SysDCSystem {
    pub fn new() -> SysDCSystem {
        SysDCSystem {
            name: Name::new_root(),
            layers: vec!()
        }
    }

    pub fn push_layer(&mut self, layer: SysDCLayer) {
        self.layers.push(layer);
    }
}

#[derive(Debug)]
pub struct SysDCLayer {
    pub name: Name,
    pub units: Vec<SysDCUnit>
}

impl SysDCLayer {
    pub fn new(namespace: &Name, layer_num: i32) -> SysDCLayer {
        SysDCLayer {
            name: Name::new(namespace, &format!("layer{}", layer_num)),
            units: vec!()
        }
    }

    pub fn push_units(&mut self, unit: SysDCUnit) {
        self.units.push(unit);
    }
}

#[derive(Debug)]
pub struct SysDCUnit {
    pub name: Name,
    pub data: Vec<Rc<RefCell<SysDCData>>>,
    pub modules: Vec<Rc<RefCell<SysDCModule>>>
}

impl SysDCUnit {
    pub fn new(namespace: &Name, name: &String) -> SysDCUnit {
        SysDCUnit {
            name: Name::new(namespace, name),
            data: vec!(),
            modules: vec!()
        }
    }

    pub fn push_data(&mut self, data: Rc<RefCell<SysDCData>>) {
        self.data.push(data);
    }

    pub fn push_module(&mut self, module: Rc<RefCell<SysDCModule>>) {
        self.modules.push(module);
    }
}

#[derive(Debug)]
pub struct SysDCData {
    pub name: Name,
    pub variables: Vec<Rc<RefCell<SysDCVariable>>>
}

impl SysDCData {
    pub fn new(namespace: &Name, name: &String) -> Rc<RefCell<SysDCData>> {
        Rc::new(
            RefCell::new(
                SysDCData {
                    name: Name::new(namespace, name),
                    variables: vec!()
                }
            )
        )
    }

    pub fn push_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
        self.variables.push(variable);
    }
}

#[derive(Debug)]
pub struct SysDCVariable {
    pub name: Name,
    pub var_type: SysDCType
}

impl SysDCVariable {
    pub fn new(namespace: &Name, name: &String, var_type: SysDCType) -> Rc<RefCell<SysDCVariable>> {
        Rc::new(
            RefCell::new(
                SysDCVariable {
                    name: Name::new(namespace, name),
                    var_type
                }
            )
        )
    }
}

#[derive(Debug)]
pub struct SysDCModule {
    pub name: Name,
    pub procedures: Vec<Rc<RefCell<SysDCProcedure>>>
}

impl SysDCModule {
    pub fn new(namespace: &Name, name: &String) -> Rc<RefCell<SysDCModule>> {
        Rc::new(
            RefCell::new(
                SysDCModule {
                    name: Name::new(namespace, name),
                    procedures: vec!()
                }
            )
        )
    }

    pub fn push_procedure(&mut self, procedure: Rc<RefCell<SysDCProcedure>>) {
        self.procedures.push(procedure);
    }
}

#[derive(Debug)]
pub struct SysDCProcedure {
    pub name: Name,
    pub return_type: SysDCType,
    pub args: Vec<Rc<RefCell<SysDCVariable>>>,
    pub uses: Vec<Rc<RefCell<SysDCVariable>>>,
    pub modifies: Vec<Rc<RefCell<SysDCVariable>>>,
    pub link: Option<Rc<RefCell<SysDCLink>>>
}

impl SysDCProcedure {
    pub fn new(namespace: &Name, name: &String) -> Rc<RefCell<SysDCProcedure>> {
        Rc::new(
            RefCell::new(
                SysDCProcedure {
                    name: Name::new(namespace, name),
                    return_type: SysDCType::NoneType,
                    args: vec!(),
                    uses: vec!(),
                    modifies: vec!(),
                    link: None
                }
            )
        )
    }

    pub fn set_return_type(&mut self, return_type: SysDCType) {
        self.return_type = return_type;
    }

    pub fn push_arg(&mut self, arg: Rc<RefCell<SysDCVariable>>) {
        self.args.push(arg);
    }

    pub fn push_using_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
        self.uses.push(variable)
    }

    pub fn push_modifying_variable(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
        self.modifies.push(variable)
    }

    pub fn set_link(&mut self, link: Rc<RefCell<SysDCLink>>) {
        match self.link {
            Some(_) => panic!("[ERROR] SysDCProcedure.link is already setted"),
            None => self.link = Some(link)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SysDCLinkType {
    Branch,
    Chain,
    InstanceOfProcedure
}

#[derive(Debug)]
pub struct SysDCLink {
    pub name: Name,
    pub link_type: SysDCLinkType,
    pub links: Option<Vec<Rc<RefCell<SysDCLink>>>>,         // Valid for link_type is Branch/Chain
    pub procedure: Option<Rc<RefCell<SysDCProcedure>>>,     // Valid for link_type is InstanceOfProcedure
    pub args: Option<Vec<Rc<RefCell<SysDCVariable>>>>       // Valid for link_type is InstanceOfProcedure
}

impl SysDCLink {
    pub fn new_branch(namespace: &Name) -> Rc<RefCell<SysDCLink>> {
        Rc::new(
            RefCell::new(
                SysDCLink {
                    name: Name::new(namespace, &"".to_string()),
                    link_type: SysDCLinkType::Branch,
                    links: Some(vec!()),
                    procedure: None,
                    args: None
                }
            )
        )
    }

    pub fn new_chain(namespace: &Name) -> Rc<RefCell<SysDCLink>> {
        Rc::new(
            RefCell::new(
                SysDCLink {
                    name: Name::new(namespace, &"".to_string()),
                    link_type: SysDCLinkType::Chain,
                    links: Some(vec!()),
                    procedure: None,
                    args: None
                }
            )
        )
    }

    pub fn new_instance_of_procedure(namespace: &Name) -> Rc<RefCell<SysDCLink>> {
        Rc::new(
            RefCell::new(
                SysDCLink {
                    name: Name::new(namespace, &"".to_string()),
                    link_type: SysDCLinkType::InstanceOfProcedure,
                    links: None,
                    procedure: None,
                    args: Some(vec!())
                }
            )
        )
    }

    pub fn push_link(&mut self, link: Rc<RefCell<SysDCLink>>) {
        if self.link_type != SysDCLinkType::InstanceOfProcedure {
            self.links.as_mut().unwrap().push(link);
        } else {
            panic!("[ERROR] SysDCLink.link_type is InstanceOfProcedure, but push_link called")
        }
    }

    pub fn set_procedure(&mut self, procedure: Rc<RefCell<SysDCProcedure>>) {
        if self.link_type == SysDCLinkType::InstanceOfProcedure {
            match self.procedure {
                Some(_) => panic!("[ERROR] SysDCLink.procedure is already setted"),
                None => self.procedure = Some(procedure)
            }
        } else {
            panic!("[ERROR] SysDCLink.link_type is Branch/Chain, but set_procedure called")
        }
    }

    pub fn push_arg(&mut self, variable: Rc<RefCell<SysDCVariable>>) {
        if self.link_type == SysDCLinkType::InstanceOfProcedure {
            self.args.as_mut().unwrap().push(variable);
        } else {
            panic!("[ERROR] SysDCLink.link_type is Branch/Chain, but push_arg_mapping called")
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::super::types::SysDCType;
    use super::{ SysDCSystem, SysDCLayer, SysDCUnit, SysDCData, SysDCVariable, SysDCModule, SysDCProcedure };

    #[test]
    fn create_sysdc_tree() {
        /* [file: user.def]
            data User {
                id: int32,
                age: int32,
                name: string
            }

            module UserModule binds User as this {
                greet() -> none {
                    use = this.age;
                    link = chain {
                        Printer::print(text: this.id),
                        Printer::print(text: this.name)
                    };
                }
            }
        */
        /* [file: printer.def]
            module Printer {
                print(text: string) -> none {
                    use text; 
                }
            }
        */

        let mut system = SysDCSystem::new();
        {
            let mut layer_1 = SysDCLayer::new(&system.name, 1);
            {
                let mut printer_unit = SysDCUnit::new(&layer_1.name, &"printer".to_string());
                {
                    let printer_module = SysDCModule::new(&printer_unit.name, &"Printer".to_string());
                    {
                        let print_procedure = SysDCProcedure::new(&printer_module.borrow().name, &"print".to_string());
                        let print_procedure_text = SysDCVariable::new(&print_procedure.borrow().name, &"text".to_string(), SysDCType::Int32);
                        print_procedure.borrow_mut().push_arg(Rc::clone(&print_procedure_text));
                        print_procedure.borrow_mut().push_using_variable(Rc::clone(&print_procedure_text));
                        printer_module.borrow_mut().push_procedure(print_procedure);
                    }
                    printer_unit.push_module(printer_module);
                }
                layer_1.push_units(printer_unit);
            }
            system.push_layer(layer_1);

            let mut layer_0 = SysDCLayer::new(&system.name, 0);
            {
                let mut user_unit = SysDCUnit::new(&layer_0.name, &"user".to_string());
                {
                    let user_data = SysDCData::new(&user_unit.name, &"User".to_string());
                    let user_data_name = &user_data.borrow().name.clone();
                    user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, &"id".to_string(), SysDCType::Int32));
                    user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, &"age".to_string(), SysDCType::Int32));
                    user_data.borrow_mut().push_variable(SysDCVariable::new(user_data_name, &"name".to_string(), SysDCType::StringType));
                    user_unit.push_data(user_data);

                    let user_module = SysDCModule::new(&user_unit.name, &"UserModule".to_string());
                    {
                        let greet_procedure = SysDCProcedure::new(&user_module.borrow().name, &"greet".to_string());
                        // New Job(Connector): use = this.age;
                        // New Job(Connector): link = chain { Printer::print(text: this.id), Printer::print(text: this.name) }
                        user_module.borrow_mut().push_procedure(greet_procedure);
                    }
                    user_unit.push_module(user_module);
                }
                layer_0.push_units(user_unit);
            }
            system.push_layer(layer_0);
        }
    }
}
