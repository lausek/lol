use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;
use crate::{LOLC_EXTENSION, LOL_EXTENSION};

fn load_lol_module<T>(path: T) -> Result<Module, String>
where
    T: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) if ext == LOL_EXTENSION => {
            let mut trans = Transpiler::new();
            trans.build_from_path(path)
        }
        Some(ext) if ext == LOLC_EXTENSION => {
            Module::load_from_file(path).map_err(|e| format!("{:?}", e))
        }
        _ => Err("invalid file extension".to_string()),
    }
}

fn load_hook(req: &lovm2::vm::LoadRequest) -> Lovm2Result<Option<Module>> {
    if let Ok(path) = lovm2::vm::find_candidate(req) {
        let module = load_lol_module(path)?;
        return Ok(Some(module));
    }
    Ok(None)
}

fn import_hook(module: Option<&str>, name: &str) -> String {
    match module {
        Some(module) => format!("{}-{}", module, name),
        _ => name.to_string(),
    }
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut vm = Vm::with_std();

        vm.set_load_hook(load_hook);
        vm.set_import_hook(import_hook);

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
        // import module namespaced
        self.vm.add_module(module, true)
    }

    pub fn load_global(&mut self, module: Module) -> Lovm2Result<()> {
        // import module namespaced
        self.vm.add_module(module, false)
    }

    pub fn load_main(&mut self, module: Module) -> Lovm2Result<()> {
        self.vm.add_main_module(module)
    }

    pub fn run(&mut self) -> Lovm2Result<Value> {
        self.vm.run()
    }

    pub fn run_from_path<T>(&mut self, path: T) -> Lovm2Result<Value>
    where
        T: AsRef<Path>,
    {
        let module = load_lol_module(path)?;

        self.vm.add_main_module(module)?;

        self.vm.run()
    }
}
