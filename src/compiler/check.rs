use super::name::Name;
use super::types::{ Type, TypeKind };
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild  };

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
        SysDCData::new(
            data.name,
            data.member
                .into_iter()
                .map(|(name, types)| match types.kind {
                    TypeKind::Int32 => (name, types),
                    _ => (name.clone(), self.def_manager.try_match_from_type(&name.get_namespace(), types))
                })
                .collect()
        )
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
        let checked_args = func.args
            .into_iter()
            .map(|(name, types)| (name.clone(), self.def_manager.try_match_from_type(&name.get_namespace(), types)))
            .collect::<Vec<(Name, Type)>>();

        let mut checked_spawns = vec!();
        for SysDCSpawn { result: (name, types), detail } in func.spawns {
            let resolved_result = (name.clone(), self.def_manager.try_match_from_type(&name.get_namespace(), types));
            let mut resolved_detail = vec!();
            for uses in detail {
                match uses {
                    SysDCSpawnChild::Use{ name, types: _ } => {
                        // let resolved_type = self.def_manager.try_match_from_type(name.get_namespace(), &defined);
                        let resolved_type = Type::new_unsovled_nohint();
                        resolved_detail.push(SysDCSpawnChild::new_use(name, resolved_type));
                    }
                }
            }
            checked_spawns.push(SysDCSpawn::new(resolved_result, resolved_detail))
        }

        let (ret_name, ret_type) = func.returns.unwrap();
        let resolved_ret_type = self.def_manager.try_match_from_type(&ret_name.clone().get_namespace(), ret_type);
        let resolved_ret = (ret_name, resolved_ret_type);

        SysDCFunction::new(func.name, checked_args, resolved_ret, checked_spawns)
    }
}

#[derive(Debug, Clone)]
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
    refs: Name
}

impl Define {
    pub fn new(kind: DefineKind, refs: Name) -> Define {
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

    pub fn try_match_from_type(&self, namespace: &String, child: Type) -> Type {
        match &child.kind {
            TypeKind::Int32 => child,
            TypeKind::Unsolved(hint) => {
                let found_def = self.find(&namespace, hint);
                match found_def.kind {
                    DefineKind::Data => Type::new(TypeKind::Data, Some(found_def.refs)),
                    _ => panic!("[ERROR] \"{:?}\" is defined but type is unmatched", child)
                }
            },
            _ => panic!("[ERROR] Called unmatch try_match function (from_type)")
        }
    }

    fn find(&self, namespace: &String, name: &String) -> Define {
        if namespace.len() == 0 {
            panic!("[ERROR] Cannot find the name \"{}\"", name);
        }

        for Define{ kind, refs } in &self.defines {
            if &refs.get_namespace() == namespace && &refs.get_local_name() == name {
                return Define::new(kind.clone(), refs.clone())
            }
        }
        let splitted_namespace = namespace.split(".").collect::<Vec<&str>>();
        let par_namespace = splitted_namespace[0..splitted_namespace.len()-1].join(".");
        self.find(&par_namespace, name)
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
                    let mut d = vec!(Define::new(DefineKind::Data, data.name.clone()));
                    d.extend(DefinesManager::listup_defines_data(data));
                    d
                })
                .collect::<Vec<Define>>()
        );
        defined.extend(
            unit.modules
                .iter()
                .flat_map(|module| {
                    let mut d = vec!(Define::new(DefineKind::Module, module.name.clone()));
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
            .map(|(name, _)| Define::new(DefineKind::DataMember, name.clone()))
            .collect::<Vec<Define>>()
    }

    fn listup_defines_module(module: &SysDCModule) -> Vec<Define> {
        module.functions
            .iter()
            .flat_map(|func| { 
                let mut d = vec!(Define::new(DefineKind::Function, func.name.clone()));
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
                .map(|(name, _)| Define::new(DefineKind::Variable, name.clone()))
                .collect::<Vec<Define>>()
        );
        defined.extend(
            func.spawns
                .iter()
                .map(|SysDCSpawn { result: (name, _), detail: _}| Define::new(DefineKind::Variable, name.clone()))
                .collect::<Vec<Define>>()
        );
        defined
    }
} 
