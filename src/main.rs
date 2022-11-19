use lexer::Indentifier;
use model::{Attribute, Class, View};

mod lexer;
mod model;
use model::View::*;
use model::{Function, Type};

// use crate::model::Function;

// TODO Read file and write file
// add super() in constructor
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
    let idents = lexer::get_identifiers("/home/oskar/dev/rust/java2plantuml/test.puml");
    // for class in get_objects(&idents.unwrap()).iter() {
    //     write_class(class);
    // }
    println!("{}", class.as_string());
    println!("{:?}", idents);
    let binding = idents.unwrap();
    let classes = get_objects(&binding);
    for class in classes {
        println!("{}", class.as_string());
        println!("{:?}", class);
    }
}

// TODO wait for start/enduml
fn get_objects<'a>(indents: &Vec<Indentifier>) -> Vec<Class> {
    // wait for class -> get data about class until object end -> construct
    // class with data
    let mut out = Vec::new();
    let mut skip = 0;
    for i in 0..indents.len() {
        if skip > i {
            println!("in i skip loop");
            continue;
        }
        match indents[i] {
            Indentifier::Class => {
                let mut classname = "Klasse";
                match &indents[i + 1] {
                    Indentifier::Name(n) => classname = n,
                    _ => (),
                }
                let mut class = Class::build(&classname, Public, false);

                let mut skipk = i + 3;
                for k in (i + 3)..indents.len() {
                    if skipk > k && skipk < indents.len() {
                        println!("in k skip loop skipk:{} k:{} idnet:{}", skipk, k, indents.len());
                        continue;
                    }

                    let mut view = View::Normal;
                    match &indents[k] {
                        Indentifier::Public => view = View::Public,
                        Indentifier::Private => view = View::Private,
                        Indentifier::Protected => view = View::Private,
                        // add attribute if var is found
                        Indentifier::Variable(var) => {
                            match &indents[k + 1] {
                                Indentifier::Type(vartype) => {
                                    class = class.with_attribute(Attribute::new(
                                        view,
                                        var,
                                        Type::Other(vartype),
                                        false,
                                    ));
                                }
                                _ => (),
                            };
                        }
                        // add method if name is found
                        Indentifier::Name(name) => {
                            let mut parameters = Vec::new();
                            let mut is_abstract = false;
                            let mut skipm = k + 2;
                            for m in (k + 2)..indents.len() {
                                if skipm > m {
                                    println!("in m skip loop");
                                    continue;
                                }

                                match &indents[m] {
                                    Indentifier::Variable(var) => match &indents[m + 1] {
                                        Indentifier::Type(vartype) => {
                                            parameters.push(Attribute::new(
                                                Normal,
                                                var,
                                                Type::Other(vartype),
                                                false,
                                            ));
                                            skipm += 1;
                                        }
                                        _ => (),
                                    },
                                    Indentifier::Abstract => is_abstract = true,
                                    Indentifier::EndMethod => {
                                        let mut returntype = Type::Other("");
                                        // chagne returntype of method if there is one
                                        match &indents[m + 1] {
                                            Indentifier::Type(methodtype) => {
                                                returntype = Type::Other(methodtype);
                                            }
                                            _ => (),
                                        }
                                        class = class.with_method(Function::new(
                                            name,
                                            view,
                                            returntype,
                                            parameters.clone(),
                                            is_abstract,
                                        ));
                                        break;
                                    }
                                    _ => continue,
                                }
                            }
                            skipk = skipm;
                        }
                        Indentifier::EndObject => break,
                        _ => continue,
                    }
                }
                skip = skipk;
                out.push(class);
            }
            Indentifier::InheritesLeft => {}
            Indentifier::InheritesRight => {}
            _ => (),
        };
    }
    out
}

fn write_class<'a>(class: &Class<'a>) {
    todo!()
}
