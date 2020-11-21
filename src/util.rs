#[macro_export]
macro_rules! create_lol_runtime {
    ($name:expr, $src:expr) => {{
        let mut int = lol::Interpreter::new();
        let mut trans = lol::Transpiler::new();
        let meta: ModuleMeta = $name.to_string().into();
        let module = trans.build(meta, $src).unwrap();

        println!("{}", module);

        int.load(module).unwrap();
        int
    }};
}
