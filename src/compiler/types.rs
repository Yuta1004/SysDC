use std::fmt::{ Debug, Formatter };

use super::name::Name;
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };

#[derive(Clone, PartialEq)]
pub enum Type {
    /* Default */
    Int32,

    /* User defined */
    Solved(Name),
    Unsolved(Name),
    UnsolvedNoHint
}

impl Type {
    pub fn from(namespace: &Name, name: String) -> Type {
        match name.as_str() {
            "i32" => Type::Int32,
            _ => Type::Unsolved(Name::from(namespace, name))
        }
    }

    pub fn get_name(&self) -> Name {
        match self {
            Type::Int32 => Name::new_on_global_namespace("i32".to_string()),
            Type::Solved(name) => name.clone(),
            Type::Unsolved(name) => name.clone(),
            Type::UnsolvedNoHint => Name::new_on_global_namespace("NoHintType".to_string())
        }
    }
}

pub struct Resolver;

impl Resolver {
    pub fn resolve(system: SysDCSystem) -> SysDCSystem {
        let mut resolved_units = vec!();
        for unit in system.units {
            resolved_units.push(Resolver::resolve_unit(unit, vec!(), vec!()));
        }
        SysDCSystem::new(resolved_units)
    }

    fn resolve_unit(unit: SysDCUnit, mut defined_t: Vec<Name>, defined_f: Vec<Name>) -> SysDCUnit {
        let new_defined_t = unit.data.iter().map(|x| x.name.clone()).collect::<Vec<Name>>();
        println!("Defined new found types => {:?}", new_defined_t);
        defined_t.extend(new_defined_t);

        let mut resolved_data = vec!();
        for data in unit.data {
            resolved_data.push(Resolver::resolve_data(data, defined_t.clone()))
        }

        let mut resolved_modules = vec!();
        for module in unit.modules {
            resolved_modules.push(Resolver::resolve_module(module, defined_t.clone(), defined_f.clone()))
        }

        SysDCUnit::new(unit.name, resolved_data, resolved_modules)
    }

    fn resolve_data(data: SysDCData, defined_t: Vec<Name>) -> SysDCData {
        SysDCData::new(
            data.name,
            data.member.into_iter()
                       .map(|(n, t)| (n, Resolver::resolve_type(t, &defined_t)))
                       .collect()
        )
    }

    fn resolve_module(module: SysDCModule, defined_t: Vec<Name>, mut defined_f: Vec<Name>) -> SysDCModule {
        let new_defined_f = module.functions.iter().map(|x| x.name.clone()).collect::<Vec<Name>>();
        println!("Defined new found functions => {:?}", new_defined_f);
        defined_f.extend(new_defined_f);

        let mut resolved_functions = vec!();
        for func in module.functions {
            resolved_functions.push(Resolver::resolve_function(func, defined_t.clone(), defined_f.clone()))
        }
        SysDCModule::new(module.name, resolved_functions)
    }

    fn resolve_function(func: SysDCFunction, defined_t: Vec<Name>, defined_f: Vec<Name>) -> SysDCFunction {
        let mut resolved_args = vec!();
        for (name, types) in func.args {
            resolved_args.push((name, Resolver::resolve_type(types, &defined_t)));
        }

        let mut defined_vars = resolved_args.clone();
        for SysDCSpawn { result: (name, types), detail: _ } in &func.spawns {
            defined_vars.push((name.clone(), Resolver::resolve_type(types.clone(), &defined_t)));
        }

        let mut resolved_spanws = vec!();
        for SysDCSpawn { result: (name, types), detail } in func.spawns {
            let resolved_result = (name.clone(), Resolver::resolve_type(types, &defined_t));
            let mut resolved_detail = vec!();
            for uses in detail {
                match uses {
                    SysDCSpawnChild::Use{ name, types: _ } => {
                        let resolved_type = Resolver::resolve_var(&name, &defined_vars);
                        resolved_detail.push(SysDCSpawnChild::new_use(name, resolved_type));
                    }
                }
            }
            resolved_spanws.push(SysDCSpawn::new(resolved_result, resolved_detail))
        }

        let (ret_name, _) = func.returns.unwrap();
        let resolved_ret_type = Resolver::resolve_var(&ret_name, &defined_vars);

        SysDCFunction::new(func.name, resolved_args, (ret_name, resolved_ret_type), resolved_spanws)
    }

    fn resolve_type(types: Type, defined: &Vec<Name>) -> Type {
        match types {
            Type::UnsolvedNoHint => panic!("[ERROR] Found unsolved type which hasn't hint"),
            Type::Unsolved(hint) => {
                match defined.iter().find(|x| x.get_local_name() == hint.get_local_name()) {
                    Some(name) => Type::Solved(name.clone()),
                    None => panic!("[ERROR] Type \"{}\" is not defined", hint.get_global_name())
                }
            },
            types => types
        }
    }

    fn resolve_var(name: &Name, defined: &Vec<(Name, Type)>) -> Type {
        match defined.iter().find(|(x, _)| x.get_local_name() == name.get_local_name()) {
            Some((_, types)) => types.clone(),
            None => panic!("[ERROR] Variable \"{}\" is not defined", name.get_global_name())
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name().get_global_name())
    }
}

#[cfg(test)]
mod test {
    use super::Name;
    use super::Type;

    #[test]
    fn from_all_ok() {
        assert_eq!(Type::from(&Name::new_root(), "i32".to_string()), Type::Int32);
        assert_eq!(Type::from(&Name::new_root(), "cocoa".to_string()), Type::Unsolved(Name::from(&Name::new_root(), "cocoa".to_string())));
    }
}
