use std::rc::Rc;
use std::collections::HashMap;

use super::types::SysDCType;

pub struct SysDCSystem {
    layers: Vec<SysDCLayer>
}

pub struct SysDCLayer {
    name: String,
    namespace: String,
    units: Vec<SysDCUnit>
}

pub struct SysDCUnit {
    name: String,
    namespace: String,
    data: Vec<Rc<SysDCData>>,
    modules: Vec<Rc<SysDCModule>>
}

pub struct SysDCData {
    name: String,
    namespace: String,
    variables: Vec<Rc<SysDCVariable>>
}

impl SysDCType for SysDCData {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_full_name(&self) -> String {
        self.namespace.clone() + &self.name
    }
}

pub struct SysDCVariable {
    name: String,
    namespace: String,
    var_type: Vec<Rc<dyn SysDCType>>
}

impl SysDCType for SysDCVariable {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_full_name(&self) -> String {
        self.namespace.clone() + &self.name
    }
}

pub struct SysDCModule {
    name: String,
    namespace: String,
    procedures: Vec<Rc<SysDCProcedure>>
}

pub struct SysDCProcedure {
    name: String,
    namespace: String,
    uses: Vec<Rc<SysDCVariable>>,
    modifies: Vec<Rc<SysDCVariable>>,
    links: Vec<Rc<SysDCLink>>
}

pub enum SysDCLinkType {
    Branch,
    Chain
}

pub struct SysDCLink {
    namespace: String,
    link_type: SysDCLinkType,
    instances: Vec<Rc<SysDCInstanceOfProcedure>>
}

pub struct SysDCInstanceOfProcedure {
    namespace: String,
    procedure: Rc<SysDCProcedure>,
    var_mapping: HashMap<String, Rc<SysDCVariable>>
}
