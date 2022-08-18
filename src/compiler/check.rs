use super::name::Name;
use super::types::{ Type, TypeKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn };

pub struct Checker;

impl Checker {
    pub fn check(system: SysDCSystem) -> SysDCSystem {
        system
    }
}

#[derive(Debug)]
enum DefineKind {
    Data,
    DataMember,
    Module,
    Function,
    Variable
}

#[derive(Debug)]
struct Define {
    kind: DefineKind,
    refs: Option<Name>
}

impl Define {
    pub fn new(kind: DefineKind, refs: Option<Name>) -> Define {
        Define { kind, refs }
    }
}

struct DefinesManager<'a> {
    system: &'a SysDCSystem,
    defines: Vec<Define>
}

impl<'a> DefinesManager<'a> {
    pub fn new(system: &'a SysDCSystem) -> DefinesManager {
        let mut manager = DefinesManager {
            system,
            defines: vec!()
        };
        manager.defines = manager.listup_defines();
        manager
    }

    fn listup_defines(&self) -> Vec<Define> {
        self.system.units
            .iter()
            .flat_map(|unit| self.listup_defines_unit(unit))
            .collect()
    }

    fn listup_defines_unit(&self, unit: &SysDCUnit) -> Vec<Define> {
        let mut defined = vec!();
        defined.extend(
            unit.data
                .iter()
                .flat_map(|data| {
                    let mut d = vec!(Define::new(DefineKind::Data, Some(data.name.clone())));
                    d.extend(self.listup_defines_data(data));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined.extend(
            unit.modules
                .iter()
                .flat_map(|module| {
                    let mut d = vec!(Define::new(DefineKind::Module, Some(module.name.clone())));
                    d.extend(self.listup_defines_module(module));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined
    }

    fn listup_defines_data(&self, data: &SysDCData) -> Vec<Define> {
        data.member
            .iter()
            .map(|(name, _)| Define::new(DefineKind::DataMember, Some(name.clone())))
            .collect::<Vec<Define>>()
    }

    fn listup_defines_module(&self, module: &SysDCModule) -> Vec<Define> {
        module.functions
            .iter()
            .flat_map(|func| { 
                let mut d = vec!(Define::new(DefineKind::Function, Some(func.name.clone())));
                d.extend(self.listup_defines_function(func));
                d
            })
            .collect::<Vec<Define>>()
    }

    fn listup_defines_function(&self, func: &SysDCFunction) -> Vec<Define> {
        let mut defined = vec!();
        defined.extend(
            func.args
                .iter()
                .map(|(name, _)| Define::new(DefineKind::Variable, Some(name.clone())))
                .collect::<Vec<Define>>()
        );
        defined.extend(
            func.spawns
                .iter()
                .map(|SysDCSpawn { result: (name, _), detail: _}| Define::new(DefineKind::Variable, Some(name.clone())))
                .collect::<Vec<Define>>()
        );
        defined
    }
} 
