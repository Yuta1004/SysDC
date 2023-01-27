use std::hash::Hash;
use std::collections::HashMap;

use crate::{ Advice, AdviceLevel };
use sysdc_core::name::Name;
use sysdc_core::structure::{ SysDCSystem, SysDCFunction, SysDCAnnotation };

pub fn eval_duplication_stat(system: &SysDCSystem) -> Option<Advice> {
    let mut fid_issuer: IdIssuer<String> = IdIssuer::new();
    let mut fabst_all = HashMap::new();
    for unit in &system.units {
        for module in &unit.modules {
            for func in &module.functions {
                let (fname, fabst) = gen_func_abst(&mut fid_issuer, &func);
                fabst_all.insert(fname.get_full_name(), fabst);
            }
        }
    }

    let mut hid_issuer: IdIssuer<FuncAbst> = IdIssuer::new();
    let mut grouped_f: HashMap<i32, Vec<String>> = HashMap::new();
    for (fname, fabst) in fabst_all.into_iter() {
        let hid = hid_issuer.get_id(fabst);
        match grouped_f.get_mut(&hid) {
            Some(v) => v.push(fname),
            None => { grouped_f.insert(hid, vec![fname]); }
        }
    }

    let mut messages = vec![];
    for (_, fnames) in grouped_f.iter() {
        if fnames.len() > 1 {
            messages.push(fnames.join(" / "))
        }
    }

    match messages.len() {
        0 => None,
        _ => Some(
            Advice::new(
                AdviceLevel::Warning,
                "統合可能な処理".to_string(),
                messages
            )
        )
    }
}

type FuncAbst = (
    i32,            // return
    Vec<i32>,       // args
    Vec<i32>,       // affect
);

struct IdIssuer<T>
    where T: Eq + Hash
{
    items: HashMap<T, i32>
}

impl<T> IdIssuer<T>
   where T: Eq + Hash
{
    pub fn new() -> IdIssuer<T> {
        IdIssuer { items: HashMap::new() }
    }

    pub fn get_id(&mut self, item: T) -> i32 {
        match self.items.get(&item) {
            Some(id) => *id,
            None => {
                self.items.insert(item, self.items.len() as i32);
                (self.items.len() - 1) as i32
            }
        }
    }
}

fn gen_func_abst<'a>(id_issuer: &mut IdIssuer<String>, func: &'a SysDCFunction) -> (&'a Name, FuncAbst) {
    // return
    let ret_id = match &func.returns.1.refs {
        Some(rtype) => id_issuer.get_id(rtype.get_full_name()),
        None => id_issuer.get_id(format!("{:?}", func.returns.1.kind))
    };

    // args
    let mut args_set = vec![];
    for (_, atype) in &func.args {
        let id = match &atype.refs {
            Some(atype) => id_issuer.get_id(atype.get_full_name()),
            None => id_issuer.get_id(format!("{:?}", func.returns.1.kind))
        };
        args_set.push(id);
    }
    args_set.sort();

    // affect
    let mut affect_set = vec![];
    for anno in &func.annotations {
        match anno {
            SysDCAnnotation::Affect { func: (fname, _), .. } => {
                affect_set.push(id_issuer.get_id(fname.get_full_name()))
            },
            _ => {}
        }
    }

    (&func.name, (ret_id, args_set, affect_set))
}
