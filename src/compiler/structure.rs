use std::rc::Rc;

pub struct SysDCSystem {
    layers: Vec<Rc<SysDCLayer>>
}

pub struct SysDCLayer {
    layer: i32,
    units: Vec<Rc<SysDCLayer>>
}

pub struct SysDCUnit {
    name: String,
    data: Vec<Rc<SysDCData>>,
    modules: Vec<Rc<SysDCModule>>
}

pub struct SysDCData {
    name: String,
    variables: Vec<Rc<SysDCVariable>>
}

pub struct SysDCVariable {
    name: String,
    _type: String   // mock
}

pub struct SysDCModule {
    name: String,
    procedures: Vec<Rc<SysDCProcedure>>
}

pub struct SysDCProcedure {
}
