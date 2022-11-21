mod lexer;
mod model;
mod generate;

// TODO 
// add super() in constructor
// impl Object trait

fn main() {
    generate::generate_files("test.puml", "./java_folder/")
}

