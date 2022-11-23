use clap::Parser;

mod lexer;
mod model;
mod generate;

// TODO 
// add CLI see: https://www.rust-lang.org/what/cli
// impl Object trait


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
    generate::generate_files(&args.input, &args.output);
}

