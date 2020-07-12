use lovm2::vm::Vm;
use std::path::Path;

use crate::transpiler::Transpiler;

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
        let source = std::fs::read_to_string(path).map_err(|e| format!("{}", e))?;
        let (sexprs, err) = ess::parser::parse(source.as_ref());
        if let Some(err) = err {
            return Err(format!("{:?}", err));
        }

        let mut trans = Transpiler::new();
        let module = trans.translate(&sexprs)?;
        println!("{:#?}", module);
        self.vm.load_and_import_all(module)?;
        self.vm.run()
    }
}