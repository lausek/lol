use std::path::{Path, PathBuf};

use structopt::StructOpt;

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

        let module = transpiler.build_from_path(file).unwrap();

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

#[derive(StructOpt)]
#[structopt(
    name = "lol",
    version = "0.4.8",
    author = "lausek",
    about = "lol - lausek's own lisp"
)]
enum CliOptions {
    #[structopt()]
    Build {
        #[structopt(name = "DIRECTORY")]
        path: String,
    },
    #[structopt()]
    Run {
        #[structopt(name = "FILE")]
        path: Option<String>,
    },
}

fn main() {
    let args = CliOptions::from_args();

    match args {
        CliOptions::Build { path } => {
            build(Path::new(&path).to_path_buf());
        }
        CliOptions::Run { path } => {
            run(path.as_ref().map(Path::new));
        }
    }
}
