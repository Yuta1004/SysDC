use super::utils::define::DefinesManager;
use crate::error::{PError, PErrorKind};
use crate::name::Name;
use crate::structure::{SysDCAnnotation, SysDCFunction, SysDCSpawnDetail, SysDCSystem};
use crate::types::{Type, TypeKind};

pub struct TypeMatchChecker<'a> {
    def_manager: &'a DefinesManager,
    imports: &'a Vec<Name>,
}

impl<'a> TypeMatchChecker<'a> {
    pub fn check(
        system: &SysDCSystem,
        def_manager: &'a DefinesManager,
        imports: &'a Vec<Name>,
    ) -> anyhow::Result<()> {
        let checker = TypeMatchChecker {
            def_manager,
            imports,
        };
        for unit in &system.units {
            for module in &unit.modules {
                for func in &module.functions {
                    checker.check_function(func)?;
                }
            }
        }
        Ok(())
    }

    fn check_function(&self, func: &SysDCFunction) -> anyhow::Result<()> {
        if func.returns.1.kind != TypeKind::Void {
            let req_ret_type = &func.returns.1;
            let act_ret_type = self
                .def_manager
                .resolve_from_name(func.returns.0.clone(), self.imports)?
                .1;
            if req_ret_type != &act_ret_type {
                return Err(PError::from(PErrorKind::TypeUnmatch2(
                    req_ret_type.clone(),
                    act_ret_type,
                ))
                .into());
            }
        }

        for annotation in &func.annotations {
            match annotation {
                SysDCAnnotation::Affect { func, args } => {
                    self.check_annotation_affect(func, args)?
                }
                SysDCAnnotation::Spawn { result, details } => {
                    self.check_annotation_spawn(result, details)?
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn check_annotation_affect(
        &self,
        (func, _): &(Name, Type),
        args: &Vec<(Name, Type)>,
    ) -> anyhow::Result<()> {
        let act_arg_types = args;
        let req_arg_types = self.def_manager.get_args_type(func, self.imports)?;
        if act_arg_types.len() != req_arg_types.len() {
            return Err(PError::from(PErrorKind::ArgumentsLengthNotMatch).into());
        }
        for ((_, act_arg_type), req_arg_type) in act_arg_types.iter().zip(req_arg_types.iter()) {
            if act_arg_type != req_arg_type {
                return Err(PError::from(PErrorKind::TypeUnmatch2(
                    req_arg_type.clone(),
                    act_arg_type.clone(),
                ))
                .into());
            }
        }
        Ok(())
    }

    fn check_annotation_spawn(
        &self,
        result: &(Name, Type),
        details: &Vec<SysDCSpawnDetail>,
    ) -> anyhow::Result<()> {
        for detail in details {
            match detail {
                SysDCSpawnDetail::Return(_, act_ret_type) => {
                    if &result.1 != act_ret_type {
                        return Err(PError::from(PErrorKind::TypeUnmatch2(
                            result.1.clone(),
                            act_ret_type.clone(),
                        ))
                        .into());
                    }
                }
                SysDCSpawnDetail::LetTo {
                    func: (func, _),
                    args,
                    ..
                } => {
                    let act_arg_types = args;
                    let req_arg_types = self.def_manager.get_args_type(func, self.imports)?;
                    if act_arg_types.len() != req_arg_types.len() {
                        return Err(PError::from(PErrorKind::ArgumentsLengthNotMatch).into());
                    }
                    for ((_, act_arg_type), req_arg_type) in
                        act_arg_types.iter().zip(req_arg_types.iter())
                    {
                        if act_arg_type != req_arg_type {
                            return Err(PError::from(PErrorKind::TypeUnmatch2(
                                req_arg_type.clone(),
                                act_arg_type.clone(),
                            ))
                            .into());
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
