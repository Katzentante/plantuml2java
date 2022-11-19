use crate::{lexer::{self, Indentifier}, model::{Class, View, Type, Attribute, Function}};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn generate_files(inputfile: &str, outputlocation: &str) {
    let idents = match lexer::get_identifiers(inputfile) {
        Ok(o) => o,
        Err(e) => panic!("Error during creation of idnets: {}", e),
    };
    let classes = get_objects(&idents);
    for class in classes.iter() {
        write_class(class, outputlocation);
    }
}


// TODO: wait for start/enduml
// FIX: use abstract properly
fn get_objects<'a>(indents: &Vec<Indentifier>) -> Vec<Class> {
    // wait for class -> get data about class until object end -> construct
    // class with data
    let mut out = Vec::new();
    let mut skip = 0;
    for i in 0..indents.len() {
        if skip > i {
            // println!("in i skip loop");
            continue;
        }
        match indents[i] {
            Indentifier::Class => {
                let mut classname = "Klasse";
                match &indents[i + 1] {
                    Indentifier::Name(n) => classname = n,
                    _ => (),
                }
                let mut class = Class::build(&classname, View::Public, false);
                let mut is_abstract = false;

                let mut skipk = i + 3;
                for k in (i + 3)..indents.len() {
                    if skipk > k && skipk < indents.len() {
                        // println!("in k skip loop skipk:{} k:{} idnet:{}", skipk, k, indents.len());
                        continue;
                    }

                    let mut view = View::Normal;
                    match &indents[k] {
                        Indentifier::Public => view = View::Public,
                        Indentifier::Private => view = View::Private,
                        Indentifier::Protected => view = View::Private,
                        Indentifier::Abstract => is_abstract = true,
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
                                    is_abstract = false;
                                }
                                _ => (),
                            };
                        }
                        // add method if name is found
                        Indentifier::Name(name) => {
                            let mut parameters = Vec::new();
                            let mut skipm = k + 2;
                            for m in (k + 2)..indents.len() {
                                if skipm > m {
                                    // println!("in m skip loop");
                                    continue;
                                }

                                match &indents[m] {
                                    Indentifier::Variable(var) => match &indents[m + 1] {
                                        Indentifier::Type(vartype) => {
                                            parameters.push(Attribute::new(
                                                View::Normal,
                                                var,
                                                Type::Other(vartype),
                                                false,
                                            ));
                                            skipm += 1;
                                        }
                                        _ => (),
                                    },
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
                                        is_abstract = false;
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

fn write_class<'a>(class: &Class<'a>, location: &str) {
    let pathname = format!("{}{}.java", location, class.name);
    let path = Path::new(&pathname);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(class.as_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
