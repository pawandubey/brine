use std::fs;

use camino::Utf8PathBuf;
use rustyline::{error::*, Editor};
use structopt::StructOpt;

mod tree_walk;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Options {
    #[structopt(help = "Path to source file")]
    file: Option<Utf8PathBuf>,
}

fn main() {
    let options = Options::from_args();

    match options.file {
        Some(file) => run_file(&file),
        None => run_prompt(),
    }
}

fn run_prompt() {
    let pkg_version = option_env!("CARGO_PKG_VERSION").unwrap_or("?");
    let prompt = format!("(brine:{})> ", pkg_version);
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => run(line),
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }
    }
}

fn run_file(file: &Utf8PathBuf) {
    let source = fs::read_to_string(file).expect("Error reading file");
    run(source)
}

fn run(source: String) {
    tree_walk::TreeWalk::new().run(source)
}
