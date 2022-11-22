mod lexer;
mod model;
mod generate;
use log::info;

// TODO 
// add super() in constructor
// impl Object trait

fn main() {
    env_logger::init();
    generate::generate_files("test.puml", "./java_folder/")
}

