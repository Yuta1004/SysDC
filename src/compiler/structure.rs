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
    pub spawns: Vec<SysDCSpawn>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysDCSpawn {
    pub result: (Name, Type),
    pub details: Vec<SysDCSpawnChild>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SysDCSpawnChild {
    Use(Name, Type),
    Return(Name, Type),
    LetTo { name: Name, func: (Name, Type), args: Vec<(Name, Type)> },
}

pub mod unchecked {
    use std::error::Error;

    use super::Name;
    use super::Type;

    #[derive(Debug)]
    pub struct SysDCSystem {
        pub units: Vec<SysDCUnit>
    }

    impl SysDCSystem {
        pub fn new(units: Vec<SysDCUnit>) -> SysDCSystem {
            SysDCSystem { units }
        }

        pub fn convert<F>(self, converter: F) -> Result<super::SysDCSystem, Box<dyn Error>>
        where
            F: Fn(SysDCUnit) -> Result<super::SysDCUnit, Box<dyn Error>>
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
        pub modules: Vec<SysDCModule>
    }

    impl SysDCUnit {
        pub fn new(name: Name, data: Vec<SysDCData>, modules: Vec<SysDCModule>) -> SysDCUnit {
            SysDCUnit { name, data, modules }
        }

        pub fn convert<F, G>(self, d_converter: F, m_converter: G) -> Result<super::SysDCUnit, Box<dyn Error>>
        where
            F: Fn(SysDCData) -> Result<super::SysDCData, Box<dyn Error>>,
            G: Fn(SysDCModule) -> Result<super::SysDCModule, Box<dyn Error>>
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

        pub fn convert<F>(self, converter: F) -> Result<super::SysDCData, Box<dyn Error>>
        where
            F: Fn((Name, Type)) -> Result<(Name, Type), Box<dyn Error>>
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

        pub fn convert<F>(self, converter: F) -> Result<super::SysDCModule, Box<dyn Error>>
        where
            F: Fn(SysDCFunction) -> Result<super::SysDCFunction, Box<dyn Error>> 
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
        pub spawns: Vec<SysDCSpawn>
    }

    impl SysDCFunction {
        pub fn new(name: Name, args: Vec<(Name, Type)>, returns: (Name, Type), spawns: Vec<SysDCSpawn>) -> SysDCFunction {
            SysDCFunction { name, args, returns: Some(returns), spawns }
        }

        pub fn convert<F, G, H>(self, a_convert: F, r_convert: G, s_convert: H) -> Result<super::SysDCFunction, Box<dyn Error>>
        where
            F: Fn((Name, Type)) -> Result<(Name, Type), Box<dyn Error>>,
            G: Fn(Option<(Name, Type)>) -> Result<Option<(Name, Type)>, Box<dyn Error>>,
            H: Fn(SysDCSpawn) -> Result<super::SysDCSpawn, Box<dyn Error>>
        {
            let (returns, mut args, mut spawns) = (r_convert(self.returns)?, vec!(), vec!());
            for arg in self.args {
                args.push(a_convert(arg)?);
            }
            for spawn in self.spawns {
                spawns.push(s_convert(spawn)?);
            }
            Ok(super::SysDCFunction { name: self.name, args, returns, spawns })
        }
    }

    #[derive(Debug, Clone)]
    pub struct SysDCSpawn {
        pub result: (Name, Type),
        pub details: Vec<SysDCSpawnChild>
    }

    impl SysDCSpawn {
        pub fn new(result: (Name, Type), details: Vec<SysDCSpawnChild>) -> SysDCSpawn {
            SysDCSpawn { result, details }
        }

        pub fn convert<F, G>(self, r_converter: F, d_converter: G) -> Result<super::SysDCSpawn, Box<dyn Error>>
        where
            F: Fn((Name, Type)) -> Result<(Name, Type), Box<dyn Error>>,
            G: Fn(SysDCSpawnChild) -> Result<super::SysDCSpawnChild, Box<dyn Error>> 
        {
            let (result, mut details) = (r_converter(self.result)?, vec!());
            for detail in self.details {
                details.push(d_converter(detail)?);
            }
            Ok(super::SysDCSpawn{ result, details })
        }
    }

    #[derive(Debug, Clone)]
    pub enum SysDCSpawnChild {
        Use(Name, Type),
        Return(Name, Type),
        LetTo { name: Name, func: (Name, Type), args: Vec<(Name, Type)> },
    }

    impl SysDCSpawnChild {
        pub fn new_use(name: Name, types: Type) -> SysDCSpawnChild {
            SysDCSpawnChild::Use(name, types)
        }

        pub fn new_return(name: Name, types: Type) -> SysDCSpawnChild {
            SysDCSpawnChild::Return(name, types)
        }

        pub fn new_let_to(name: Name, func: (Name, Type), args: Vec<(Name, Type)>) -> SysDCSpawnChild {
            SysDCSpawnChild::LetTo { name, func, args }
        }

        pub fn convert<F, G>(self, u_converter: F, r_converter: F, l_converter: G) -> Result<super::SysDCSpawnChild, Box<dyn Error>>
        where
            F: Fn((Name, Type)) -> Result<(Name, Type), Box<dyn Error>>,
            G: Fn(Name, (Name, Type), Vec<(Name, Type)>) -> Result<(Name, (Name, Type), Vec<(Name, Type)>), Box<dyn Error>>, 
        {
            match self {
                SysDCSpawnChild::Use(name, types) => {
                    let (name, types) = u_converter((name, types))?;
                    Ok(super::SysDCSpawnChild::Use(name, types))
                },
                SysDCSpawnChild::Return(name, types) => {
                    let (name, types) = r_converter((name, types))?;
                    Ok(super::SysDCSpawnChild::Return(name, types))
                },
                SysDCSpawnChild::LetTo { name, func, args } => {
                    let (name, func, args) = l_converter(name, func, args)?;
                    Ok(super::SysDCSpawnChild::LetTo { name, func, args })
                }
            }
        }
    }
}
