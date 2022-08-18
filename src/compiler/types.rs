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
    pub fn new(kind: TypeKind, name: Option<Name>) -> Type {
        Type {
            kind,
            refs: name
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
            _ => {
                write!(f, "{:?}:{:?}", self.kind, self.refs)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    /* プリミティブ型 */
    Int32,

    /* ユーザ定義型 */
    Data,

    /* 定義済みチェック用 (解決後のSysDCSystemには含まれない) */
    Module,
    Function,
    DataMember,
    Variable,
    
    /* パーサ用 (解決後のSysDCSystemには含まれない) */
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
            s => panic!("[ERROR] Found unknown type at deserializing => \"{}\"", s)
        })
    }
}

pub struct TypeResolver;

impl TypeResolver {
    pub fn resolve(system: SysDCSystem) -> SysDCSystem {
        SysDCSystem::new(
            system.units
                .into_iter()
                .map(|u| TypeResolver::resolve_unit(u, vec!()))
                .collect()
        )
    }

    fn resolve_unit(unit: SysDCUnit, mut defined: Vec<Type>) -> SysDCUnit {
        defined.extend(
            unit.data
                .iter()
                .map(|x| Type::new(TypeKind::Data, Some(x.name.clone())))
                .collect::<Vec<Type>>()
        );
        defined.extend(
            unit.data
                .iter()
                .flat_map(|x| &x.member)
                .map(|(n, t)| Type::new(TypeKind::DataMember, Some(n.clone())))
                .collect::<Vec<Type>>()
        );
        defined.extend(
            unit.modules
                .iter()
                .map(|x| Type::new(TypeKind::Module, Some(x.name.clone())))
                .collect::<Vec<Type>>()
        );

        SysDCUnit::new(
            unit.name,
            unit.data
                .into_iter()
                .map(|d| TypeResolver::resolve_data(d, defined.clone()))
                .collect(),
            unit.modules
                .into_iter()
                .map(|m| TypeResolver::resolve_module(m, defined.clone()))
                .collect()
        )
    }

    fn resolve_data(data: SysDCData, defined: Vec<Type>) -> SysDCData {
        SysDCData::new(
            data.name,
            data.member
                .into_iter()
                .map(|(n, t)| (n, TypeResolver::resolve_type(&t, &defined)))
                .collect()
        )
    }

    fn resolve_module(module: SysDCModule, mut defined: Vec<Type>) -> SysDCModule {
        defined.extend(
            module.functions
                .iter()
                .map(|x| Type::new(TypeKind::Function, Some(x.name.clone())))
                .collect::<Vec<Type>>()
        );

        SysDCModule::new(
            module.name,
            module.functions
                .into_iter()
                .map(|f| TypeResolver::resolve_function(f, defined.clone()))
                .collect()
        )
    }

    fn resolve_function(func: SysDCFunction, mut defined: Vec<Type>) -> SysDCFunction {
        let resolved_args = func.args
            .iter()
            .map(|(n, t)| (n.clone(), TypeResolver::resolve_type(t, &defined)))
            .collect::<Vec<(Name, Type)>>();

        defined.extend(
            resolved_args
                .iter()
                .map(|(n, _)| Type::new(TypeKind::Variable, Some(n.clone())))
                .collect::<Vec<Type>>()
        );
        defined.extend(
            func.spawns
                .iter()
                .map(|SysDCSpawn { result: (n, t), detail: _}| Type::new(TypeKind::Variable, Some(n.clone())))
                .collect::<Vec<Type>>()
        );

        let mut resolved_spanws = vec!();
        for SysDCSpawn { result: (name, types), detail } in func.spawns {
            let resolved_result = (name.clone(), TypeResolver::resolve_type(&types, &defined));
            let mut resolved_detail = vec!();
            for uses in detail {
                match uses {
                    SysDCSpawnChild::Use{ name, types: _ } => {
                        let resolved_type = TypeResolver::resolve_var(&name, &defined);
                        resolved_detail.push(SysDCSpawnChild::new_use(name, resolved_type));
                    }
                }
            }
            resolved_spanws.push(SysDCSpawn::new(resolved_result, resolved_detail))
        }

        let (ret_name, ret_type) = func.returns.unwrap();
        let resolved_ret_type = TypeResolver::resolve_type(&ret_type, &defined);

        SysDCFunction::new(func.name, resolved_args, (ret_name, resolved_ret_type), resolved_spanws)
    }

    fn resolve_type(types: &Type, defined: &Vec<Type>) -> Type {
        println!("{:?}", defined);
        Type::from("i32".to_string())
    }

    fn resolve_var(name: &Name, defined: &Vec<Type>) -> Type {
        Type::from("i32".to_string())
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
