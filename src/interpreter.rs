use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;
use crate::{LOLC_EXTENSION, LOL_EXTENSION};

fn to_str(e: lovm2::prelude::Lovm2Error) -> String {
    format!("{:?}", e)
}

fn import_hook(module: &str, name: &str) -> String {
    if module.is_empty() {
        return name.to_string();
    }
    format!("{}.{}", module, name)
}

fn load_hook(req: &lovm2::context::LoadRequest) -> Lovm2Result<Option<Module>> {
    if let Ok(path) = lovm2::context::find_candidate(req) {
        let mut trans = Transpiler::new();
        return trans
            .build_from_source(path)
            .map(|m| Some(m))
            .map_err(|e| e.into());
    }

    Ok(None)
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut vm = Vm::with_std();

        vm.context_mut().set_import_hook(import_hook);
        vm.context_mut().set_load_hook(load_hook);

        Self { vm }
    }

    /*
    pub fn run_source<T>(&mut self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let mut trans = Transpiler::new();

        let module = trans.build_from_source(path)?;

        self.vm.load_and_import_all(module).map_err(to_str)?;

        self.vm.run().map_err(to_str)
    }
    */

    pub fn run<T>(&mut self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let module = match path.as_ref().extension() {
            Some(ext) if ext == LOL_EXTENSION => {
                let mut trans = Transpiler::new();
                trans.build_from_source(path)
            }
            Some(ext) if ext == LOLC_EXTENSION => {
                Module::load_from_file(path).map_err(|e| format!("{:?}", e))
            }
            _ => Err("invalid file extension".to_string()),
        }?;

        self.vm.load_and_import_all(module).map_err(to_str)?;

        self.vm.run().map_err(to_str)
    }
}
