use crate::name::Name;
use crate::types::{ Type, TypeKind };
use crate::error::PResult;
use crate::structure::{ SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCAnnotation, SysDCSpawnDetail };
use crate::structure::unchecked;
use super::utils::define::DefinesManager;

pub struct TypeResolver<'a> {
    def_manager: &'a DefinesManager,
    imports: &'a Vec<Name>
}

impl<'a> TypeResolver<'a> {
    pub fn resolve(system: unchecked::SysDCSystem, def_manager: &'a DefinesManager, imports: &'a Vec<Name>) -> PResult<SysDCSystem> {
        let mut resolver = TypeResolver { def_manager, imports };
        system.convert(|unit| resolver.resolve_unit(unit))
    }

    fn resolve_unit(&mut self, unit: unchecked::SysDCUnit) -> PResult<SysDCUnit> {
        unit.convert(
            |data| self.resolve_data(data,),
            |module| self.resolve_module(module),
        )
    }

    fn resolve_data(&self, data: unchecked::SysDCData) -> PResult<SysDCData>{
        data.convert(|(name, types): (Name, Type)|
            if types.kind.is_primitive() {
                Ok((name, types))
            } else {
                self.def_manager.resolve_from_type((name, types), &self.imports)
            }
        )
    }

    fn resolve_module(&self, module: unchecked::SysDCModule) -> PResult<SysDCModule> {
        module.convert(|func| self.resolve_function(func))
    }

    fn resolve_function(&self, func: unchecked::SysDCFunction) -> PResult<SysDCFunction> {
        let a_converter = |arg| self.def_manager.resolve_from_type(arg, &self.imports);
        let r_converter = |returns: Option<(Name, Type)>| {
            match returns {
                Some(returns) => {
                    let returns = self.def_manager.resolve_from_type(returns, &self.imports)?;
                    Ok(Some(returns))
                },
                None => Ok(None)
            }
        };
        func.convert(a_converter, r_converter, |annotation| self.resolve_annotation(annotation))
    }

    fn resolve_annotation(&self, annotation: unchecked::SysDCAnnotation) -> PResult<SysDCAnnotation> {
        let s_converter = |(name, _): (Name, Type), details| {
            let result = self.def_manager.resolve_from_name(name, &self.imports)?;
            let details = self.resolve_annotation_spawn_details(details)?;
            Ok((result, details))
        };
        let m_converter = |(name, _), uses| {
            let target = self.def_manager.resolve_from_name(name, &self.imports)?;
            let mut ruses = vec!();
            for (name, _) in uses {
                ruses.push(self.def_manager.resolve_from_name(name, &self.imports)?);
            }
            Ok((target, vec!()))
        };
        annotation.convert(s_converter, m_converter)
    }

    fn resolve_annotation_spawn_details(&self, details: Vec<unchecked::SysDCSpawnDetail>) -> PResult<Vec<SysDCSpawnDetail>> {
        let ur_converter = |(name, _): (Name, Type)| self.def_manager.resolve_from_name(name.clone(), &self.imports);
        let l_converter = |name: Name, func: (Name, Type), args: Vec<(Name, Type)>| {
            if let Type { kind: TypeKind::Unsolved(_), .. } = func.1 {
                let mut let_to_args = vec!();
                for (arg_name, _) in args {
                    let (arg_name, arg_type) = self.def_manager.resolve_from_name(arg_name.clone(), &self.imports)?;
                    let_to_args.push((arg_name, arg_type));
                }
                let resolved_func = self.def_manager.resolve_from_type((name.clone(), func.1), &self.imports)?;
                return Ok((name, resolved_func, let_to_args));
            }
            panic!("Internal Error")
        };

        let mut rdetails = vec!();
        for detail in details {
            rdetails.push(detail.convert(ur_converter, ur_converter, l_converter)?)
        }
        Ok(rdetails)
    }
}
