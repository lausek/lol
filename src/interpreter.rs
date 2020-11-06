use lovm2::module::ModuleProtocol;
use lovm2::prelude::*;
use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;

fn to_str(e: lovm2::prelude::Lovm2Error) -> String {
    format!("{:?}", e)
}

fn load_hook(req: &lovm2::context::LoadRequest) -> Lovm2Result<Option<GenericModule>> {
    if let Ok(path) = lovm2::context::find_candidate(req) {
        use std::rc::Rc;
        let mut trans = Transpiler::new();
        return trans
            .process(path)
            .map(|m| Some(Rc::new(m) as Rc<dyn ModuleProtocol>))
            .map_err(|e| e.into());
    }

    Ok(None)
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut vm = Vm::new();

        vm.context_mut().set_load_hook(load_hook);

        Self { vm }
    }

    pub fn run_source<T>(&mut self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let mut trans = Transpiler::new();

        let module = trans.process(path)?;

        self.vm.load_and_import_all(module).map_err(to_str)?;

        self.vm.run().map_err(to_str)
    }
}
