pub fn create_lol_module(
    name: &str,
    src: &str,
) -> lovm2::prelude::Lovm2Result<lovm2::module::Module> {
    let mut trans = crate::Transpiler::new();
    let meta: lovm2::module::meta::ModuleMeta = name.to_string().into();
    let module: lovm2::module::Module = trans.build(meta, src).unwrap();
    Ok(module)
}

pub fn create_lol_runtime(name: &str, src: &str) -> crate::Interpreter {
    let mut int = crate::Interpreter::new();
    let module = crate::create_lol_module(name, src).unwrap();

    println!("{}", module);

    int.load(module).unwrap();
    int
}
