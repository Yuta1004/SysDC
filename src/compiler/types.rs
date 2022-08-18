use std::fmt::{ Debug, Formatter };

use serde::ser::{ Serialize, Serializer };
use serde::de::{ Deserialize, Deserializer };

use super::name::Name;
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };

#[derive(Clone, PartialEq)]
pub enum Type {
    /* Data */
    Int32,
    UserDefined(Name),

    /* for TypeResolver */
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
            Type::UserDefined(name) => name.clone(),
            Type::Unsolved(name) => name.clone(),
            Type::UnsolvedNoHint => Name::new_on_global_namespace("NoHintType".to_string())
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Type::Unsolved(_) |
            Type::UnsolvedNoHint => panic!("[ERROR] Cannot serialize object containing unsolved types."),
            t => t.get_name().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Type, D::Error>
    where
        D: Deserializer<'de> 
    {
        let name = Name::deserialize(deserializer)?;
        Ok(match name.get_local_name().as_str() {
            "i32" => Type::Int32,
            _ => {
                let namespace = Name::from(&Name::new_root(), name.get_global_name().replace(".0.", ""));
                Type::UserDefined(Name::from(&namespace, name.get_local_name()))
            }
        })
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name().get_global_name())
    }
}

pub struct Resolver;

impl Resolver {
    pub fn resolve(system: SysDCSystem) -> SysDCSystem {
        SysDCSystem::new(
            system.units
                .into_iter()
                .map(|u| Resolver::resolve_unit(u, vec!(), vec!()))
                .collect()
        )
    }

    fn resolve_unit(unit: SysDCUnit, mut defined_t: Vec<Name>, defined_f: Vec<Name>) -> SysDCUnit {
        defined_t.extend(
            unit.data
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<Name>>()
        );

        SysDCUnit::new(
            unit.name,
            unit.data
                .into_iter()
                .map(|d| Resolver::resolve_data(d, defined_t.clone()))
                .collect(),
            unit.modules
                .into_iter()
                .map(|m| Resolver::resolve_module(m, defined_t.clone(), defined_f.clone()))
                .collect()
        )
    }

    fn resolve_data(data: SysDCData, defined_t: Vec<Name>) -> SysDCData {
        SysDCData::new(
            data.name,
            data.member
                .into_iter()
                .map(|(n, t)| (n, Resolver::resolve_type(t, &defined_t)))
                .collect()
        )
    }

    fn resolve_module(module: SysDCModule, defined_t: Vec<Name>, mut defined_f: Vec<Name>) -> SysDCModule {
        defined_f.extend(
            module.functions
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<Name>>()
        );

        SysDCModule::new(
            module.name,
            module.functions
                .into_iter()
                .map(|f| Resolver::resolve_function(f, defined_t.clone(), defined_f.clone()))
                .collect()
        )
    }

    fn resolve_function(func: SysDCFunction, defined_t: Vec<Name>, _: Vec<Name>) -> SysDCFunction {
        let resolved_args = func.args
            .into_iter()
            .map(|(n, t)| (n, Resolver::resolve_type(t.clone(), &defined_t)))
            .collect::<Vec<(Name, Type)>>();

        let mut defined_vars = resolved_args.clone();
        defined_vars.extend(
            func.spawns
                .iter()
                .map(|SysDCSpawn { result: (n, t), detail: _}| (n.clone(), Resolver::resolve_type(t.clone(), &defined_t)))
                .collect::<Vec<(Name, Type)>>()
        );

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
                    Some(name) => Type::UserDefined(name.clone()),
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
