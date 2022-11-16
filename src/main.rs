use model::Attribute;

mod lexer;
mod convert;
mod model;
use model::{Function, Type};
use model::View::*;

// use crate::model::Function;

// TODO Read file and write file
// impl Object trait
fn main() {
    let class = model::Class::build("Held", Public, true)
        .with_attribute(Attribute::new(Public, "stärke", Type::Other("int"), false))
        .with_attribute(Attribute::new(
            Public,
            "lebenspunkte",
            Type::Other("int"),
            false,
        ))
        .with_attribute(Attribute::new(
            Public,
            "angriffswert",
            Type::Other("int"),
            false,
        ))
        .with_attribute(Attribute::new(Public, "name", Type::Other("String"), false))
        .with_attribute(Attribute::new(Public, "waffe", Type::Other("Waffe"), false))
        .with_method(Function::new(
            "addLebenspunkte",
            Public,
            Type::Other("boolean"),
            vec![Attribute::new(Normal, "faktor", Type::Other("int"), true)],
            false,
        ));
    println!("{}", class.as_string());
    println!("{:?}", lexer::get_identifiers("class Held{\n+ method_name(held: Held) : void\n+ name : type\n}\n".to_string()));
}

// fn get_objects<'a>(indents: Vec<Identifier>) -> Vec<dyn Object> {
//     todo!()
// }
