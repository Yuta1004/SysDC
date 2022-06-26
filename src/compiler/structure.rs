use std::rc::Rc;
use std::collections::HashMap;

use super::name::Name;
use super::types::SysDCType;

pub struct SysDCSystem {
    name: Name,
    layers: Vec<SysDCLayer>
}

pub struct SysDCLayer {
    name: Name,
    units: Vec<SysDCUnit>
}

pub struct SysDCUnit {
    name: Name,
    data: Vec<Rc<SysDCData>>,
    modules: Vec<Rc<SysDCModule>>
}

pub struct SysDCData {
    name: Name,
    variables: Vec<Rc<SysDCVariable>>
}

impl SysDCType for SysDCData {
    fn get_name(&self) -> String {
        self.name.get_name()
    }

    fn get_full_name(&self) -> String {
        self.name.get_full_name()
    }
}

pub struct SysDCVariable {
    name: Name,
    var_type: Vec<Rc<dyn SysDCType>>
}

impl SysDCType for SysDCVariable {
    fn get_name(&self) -> String {
        self.name.get_name()
    }

    fn get_full_name(&self) -> String {
        self.name.get_full_name()
    }
}

pub struct SysDCModule {
    name: Name,
    procedures: Vec<Rc<SysDCProcedure>>
}

pub struct SysDCProcedure {
    name: Name,
    uses: Vec<Rc<SysDCVariable>>,
    modifies: Vec<Rc<SysDCVariable>>,
    links: Vec<Rc<SysDCLink>>
}

pub enum SysDCLinkType {
    Branch,
    Chain
}

pub struct SysDCLink {
    name: Name,
    link_type: SysDCLinkType,
    instances: Vec<Rc<SysDCInstanceOfProcedure>>
}

pub struct SysDCInstanceOfProcedure {
    name: Name,
    procedure: Rc<SysDCProcedure>,
    var_mapping: HashMap<String, Rc<SysDCVariable>>
}
