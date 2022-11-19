use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use generate::generate_files;
use lexer::Indentifier;
use model::{Attribute, Class, View};

mod lexer;
mod model;
mod generate;
use model::View::*;
use model::{Function, Type};

// use crate::model::Function;

// TODO Read file and write file
// add super() in constructor
// impl Object trait
//
//

const JAVA_FOLDER: &'static str = "/home/oskar/dev/rust/test/javafolder";

fn main() {
    generate::generate_files("test.puml", "./java_folder/")
}

