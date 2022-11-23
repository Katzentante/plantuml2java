mod lexer;
mod model;
mod generate;

// TODO 
// add super() in constructor
// add CLI see: https://www.rust-lang.org/what/cli
// impl Object trait

fn main() {
    env_logger::init();
    generate::generate_files("test.puml", "./java_folder/");
}

