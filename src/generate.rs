use crate::{
    lexer::{self, Identifier},
    model::{Attribute, Class, Function, Type, View},
};
use log::{error, info};
use std::fs::{File, self};
use std::io::prelude::*;
use std::path::Path;

pub fn generate_files(inputfile: &str, outputlocation: &str) {
    match fs::create_dir_all(outputlocation) {
        Ok(_) => (),
        Err(e) => panic!("Error while creating folder {} : {}", outputlocation, e),
    }
    let idents = match lexer::get_identifiers(inputfile) {
        Ok(idents) => idents,
        Err(e) => panic!("Error during creation of idnets: {}", e),
    };
    // println!("{:?}", idents);

    // println!("{:?}", idents);
    let classes = get_objects(&idents);
    // println!("{:?}", classes);
    // println!("{:?}", classes);
    for class in classes.iter() {
        write_class(class, outputlocation);
    }
}

// TODO:
// wait for start/enduml
// FIXME: Add errors
fn get_objects<'a>(idents: &'a [Identifier]) -> Vec<Class<'a>> {
    info!("{:?}", idents);
    let mut classes = Vec::new();
    let mut is_abstract = false;
    let mut i = 0;
    // let mut iditer = idents.iter().peekable();

    while i < idents.len() {
        match &idents[i] {
            Identifier::Abstract => is_abstract = true,
            Identifier::Class => match &idents[i + 1] {
                Identifier::Name(name) => {
                    match &idents[i + 2] {
                        Identifier::StartObject => {
                            let (skip, mut class) = gen_class(idents, i + 3, name);
                            class = class.with_abstract(is_abstract);
                            // let mut class = Class::build(name, View::Public, false);
                            i += skip + 3;
                            classes.push(class);
                            is_abstract = false;
                        }
                        _ => error!(
                            "Expected start of object after name class identifier, id:{}",
                            i
                        ),
                    };
                }
                _ => error!("Expected name after class identifier, id:{}", i),
            },
            Identifier::InheritesLeft => {
                let mastername = match &idents[i - 1] {
                    Identifier::Name(name) => name,
                    _ => continue,
                };
                let mut childname = "";
                for j in i..idents.len() {
                    match &idents[j] {
                        Identifier::Name(name) => {
                            childname = name;
                            break;
                        }
                        _ => continue,
                    };
                }
                info!("{}<|--{}", mastername, childname);
                let master = match classes.iter().find(|c| c.name == mastername) {
                    Some(c) => c.clone(),
                    None => panic!("Expected master class {}", mastername),
                };
                let child = match classes.iter_mut().find(|c| c.name == childname) {
                    Some(c) => c,
                    None => panic!("Expected child class {}", childname),
                };
                child.set_inherits(master);
            }
            Identifier::InheritesRight => {
                let mastername = match &idents[i + 1] {
                    Identifier::Name(name) => name,
                    _ => continue,
                };
                let mut childname = "";
                for j in (0..i).rev() {
                    info!("test rightinherit");
                    match &idents[j] {
                        Identifier::Name(name) => {
                            childname = name;
                            break;
                        }
                        _ => continue,
                    };
                }
                info!("{}<|--{}", mastername, childname);
                let master = match classes.iter().find(|c| c.name == mastername) {
                    Some(c) => c.clone(),
                    None => panic!("Expected master class {}", mastername),
                };
                let child = match classes.iter_mut().find(|c| c.name == childname) {
                    Some(c) => c,
                    None => panic!("Expected child class {}", childname),
                };
                child.set_inherits(master);
                // info!("{:?}", child.get_inherits().unwrap());
            }
            _ => (),
        }
        i += 1;
    }

    classes
}

fn gen_class<'a>(idents: &'a [Identifier], index: usize, classname: &'a str) -> (usize, Class<'a>) {
    let mut class = Class::build(classname, View::Public, false);
    let mut skip = 0;
    let mut is_abstract = false;
    let mut is_static = false;
    let mut i = index;
    let mut view = View::Normal;

    while i < idents.len() {
        // println!("{:?}", idents[i]);
        match &idents[i] {
            Identifier::Public => view = View::Public,
            Identifier::Private => view = View::Private,
            Identifier::Protected => view = View::Protected,
            Identifier::Abstract => {
                is_abstract = true;
                class = class.with_abstract(true)
            }
            Identifier::Static => is_static = true,
            Identifier::Variable(varname) => {
                match &idents[i + 1] {
                    Identifier::Type(vartype) => {
                        class = class.with_attribute(Attribute::new(
                            view,
                            varname,
                            Type::Other(vartype),
                            false,
                        ));
                    }
                    _ => error!("Unexpected Identifier after Variable"),
                };
                view = View::Normal;
                is_static = false;
                is_abstract = false;
                i += 1;
                skip += 1;
            }
            Identifier::StartMethod => match &idents[i - 1] {
                Identifier::Name(methodname) => {
                    let (mskip, method) =
                        gen_method(idents, i + 1, methodname, view, is_abstract, is_static);
                    i += mskip;
                    skip += mskip;
                    class = class.with_method(method);
                    view = View::Normal;
                    is_static = false;
                    is_abstract = false;
                }
                _ => error!("Expected methodname beofre methodstart id:{}", i),
            },
            Identifier::EndObject => break,
            _ => (),
        }
        i += 1;
    }

    (skip, class)
}

fn gen_method<'a>(
    idents: &'a [Identifier],
    index: usize,
    methodname: &'a str,
    view: View,
    is_abstract: bool,
    is_static: bool,
) -> (usize, Function<'a>) {
    let mut paremeters = Vec::new();
    let mut skip = 0;
    let mut i = index;
    let mut returntype = Type::Other("");

    while i < idents.len() {
        match &idents[i] {
            Identifier::Variable(varname) => match &idents[i + 1] {
                Identifier::Type(typename) => {
                    paremeters.push(Attribute::new(view, varname, Type::Other(typename), false))
                }
                _ => error!("Expected type after var name"),
            },
            Identifier::EndMethod => match &idents[i + 1] {
                Identifier::Type(returnname) => {
                    returntype = Type::Other(returnname);
                    i += 1;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
        i += 1;
    }
    skip += i - index;

    (
        skip,
        Function::new(
            methodname,
            view,
            returntype,
            paremeters,
            is_abstract,
            is_static,
        ),
    )
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
    match file.write_all(class.to_java().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("successfully wrote to {}", display),
    }
}
