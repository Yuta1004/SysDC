use super::name::Name;
use super::types::{ Type, TypeKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn };

pub struct Checker {
    def_manager: DefinesManager
}

impl Checker {
    pub fn check(system: SysDCSystem) -> SysDCSystem {
        let checker = Checker { def_manager: DefinesManager::new(&system) };
        SysDCSystem::new(
            system.units
                .into_iter()
                .map(|unit| checker.check_unit(unit))
                .collect()
        )
    }

    fn check_unit(&self, unit: SysDCUnit) -> SysDCUnit {
        SysDCUnit::new(
            unit.name,
            unit.data
                .into_iter()
                .map(|data| self.check_data(data))
                .collect(),
            unit.modules
                .into_iter()
                .map(|module| self.check_module(module))
                .collect()
        )
    }

    fn check_data(&self, data: SysDCData) -> SysDCData {
        data
    }

    fn check_module(&self, module: SysDCModule) -> SysDCModule {
        SysDCModule::new(
            module.name,
            module.functions
                .into_iter()
                .map(|func| self.check_function(func))
                .collect()
        )
    }

    fn check_function(&self, func: SysDCFunction) -> SysDCFunction {
        func
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

struct DefinesManager {
    defines: Vec<Define>
}

impl DefinesManager {
    pub fn new(system: &SysDCSystem) -> DefinesManager {
        DefinesManager { defines: DefinesManager::listup_defines(system) }
    }

    fn listup_defines(system: &SysDCSystem) -> Vec<Define> {
        system.units
            .iter()
            .flat_map(|unit| DefinesManager::listup_defines_unit(unit))
            .collect()
    }

    fn listup_defines_unit(unit: &SysDCUnit) -> Vec<Define> {
        let mut defined = vec!();
        defined.extend(
            unit.data
                .iter()
                .flat_map(|data| {
                    let mut d = vec!(Define::new(DefineKind::Data, Some(data.name.clone())));
                    d.extend(DefinesManager::listup_defines_data(data));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined.extend(
            unit.modules
                .iter()
                .flat_map(|module| {
                    let mut d = vec!(Define::new(DefineKind::Module, Some(module.name.clone())));
                    d.extend(DefinesManager::listup_defines_module(module));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined
    }

    fn listup_defines_data(data: &SysDCData) -> Vec<Define> {
        data.member
            .iter()
            .map(|(name, _)| Define::new(DefineKind::DataMember, Some(name.clone())))
            .collect::<Vec<Define>>()
    }

    fn listup_defines_module(module: &SysDCModule) -> Vec<Define> {
        module.functions
            .iter()
            .flat_map(|func| { 
                let mut d = vec!(Define::new(DefineKind::Function, Some(func.name.clone())));
                d.extend(DefinesManager::listup_defines_function(func));
                d
            })
            .collect::<Vec<Define>>()
    }

    fn listup_defines_function(func: &SysDCFunction) -> Vec<Define> {
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
