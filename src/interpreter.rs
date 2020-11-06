use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;

fn to_str(e: lovm2::prelude::Lovm2Error) -> String {
    format!("{:?}", e)
}

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { vm: Vm::new() }
    }

    pub fn run_source<T>(&mut self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let mut trans = Transpiler::new();

        let module = trans.process(path)?;
        println!("{:#?}", module);

        self.vm.load_and_import_all(module).map_err(to_str)?;

        self.vm.run().map_err(to_str)
    }
}
