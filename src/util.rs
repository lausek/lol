#[macro_export]
macro_rules! create_lol_module {
    ($name:expr, $src:expr) => {{
        let mut trans = lol::Transpiler::new();
        let meta: lovm2::module::meta::ModuleMeta = $name.to_string().into();
        let module = trans.build(meta, $src).unwrap();
        module
    }};
}

#[macro_export]
macro_rules! create_lol_runtime {
    ($name:expr, $src:expr) => {{
        let mut int = lol::Interpreter::new();
        let module = lol::create_lol_module!($name, $src);

        println!("{}", module);

        int.load(module).unwrap();
        int
    }};
}
