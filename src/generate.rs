// Copyright (c) 2023, Oskar Ohlenmacher
// All rights reserved
//
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    tokenizer::{self, Token},
    model::{Attribute, Class, Function, Type, View},
};
use log::{debug, error, info};
use std::fs::{self, File};
use std::path::Path;
use std::{error::Error, io::prelude::*};

pub fn generate_files(
    inputfile: &str,
    outputlocation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // check inputfile and outputlocation
    let inputfile = Path::new(inputfile);
    let outputlocation = Path::new(outputlocation);

    if !outputlocation.exists() {
        if outputlocation.is_dir() {
            fs::create_dir_all(outputlocation)?;
        } else {
            error!("Given output is not a directory");
            return Err(Box::new(CustomError::OutputNotDirectory));
        }
    }

    if !inputfile.exists() {
        error!("Input file does not exist");
        return Err(Box::new(CustomError::InputNotFound));
    } else {
        if !inputfile.is_file() {
            error!("Given input is not a file");
            return Err(Box::new(CustomError::InputNotFile));
        }
    }

    let idents = tokenizer::get_identifiers(inputfile)?;
    let classes = get_classes(&idents)
        .or_else(|err| {
            error!("{}", err);
            return Err(Box::new(err));
        })
        .unwrap();

    for class in classes.iter() {
        write_class(class, &outputlocation)?
    }

    Ok(())
}

fn write_class<'a>(class: &Class<'a>, location: &Path) -> Result<(), std::io::Error> {
    let classpath = Path::new(class.name).with_extension("java");
    let path = Path::join(location, classpath);
    let mut file = File::create(&path)?;
    file.write_all(class.to_java().as_bytes())?;
    info!("successfully wrote to {}", path.display());
    Ok(())
}

#[derive(Debug)]
enum GeneratorError {
    UnexpectedIdentifier(String),
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedIdentifier(str) => write!(f, "{}", str),
        }
    }
}

#[derive(Debug)]
enum CustomError {
    Utf8ParseError,
    OutputNotDirectory,
    InputNotFile,
    InputNotFound,
    // InputWrongExtension
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8ParseError => write!(f, "There is a utf8 error"),
            Self::InputNotFile => write!(f, "Input file is a directory"),
            Self::InputNotFound => write!(f, "Input file not present"),
            Self::OutputNotDirectory => write!(f, "Output directory is a file"),
            // _ => write!(f, "Some Error"),
        }
    }
}

impl Error for CustomError {}

// TODO:
// wait for start/enduml
/* fn get_classes<'a>(idents: &'a [Token]) -> Result<Vec<Class<'a>>, GeneratorError> {
    debug!("Converting identifiers: {:?}", idents);
    let mut classes = Vec::new();
    let mut is_abstract = false;
    let mut i = 0;
    // let mut iditer = idents.iter().peekable();

    while i < idents.len() {
        match &idents[i] {
            Token::Abstract => is_abstract = true,
            Token::Class => match &idents[i + 1] {
                Token::Name(name) => {
                    match &idents[i + 2] {
                        Token::StartObject => {
                            let (skip, mut class) = gen_class(idents, i + 3, name)?;
                            class = class.with_abstract(is_abstract);
                            // let mut class = Class::build(name, View::Public, false);
                            i += skip + 3;
                            classes.push(class);
                            is_abstract = false;
                        }
                        _ => {
                            let s =
                                format!("Expected Start of object after the class name {{name}}");
                            return Err(GeneratorError::UnexpectedIdentifier(s));
                        }
                    };
                }
                _ => {
                    let s = format!("Expected name after class statement");
                    return Err(GeneratorError::UnexpectedIdentifier(s));
                }
            },
            Token::InheritesLeft => {
                let mastername = match &idents[i - 1] {
                    Token::Name(name) => name,
                    _ => {
                        return Err(GeneratorError::UnexpectedIdentifier(
                            "Expected Name to be iherited from".to_string(),
                        ))
                    }
                };
                let childname = match &idents[i + 1] {
                    Token::Name(name) => name,
                    _ => {
                        let s = format!("Expected name to inherit {}", mastername);
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };

                debug!("{}<|--{}", mastername, childname);
                let master = match classes.iter().find(|c| c.name == mastername) {
                    Some(c) => c.clone(),
                    None => {
                        let s = format!("class {mastername} doesn't exist to be inherited");
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };
                let child = match classes.iter_mut().find(|c| c.name == childname) {
                    Some(c) => c,
                    None => {
                        let s = format!("class {childname} doesn't exist to inherit {mastername}");
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };
                child.set_inherits(master);
            }
            Token::InheritesRight => {
                let mastername = match &idents[i + 1] {
                    Token::Name(name) => name,
                    _ => {
                        return Err(GeneratorError::UnexpectedIdentifier(
                            "Expected Name to be iherited from".to_string(),
                        ))
                    }
                };
                let childname = match &idents[i - 1] {
                    Token::Name(name) => name,
                    _ => {
                        let s = format!("Expected name to inherit {}", mastername);
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };

                debug!("{}<|--{}", mastername, childname);
                let master = match classes.iter().find(|c| c.name == mastername) {
                    Some(c) => c.clone(),
                    None => {
                        let s = format!("class {mastername} doesn't exist to be inherited");
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };
                let child = match classes.iter_mut().find(|c| c.name == childname) {
                    Some(c) => c,
                    None => {
                        let s = format!("class {childname} doesn't exist to inherit {mastername}");
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };
                child.set_inherits(master);
            }
            _ => (),
        }
        i += 1;
    }

    Ok(classes)
}

fn gen_class<'a>(
    idents: &'a [Token],
    index: usize,
    classname: &'a str,
) -> Result<(usize, Class<'a>), GeneratorError> {
    let mut class = Class::build(classname, View::Public, false);
    let mut skip = 0;
    let mut is_abstract = false;
    let mut is_static = false;
    let mut i = index;
    let mut view = View::Normal;

    while i < idents.len() {
        match &idents[i] {
            Token::Public => view = View::Public,
            Token::Private => view = View::Private,
            Token::Protected => view = View::Protected,
            Token::Abstract => {
                is_abstract = true;
                class = class.with_abstract(true)
            }
            Token::Static => is_static = true,
            Token::Variable(varname) => {
                match &idents[i + 1] {
                    Token::Type(vartype) => {
                        class = class.with_attribute(Attribute::new(
                            view,
                            varname,
                            Type::Other(vartype),
                            false,
                        ));
                    }
                    _ => {
                        let s = format!(
                            "Expected Identifier \"Type\" after Variable \"{}\"",
                            varname
                        );
                        return Err(GeneratorError::UnexpectedIdentifier(s));
                    }
                };
                view = View::Normal;
                is_static = false;
                is_abstract = false;
                i += 1;
                skip += 1;
            }
            Token::StartMethod => match &idents[i - 1] {
                Token::Name(methodname) => {
                    let (mskip, method) =
                        gen_method(idents, i + 1, methodname, view, is_abstract, is_static)?;
                    i += mskip;
                    skip += mskip;
                    class = class.with_method(method);
                    view = View::Normal;
                    is_static = false;
                    is_abstract = false;
                }
                _ => {
                    let s = format!("Expected a method name");
                    return Err(GeneratorError::UnexpectedIdentifier(s));
                }
            },
            Token::EndObject => break,
            _ => (),
        }
        i += 1;
    }

    Ok((skip, class))
}

fn gen_method<'a>(
    idents: &'a [Token],
    index: usize,
    methodname: &'a str,
    view: View,
    is_abstract: bool,
    is_static: bool,
) -> Result<(usize, Function<'a>), GeneratorError> {
    let mut paremeters = Vec::new();
    let mut skip = 0;
    let mut i = index;
    let mut returntype = Type::Other("");

    while i < idents.len() {
        match &idents[i] {
            Token::Variable(varname) => match &idents[i + 1] {
                Token::Type(typename) => {
                    paremeters.push(Attribute::new(view, varname, Type::Other(typename), false))
                }
                _ => {
                    let s = format!(
                        "Expected Identifier \"Type\" after Variable \"{}\"",
                        varname
                    );
                    return Err(GeneratorError::UnexpectedIdentifier(s));
                }
            },
            Token::EndMethod => match &idents[i + 1] {
                Token::Type(returnname) => {
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

    Ok((
        skip,
        Function::new(
            methodname,
            view,
            returntype,
            paremeters,
            is_abstract,
            is_static,
        ),
    ))
} */

