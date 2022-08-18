use std::fmt::{ Debug, Formatter };

use serde::{ Serialize, Deserialize };
use serde::ser::Serializer;
use serde::de::Deserializer;

use super::name::Name;
use super::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCSpawn, SysDCSpawnChild };

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub refs: Option<Name>
}

impl Type {
    pub fn new(kind: TypeKind, name: Name) -> Type {
        Type {
            kind,
            refs: Some(name)
        }
    }

    pub fn new_unsovled_nohint() -> Type {
        Type {
            kind: TypeKind::UnsolvedNoHint,
            refs: None
        }
    }

    pub fn from(name: String) -> Type {
        match name.as_str() {
            "i32" => Type { kind: TypeKind::Int32, refs: None },
            _ => Type { kind: TypeKind::Unsolved(name), refs: None }
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            TypeKind::Int32 => write!(f, "Int32"),
            TypeKind::Unsolved(hint) => write!(f, "{}", hint),
            TypeKind::UnsolvedNoHint => write!(f, "UnsolvedNoHint"),
            _ => write!(f, "{}", self.refs.as_ref().unwrap().get_global_name())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    /* Primitive */
    Int32,

    /* UserDefined */
    Data,
    DataMember,
    Module,
    Function,

    /* for TypeResolver */
    Unsolved(String),
    UnsolvedNoHint
}

impl Serialize for TypeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            TypeKind::Unsolved(_) |
            TypeKind::UnsolvedNoHint => panic!("[ERROR] Cannot serialize object containing unsolved types."),
            _ => serializer.serialize_str(&format!("{:?}", self))
        }
    }
}

impl<'de> Deserialize<'de> for TypeKind {
    fn deserialize<D>(deserializer: D) -> Result<TypeKind, D::Error>
    where
        D: Deserializer<'de> 
    {
        let kind = String::deserialize(deserializer)?;
        Ok(match kind.as_str() {
            "Int32" => TypeKind::Int32,
            "Data" => TypeKind::Data,
            "DataMember" => TypeKind::DataMember,
            "Module" => TypeKind::Module,
            "Function" => TypeKind::Function,
            s => panic!("[ERROR] Found unknown type at deserializing => \"{}\"", s)
        })
    }
}

pub struct Resolver;

impl Resolver {
    pub fn resolve(system: SysDCSystem) -> SysDCSystem {
        SysDCSystem::new(
            system.units
                .into_iter()
                .map(|u| Resolver::resolve_unit(u, vec!()))
                .collect()
        )
    }

    fn resolve_unit(unit: SysDCUnit, defined: Vec<Type>) -> SysDCUnit {
        // defined.extend(
        //     unit.data
        //         .iter()
        //         .map(|x| x.name.clone())
        //         .collect::<Vec<Name>>()
        // );

        SysDCUnit::new(
            unit.name,
            unit.data
                .into_iter()
                .map(|d| Resolver::resolve_data(d, defined.clone()))
                .collect(),
            unit.modules
                .into_iter()
                .map(|m| Resolver::resolve_module(m, defined.clone()))
                .collect()
        )
    }

    fn resolve_data(data: SysDCData, defined: Vec<Type>) -> SysDCData {
        SysDCData::new(
            data.name,
            data.member
                .into_iter()
                .map(|(n, t)| (n, Resolver::resolve_type(&t, &defined)))
                .collect()
        )
    }

    fn resolve_module(module: SysDCModule, defined: Vec<Type>) -> SysDCModule {
        // defined.extend(
        //     module.functions
        //         .iter()
        //         .map(|x| x.name.clone())
        //         .collect::<Vec<Name>>()
        // );

        SysDCModule::new(
            module.name,
            module.functions
                .into_iter()
                .map(|f| Resolver::resolve_function(f, defined.clone()))
                .collect()
        )
    }

    fn resolve_function(func: SysDCFunction, defined: Vec<Type>) -> SysDCFunction {
        let resolved_args = func.args
            .into_iter()
            .map(|(n, t)| (n, Resolver::resolve_type(&t, &defined)))
            .collect::<Vec<(Name, Type)>>();

        let mut defined_vars = resolved_args.clone();
        defined_vars.extend(
            func.spawns
                .iter()
                .map(|SysDCSpawn { result: (n, t), detail: _}| (n.clone(), Resolver::resolve_type(t, &defined)))
                .collect::<Vec<(Name, Type)>>()
        );

        let mut resolved_spanws = vec!();
        for SysDCSpawn { result: (name, types), detail } in func.spawns {
            let resolved_result = (name.clone(), Resolver::resolve_type(&types, &defined));
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

    fn resolve_type(types: &Type, defined: &Vec<Type>) -> Type {
        // match types {
        //     Type::UnsolvedNoHint => panic!("[ERROR] Found unsolved type which hasn't hint"),
        //     Type::Unsolved(hint) => {
        //         match defined.iter().find(|x| x.get_local_name() == hint.get_local_name()) {
        //             Some(name) => Type::UserDefined(name.clone()),
        //             None => panic!("[ERROR] Type \"{}\" is not defined", hint.get_global_name())
        //         }
        //     },
        //     types => types
        // }
        Type::from("i32".to_string())
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
    use super::{ Type, TypeKind };

    #[test]
    fn from_all_ok() {
        assert_eq!(Type::from("i32".to_string()), Type { kind: TypeKind::Int32, refs: None });
        assert_eq!(Type::from("cocoa".to_string()), Type { kind: TypeKind::Unsolved("cocoa".to_string()), refs: None });
    }
}
