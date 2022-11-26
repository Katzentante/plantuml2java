use clap::Parser;
use log::error;

mod generate;
mod lexer;
mod model;

// TODO
// impl interfaces / enums -> evt. trait object
// auto inflict interface methods
// fix lexer -> fix comments
// finish file checks aka concat files for env::current_dir()

/// Convert .puml files to java classes / interfaces not jet implemented
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file (.puml)
    #[arg(short, long)]
    input: String,

    /// The output folder - is created if non existend
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    match generate::generate_files(&args.input, &args.output) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}
