use crate::error::{PError, PErrorKind};
use crate::name::Name;
use crate::structure::unchecked;
use crate::types::{Type, TypeKind};

#[derive(Debug, Clone, PartialEq, Eq)]
enum DefineKind {
    Data,
    DataMember(Type),
    Module,
    Function(Type),
    Argument(Type),
    Variable(Type),
    Use(Name),
}

#[derive(Debug)]
struct Define {
    kind: DefineKind,
    refs: Name,
}

impl Define {
    pub fn new(kind: DefineKind, refs: Name) -> Define {
        Define { kind, refs }
    }
}

pub struct DefinesManager {
    defines: Vec<Define>,
}

impl DefinesManager {
    pub fn new(system: &unchecked::SysDCSystem) -> anyhow::Result<DefinesManager> {
        let mut def_manager = DefinesManager { defines: vec![] };
        def_manager.listup_defines(system)?;
        Ok(def_manager)
    }

    // 与えられたnameと同じ名前を持つ定義が存在するかどうかを確認する
    pub fn check_can_import(&self, name: &Name, imports: &Vec<Name>) -> anyhow::Result<()> {
        match self.find(name.clone(), &name.name, imports)?.kind {
            DefineKind::Data | DefineKind::Module => Ok(()),
            _ => Err(PError::from(PErrorKind::NotDefined(name.name.clone())).into()),
        }
    }

    // 与えられたnameから参照可能なすべての範囲またはimports内を対象に，typesと一致する定義を探す (Data, Module, Function)
    // ※name, typesはともに関連している状態を想定
    pub fn resolve_from_type(
        &self,
        (name, types): (Name, Type),
        imports: &Vec<Name>,
    ) -> anyhow::Result<(Name, Type)> {
        if types.kind.is_primitive() || types.kind == TypeKind::Data {
            return Ok((name, types));
        }

        if let TypeKind::Unsolved(hint) = &types.kind {
            let (head, tails) = split_name(hint);
            let found_def = self.find(name.clone(), &head, imports)?;
            return match found_def.kind {
                DefineKind::Data => match tails {
                    Some(_) => Err(PError::from(PErrorKind::IllegalAccess).into()),
                    None => Ok((name, Type::new(TypeKind::Data, Some(found_def.refs)))),
                },
                DefineKind::Module => match tails {
                    Some(tails) => self.get_func_in_module(&found_def.refs, &tails, imports),
                    None => Err(PError::from(PErrorKind::MissingFunctionName).into()),
                },
                DefineKind::Function(_) => {
                    self.get_func_in_module(&name.get_namespace(true), hint, imports)
                }
                _ => Err(PError::from(PErrorKind::TypeUnmatch1(types)).into()),
            };
        }

        panic!("Internal Error");
    }

    // nameから参照可能なすべての範囲またはimports内を対象に，nameと一致する名前をもつ定義を探す (Variable)
    pub fn resolve_from_name(
        &self,
        name: Name,
        imports: &Vec<Name>,
    ) -> anyhow::Result<(Name, Type)> {
        let (head, tails) = split_name(&name.name);
        let found_def = self.find(name.clone(), &head, &vec![])?;
        match found_def.kind {
            DefineKind::Variable(types) => {
                let (_, types) = self.resolve_from_type((name.clone(), types), imports)?;
                match types.kind {
                    TypeKind::Data => match tails {
                        Some(tails) => {
                            let (_, types) = self.get_member_in_data(
                                types.refs.as_ref().unwrap(),
                                &tails,
                                imports,
                            )?;
                            Ok((name, types))
                        }
                        None => Ok((found_def.refs, types)),
                    },
                    _ => Ok((found_def.refs, types)),
                }
            }
            DefineKind::Use(use_ref) => match tails {
                Some(_) => {
                    let (dname, _) = self.resolve_from_name(use_ref, imports)?;
                    self.resolve_from_name(
                        Name::new(&dname.get_par_name(false), name.name),
                        imports,
                    )
                }
                None => self.resolve_from_name(use_ref, imports),
            },
            _ => Err(PError::from(PErrorKind::NotDefined(name.name)).into()),
        }
    }

    // 与えられた関数名に対応する関数を探し，関数に登録されている引数の型の一覧を返す
    pub fn get_args_type(
        &self,
        func_name: &Name,
        imports: &Vec<Name>,
    ) -> anyhow::Result<Vec<Type>> {
        let func_name = func_name.get_full_name();
        let mut args = vec![];
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Argument(types) = kind {
                if refs.namespace == func_name {
                    args.push(
                        self.resolve_from_type((refs.clone(), types.clone()), imports)?
                            .1,
                    );
                }
            }
        }
        Ok(args)
    }

    // data(Data)内のmember(Member)の定義を探す
    fn get_member_in_data(
        &self,
        data: &Name,
        member: &str,
        imports: &Vec<Name>,
    ) -> anyhow::Result<(Name, Type)> {
        let (head, tails) = split_name(member);
        for Define { kind, refs } in &self.defines {
            if let DefineKind::DataMember(types) = kind {
                if data.get_full_name() == refs.namespace && head == refs.name {
                    let (_, types) =
                        self.resolve_from_type((refs.clone(), types.clone()), imports)?;
                    if types.kind.is_primitive() {
                        return match tails {
                            Some(_) => Err(PError::from(PErrorKind::IllegalAccess).into()),
                            None => Ok((refs.clone(), types)),
                        };
                    }
                    if types.kind == TypeKind::Data {
                        return match tails {
                            Some(tails) => self.get_member_in_data(
                                types.refs.as_ref().unwrap(),
                                &tails,
                                imports,
                            ),
                            None => Ok((types.refs.clone().unwrap(), types)),
                        };
                    }
                    panic!("Internal Error");
                }
            }
        }
        Err(PError::from(PErrorKind::MemberNotDefinedInData(
            member.to_string(),
            data.name.clone(),
        ))
        .into())
    }

    // module(Module)内のfunc(Function)の定義を探す
    fn get_func_in_module(
        &self,
        module: &Name,
        func: &String,
        imports: &Vec<Name>,
    ) -> anyhow::Result<(Name, Type)> {
        for Define { kind, refs } in &self.defines {
            if let DefineKind::Function(types) = kind {
                if module == &refs.get_par_name(true) && func == &refs.name {
                    return Ok((
                        refs.clone(),
                        self.resolve_from_type((refs.clone(), types.clone()), imports)?
                            .1,
                    ));
                }
            }
        }
        Err(PError::from(PErrorKind::FuncNotDefinedInModule(
            func.clone(),
            module.name.clone(),
        ))
        .into())
    }

    // namespace内に存在する定義を対象に，nameと同じ名前を持つ定義を探して返す
    // namespace内に存在しない場合はimports内の名前を探して返す
    // ※namespaceはルートにたどり着くまで再帰的に更新されながら検索が続く (.a.b.c -> .a.b -> .a -> .)
    fn find(
        &self,
        mut namespace: Name,
        name: &String,
        imports: &Vec<Name>,
    ) -> anyhow::Result<Define> {
        let had_underscore = namespace.has_underscore();
        while !namespace.name.is_empty() {
            for Define { kind, refs } in &self.defines {
                if refs.namespace == namespace.namespace && &refs.name == name {
                    if let DefineKind::Variable(_) = kind {
                        if had_underscore && !refs.has_underscore() {
                            continue;
                        }
                    }
                    return Ok(Define::new(kind.clone(), refs.clone()));
                }
            }
            namespace = namespace.get_par_name(false);
        }

        for import in imports {
            if &import.name == name {
                return self.find(import.clone(), &import.name, &vec![]);
            }
        }

        Err(PError::from(PErrorKind::NotFound(name.clone())).into())
    }

    /* ----- ↓前処理用↓ ----- */

    fn define(&mut self, def: Define) -> anyhow::Result<()> {
        match &self.find(def.refs.clone(), &def.refs.name, &vec![]) {
            Ok(Define { kind, .. }) => match (kind, &def.kind) {
                (DefineKind::Argument(_), _) => {}
                (_, DefineKind::Argument(_)) => {}
                _ => return Err(PError::from(PErrorKind::AlreadyDefined(def.refs.name)).into()),
            },
            Err(_) => {}
        }
        self.defines.push(def);
        Ok(())
    }

    fn listup_defines(&mut self, system: &unchecked::SysDCSystem) -> anyhow::Result<()> {
        for unit in &system.units {
            self.listup_defines_unit(unit)?;
        }
        Ok(())
    }

    fn listup_defines_unit(&mut self, unit: &unchecked::SysDCUnit) -> anyhow::Result<()> {
        for data in &unit.data {
            self.define(Define::new(DefineKind::Data, data.name.clone()))?;
            self.listup_defines_data(data)?;
        }
        for module in &unit.modules {
            self.define(Define::new(DefineKind::Module, module.name.clone()))?;
            self.listup_defines_module(module)?;
        }
        Ok(())
    }

    fn listup_defines_data(&mut self, data: &unchecked::SysDCData) -> anyhow::Result<()> {
        for (name, types) in &data.members {
            self.define(Define::new(
                DefineKind::DataMember(types.clone()),
                name.clone(),
            ))?;
        }
        Ok(())
    }

    fn listup_defines_module(&mut self, module: &unchecked::SysDCModule) -> anyhow::Result<()> {
        for func in &module.functions {
            self.define(Define::new(
                DefineKind::Function(func.returns.1.clone()),
                func.name.clone(),
            ))?;
            self.listup_defines_function(func)?;
        }
        Ok(())
    }

    fn listup_defines_function(&mut self, func: &unchecked::SysDCFunction) -> anyhow::Result<()> {
        for (name, types) in &func.args {
            self.define(Define::new(
                DefineKind::Variable(types.clone()),
                name.clone(),
            ))?;
            self.define(Define::new(
                DefineKind::Argument(types.clone()),
                name.clone(),
            ))?;
        }
        for annotation in &func.annotations {
            if let unchecked::SysDCAnnotation::Spawn {
                result: (name, types),
                details,
            } = annotation
            {
                self.define(Define::new(
                    DefineKind::Variable(types.clone()),
                    name.clone(),
                ))?;
                self.listup_defines_annotation_spawn_details(details)?;
            }
        }
        Ok(())
    }

    fn listup_defines_annotation_spawn_details(
        &mut self,
        details: &Vec<unchecked::SysDCSpawnDetail>,
    ) -> anyhow::Result<()> {
        for detail in details {
            match detail {
                unchecked::SysDCSpawnDetail::Use(name, _) => {
                    let outer_spawn_namespace = name.clone().get_par_name(true);
                    let outer_use_name = Name::new(&outer_spawn_namespace, name.clone().name);
                    self.define(Define::new(
                        DefineKind::Use(outer_use_name.clone()),
                        name.clone(),
                    ))?;
                }
                unchecked::SysDCSpawnDetail::LetTo {
                    name,
                    func: (_, func),
                    ..
                } => {
                    self.define(Define::new(
                        DefineKind::Variable(func.clone()),
                        name.clone(),
                    ))?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn split_name(s: &str) -> (String, Option<String>) {
    let splitted = s.split('.').collect::<Vec<&str>>();
    match splitted.len() {
        1 => (splitted[0].to_string(), None),
        _ => (splitted[0].to_string(), Some(splitted[1..].join("."))),
    }
}
