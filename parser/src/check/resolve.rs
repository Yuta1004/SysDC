use super::utils::define::DefinesManager;
use crate::name::Name;
use crate::structure::unchecked;
use crate::structure::{
    SysDCAnnotation, SysDCData, SysDCFunction, SysDCModule, SysDCSpawnDetail, SysDCSystem,
    SysDCUnit,
};
use crate::types::{Type, TypeKind};

pub struct TypeResolver<'a> {
    def_manager: &'a DefinesManager,
    imports: &'a Vec<Name>,
}

impl<'a> TypeResolver<'a> {
    pub fn resolve(
        system: unchecked::SysDCSystem,
        def_manager: &'a DefinesManager,
        imports: &'a Vec<Name>,
    ) -> anyhow::Result<SysDCSystem> {
        let mut resolver = TypeResolver {
            def_manager,
            imports,
        };
        system.convert(|unit| resolver.resolve_unit(unit))
    }

    fn resolve_unit(&mut self, unit: unchecked::SysDCUnit) -> anyhow::Result<SysDCUnit> {
        unit.convert(
            |data| self.resolve_data(data),
            |module| self.resolve_module(module),
        )
    }

    fn resolve_data(&self, data: unchecked::SysDCData) -> anyhow::Result<SysDCData> {
        data.convert(|(name, types): (Name, Type)| {
            if types.kind.is_primitive() {
                Ok((name, types))
            } else {
                self.def_manager
                    .resolve_from_type((name, types), self.imports)
            }
        })
    }

    fn resolve_module(&self, module: unchecked::SysDCModule) -> anyhow::Result<SysDCModule> {
        module.convert(|func| self.resolve_function(func))
    }

    fn resolve_function(&self, func: unchecked::SysDCFunction) -> anyhow::Result<SysDCFunction> {
        let a_converter = |arg| self.def_manager.resolve_from_type(arg, self.imports);
        let r_converter = |returns: (Name, Type)| {
            let returns = self.def_manager.resolve_from_type(returns, self.imports)?;
            Ok(returns)
        };
        let ann_converter = |annotation| self.resolve_annotation(annotation);
        func.convert(a_converter, r_converter, ann_converter)
    }

    fn resolve_annotation(
        &self,
        annotation: unchecked::SysDCAnnotation,
    ) -> anyhow::Result<SysDCAnnotation> {
        let a_converter = |func, args| {
            let func = self.def_manager.resolve_from_type(func, self.imports)?;
            let mut rargs = vec![];
            for (name, _) in args {
                rargs.push(self.def_manager.resolve_from_name(name, self.imports)?);
            }
            Ok((func, rargs))
        };
        let m_converter = |(name, _), uses| {
            let target = self.def_manager.resolve_from_name(name, self.imports)?;
            let mut ruses = vec![];
            for (name, _) in uses {
                ruses.push(self.def_manager.resolve_from_name(name, self.imports)?);
            }
            Ok((target, ruses))
        };
        let s_converter = |(name, _), details| {
            let result = self.def_manager.resolve_from_name(name, self.imports)?;
            let details = self.resolve_annotation_spawn_details(details)?;
            Ok((result, details))
        };
        annotation.convert(a_converter, m_converter, s_converter)
    }

    fn resolve_annotation_spawn_details(
        &self,
        details: Vec<unchecked::SysDCSpawnDetail>,
    ) -> anyhow::Result<Vec<SysDCSpawnDetail>> {
        let ur_converter = |(name, _): (Name, Type)| {
            self.def_manager
                .resolve_from_name(name.clone(), self.imports)
        };
        let l_converter = |name: Name, func: (Name, Type), args: Vec<(Name, Type)>| {
            if let Type {
                kind: TypeKind::Unsolved(_),
                ..
            } = func.1
            {
                let mut rargs = vec![];
                for (arg_name, _) in args {
                    let (arg_name, arg_type) = self
                        .def_manager
                        .resolve_from_name(arg_name.clone(), self.imports)?;
                    rargs.push((arg_name, arg_type));
                }
                let func = self.def_manager.resolve_from_type(func, self.imports)?;
                return Ok((name, func, rargs));
            }
            panic!("Internal Error")
        };

        let mut rdetails = vec![];
        for detail in details {
            rdetails.push(detail.convert(ur_converter, ur_converter, l_converter)?)
        }
        Ok(rdetails)
    }
}
