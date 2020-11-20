use clap::{App, Arg, SubCommand};
use std::path::{Path, PathBuf};

use lol::interpreter::Interpreter;
use lol::transpiler::Transpiler;
use lol::{LOLC_EXTENSION, LOL_EXTENSION};

fn find_files<T>(path: T, files: &mut Vec<PathBuf>) -> std::io::Result<()>
where
    T: AsRef<Path>,
{
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        if let Some(ext) = entry.path().extension() {
            if ext == LOL_EXTENSION {
                files.push(entry.path().clone());
            }
        }
    }

    Ok(())
}

fn build(path: PathBuf) -> Option<PathBuf> {
    use lovm2::prelude::ENTRY_POINT;

    let mut files = vec![];
    let mut entry_point = None;

    let mut srcdir = path.clone();
    srcdir.push("src");
    find_files(&srcdir, &mut files).unwrap();

    let mut targetdir = path;
    targetdir.push("target");
    targetdir.push("lol");
    std::fs::create_dir_all(&targetdir).unwrap();

    let mut transpiler = Transpiler::new();

    for file in files.into_iter() {
        let relname = file.strip_prefix(&srcdir).unwrap().clone();
        let mut targetfile = targetdir.clone();
        targetfile.push(relname);
        targetfile.set_extension(LOLC_EXTENSION);

        println!("building {} ...", file.to_str().unwrap());

        let module = transpiler.build_from_source(file).unwrap();

        module.store_to_file(&targetfile).unwrap();

        if module.slot(&ENTRY_POINT.into()).is_some() {
            entry_point = Some(targetfile);
        }
    }

    entry_point
}

fn run<T>(path: Option<T>)
where
    T: AsRef<Path>,
{
    let mut int = Interpreter::new();

    match path {
        Some(path) => {
            if let Err(e) = int.run_from_path(path) {
                println!("{:?}", e);
            }
        }
        _ => {
            let dir = std::env::current_dir().unwrap();
            match build(dir) {
                Some(main) => {
                    if let Err(e) = int.run_from_path(main) {
                        println!("{:?}", e);
                    }
                }
                _ => println!("no entry point"),
            }
        }
    }
}

fn main() {
    let mut app = App::new("lol")
        .version("0.1.0")
        .author("lausek")
        .about("lol - lausek's own lisp")
        .subcommand(
            SubCommand::with_name("run")
                .about("run the project or a specific lol file")
                .arg(Arg::with_name("FILE").help("the name of the file to run")),
        )
        .subcommand(SubCommand::with_name("build").about("build the current directory"));
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("build", _) => {
            let dir = std::env::current_dir().unwrap();
            build(dir);
        }
        ("run", matches) => {
            let fname = matches
                .unwrap()
                .value_of("FILE")
                .map(|fname| std::fs::canonicalize(fname).unwrap());
            run(fname);
        }
        _ => app.print_help().unwrap(),
    }
}
