use serde::{ Serialize, Deserialize };

use super::name::Name;
use super::types::Type;

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCSystem {
    pub units: Vec<SysDCUnit>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCUnit {
    pub name: Name,
    pub data: Vec<SysDCData>,
    pub modules: Vec<SysDCModule>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCData {
    pub name: Name,
    pub members: Vec<(Name, Type)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCModule {
    pub name: Name,
    pub functions: Vec<SysDCFunction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCFunction {
    pub name: Name,
    pub args: Vec<(Name, Type)>,
    pub returns: Option<(Name, Type)>,
    pub annotations: Vec<SysDCAnnotation>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SysDCAnnotation {
    Spawn { result: (Name, Type), details: Vec<SysDCSpawnDetail> }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SysDCSpawnDetail {
    Use(Name, Type),
    Return(Name, Type),
    LetTo { name: Name, func: (Name, Type), args: Vec<(Name, Type)> },
}

pub mod unchecked {
    use super::Name;
    use super::Type;
    use super::super::error::PResult;

    #[derive(Debug)]
    pub struct SysDCSystem {
        pub units: Vec<SysDCUnit>
    }

    impl SysDCSystem {
        pub fn new(units: Vec<SysDCUnit> ) -> SysDCSystem {
            SysDCSystem { units }
        }

        pub fn convert<F>(self, mut converter: F) -> PResult<super::SysDCSystem>
        where
            F: FnMut(SysDCUnit) -> PResult<super::SysDCUnit>
        {
            let mut units = vec!();
            for unit in self.units {
                units.push(converter(unit)?);
            }
            Ok(super::SysDCSystem { units })
        }
    }

    #[derive(Debug)]
    pub struct SysDCUnit {
        pub name: Name,
        pub data: Vec<SysDCData>,
        pub modules: Vec<SysDCModule>,
        pub imports: Vec<Name>
    }

    impl SysDCUnit {
        pub fn new(name: Name, data: Vec<SysDCData>, modules: Vec<SysDCModule>, imports: Vec<Name>) -> SysDCUnit {
            SysDCUnit { name, data, modules, imports }
        }

        pub fn convert<F, G>(self, d_converter: F, m_converter: G) -> PResult<super::SysDCUnit>
        where
            F: Fn(SysDCData) -> PResult<super::SysDCData>,
            G: Fn(SysDCModule) -> PResult<super::SysDCModule>
        {
            let (mut data, mut modules) = (vec!(), vec!());
            for _data in self.data {
                data.push(d_converter(_data)?);
            }
            for module in self.modules {
                modules.push(m_converter(module)?);
            }
            Ok(super::SysDCUnit { name: self.name, data, modules })
        }
    }

    #[derive(Debug)]
    pub struct SysDCData {
        pub name: Name,
        pub members: Vec<(Name, Type)>
    }

    impl SysDCData {
        pub fn new(name: Name, members: Vec<(Name, Type)>) -> SysDCData {
            SysDCData { name, members }
        }

        pub fn convert<F>(self, converter: F) -> PResult<super::SysDCData>
        where
            F: Fn((Name, Type)) -> PResult<(Name, Type)>
        {
            let mut members = vec!();
            for member in self.members {
                members.push(converter(member)?);
            }
            Ok(super::SysDCData { name: self.name, members })
        }
    }

    #[derive(Debug)]
    pub struct SysDCModule {
        pub name: Name,
        pub functions: Vec<SysDCFunction>
    }

    impl SysDCModule {
        pub fn new(name: Name, functions: Vec<SysDCFunction>) -> SysDCModule {
            SysDCModule { name, functions }
        }

        pub fn convert<F>(self, converter: F) -> PResult<super::SysDCModule>
        where
            F: Fn(SysDCFunction) -> PResult<super::SysDCFunction>
        {
            let mut functions = vec!();
            for func in self.functions {
                functions.push(converter(func)?);
            }
            Ok(super::SysDCModule { name: self.name, functions })
        }
    }

    #[derive(Debug)]
    pub struct SysDCFunction {
        pub name: Name,
        pub args: Vec<(Name, Type)>,
        pub returns: Option<(Name, Type)>,
        pub annotations: Vec<SysDCAnnotation>
    }

    impl SysDCFunction {
        pub fn new(name: Name, args: Vec<(Name, Type)>, returns: Option<(Name, Type)>, annotations: Vec<SysDCAnnotation>) -> SysDCFunction {
            SysDCFunction { name, args, returns, annotations }
        }

        pub fn convert<F, G, H>(self, a_convert: F, r_convert: G, s_convert: H) -> PResult<super::SysDCFunction>
        where
            F: Fn((Name, Type)) -> PResult<(Name, Type)>,
            G: Fn(Option<(Name, Type)>) -> PResult<Option<(Name, Type)>>,
            H: Fn(SysDCAnnotation) -> PResult<super::SysDCAnnotation>
        {
            let (returns, mut args, mut annotations) = (r_convert(self.returns)?, vec!(), vec!());
            for arg in self.args {
                args.push(a_convert(arg)?);
            }
            for annotation in self.annotations {
                annotations.push(s_convert(annotation)?);
            }
            Ok(super::SysDCFunction { name: self.name, args, returns, annotations })
        }
    }

    #[derive(Debug, Clone)]
    pub enum SysDCAnnotation {
        Return(Name),
        Spawn { result: (Name, Type), details: Vec<SysDCSpawnDetail> },
        Modify { target: (Name, Type), uses: Vec<(Name, Type)>}
    }

    impl SysDCAnnotation {
        pub fn new_return(name: Name) -> SysDCAnnotation {
            SysDCAnnotation::Return(name)
        }

        pub fn new_spawn(result: (Name, Type), details: Vec<SysDCSpawnDetail>) -> SysDCAnnotation {
            SysDCAnnotation::Spawn { result, details }
        }

        pub fn new_modify(target: (Name, Type), uses: Vec<(Name, Type)>) -> SysDCAnnotation {
            SysDCAnnotation::Modify { target, uses }
        }

        pub fn convert<F>(self, s_converter: F) -> PResult<super::SysDCAnnotation>
        where
            F: Fn((Name, Type), Vec<SysDCSpawnDetail>) -> PResult<((Name, Type), Vec<super::SysDCSpawnDetail>)>
        {
            match self {
                SysDCAnnotation::Spawn { result, details } => {
                    let (result, details) = s_converter(result, details)?;
                    Ok(super::SysDCAnnotation::Spawn { result, details })
                }
                _ => panic!("Internal error")
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum SysDCSpawnDetail {
        Use(Name, Type),
        Return(Name, Type),
        LetTo { name: Name, func: (Name, Type), args: Vec<(Name, Type)> },
    }

    impl SysDCSpawnDetail {
        pub fn new_use(name: Name, types: Type) -> SysDCSpawnDetail {
            SysDCSpawnDetail::Use(name, types)
        }

        pub fn new_return(name: Name, types: Type) -> SysDCSpawnDetail {
            SysDCSpawnDetail::Return(name, types)
        }

        pub fn new_let_to(name: Name, func: (Name, Type), args: Vec<(Name, Type)>) -> SysDCSpawnDetail {
            SysDCSpawnDetail::LetTo { name, func, args }
        }

        pub fn convert<F, G>(self, u_converter: F, r_converter: F, l_converter: G) -> PResult<super::SysDCSpawnDetail>
        where
            F: Fn((Name, Type)) -> PResult<(Name, Type)>,
            G: Fn(Name, (Name, Type), Vec<(Name, Type)>) -> PResult<(Name, (Name, Type), Vec<(Name, Type)>)>,
        {
            match self {
                SysDCSpawnDetail::Use(name, types) => {
                    let (name, types) = u_converter((name, types))?;
                    Ok(super::SysDCSpawnDetail::Use(name, types))
                },
                SysDCSpawnDetail::Return(name, types) => {
                    let (name, types) = r_converter((name, types))?;
                    Ok(super::SysDCSpawnDetail::Return(name, types))
                },
                SysDCSpawnDetail::LetTo { name, func, args } => {
                    let (name, func, args) = l_converter(name, func, args)?;
                    Ok(super::SysDCSpawnDetail::LetTo { name, func, args })
                }
            }
        }
    }
}
