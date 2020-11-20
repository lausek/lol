use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;
use crate::{LOLC_EXTENSION, LOL_EXTENSION};

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

    pub fn context_mut(&mut self) -> &mut Context {
        self.vm.context_mut()
    }

    pub fn call<T>(&mut self, name: &str, args: &[T]) -> Lovm2Result<Value>
    where
        T: Into<Value> + Clone,
    {
        let args: Vec<Value> = args.iter().map(T::clone).map(T::into).collect();
        self.vm.call(name, args.as_ref())
    }

    pub fn load(&mut self, module: Module) -> Lovm2Result<()> {
        self.vm.load_and_import_all(module)
    }

    pub fn run(&mut self) -> Lovm2Result<()> {
        self.vm.run()
    }

    pub fn run_from_path<T>(&mut self, path: T) -> Lovm2Result<()>
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

        self.vm.load_and_import_all(module)?;

        self.vm.run()
    }
}
