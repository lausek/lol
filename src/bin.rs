use clap::{App, Arg, SubCommand};
use std::path::Path;

use lol::interpreter::Interpreter;

fn run<T>(path: T)
where
    T: AsRef<Path>,
{
    let mut int = Interpreter::new();
    if let Err(e) = int.run_source(path) {
        println!("{:?}", e);
    }
}

fn main() {
    let app = App::new("lol")
        .version("0.0.1")
        .author("lausek")
        .about("lol - lausek's own lisp")
        .subcommand(
            SubCommand::with_name("run").about("run a lol file").arg(
                Arg::with_name("FILE")
                    .required(true)
                    .help("the name of the file to run"),
            ),
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("run", matches) => {
            let fname = matches.unwrap().value_of("FILE").unwrap();
            let fname = std::fs::canonicalize(fname).unwrap();
            run(fname);
        }
        _ => panic!("no subcommand"),
    }
}
