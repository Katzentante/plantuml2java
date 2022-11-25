use clap::Parser;
use log::error;

mod lexer;
mod model;
mod generate;

// TODO 

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String
}


fn main() {
   let args = Args::parse();
    env_logger::init();
    match generate::generate_files(&args.input, &args.output) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        },
    }
}

