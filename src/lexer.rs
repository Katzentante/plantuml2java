// Copyright (c) 2023, Oskar Ohlenmacher
// All rights reserved
//
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree. 

use log::info;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
pub enum Identifier {
    Class,
    Interface,
    // Enum,
    StartObject,
    EndObject,
    StartMethod,
    EndMethod,

    Public,
    Protected,
    Private,
    Abstract,
    Static,

    Type(String),
    Variable(String),
    Name(String),

    InheritesLeft,
    InheritesRight,

    Startuml,
    Enduml,
}

// TODO merge lists ot one
pub fn get_identifiers<'a>(filename: &'a str) -> std::io::Result<Vec<Identifier>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    info!("Opened {} to parse from", filename);
    let mut out = Vec::new();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(|mut x| {
            x.push(' ');
            x
        })
        .collect();
    for line in lines.iter() {
        out.extend(parse_line(&line))
    }
    // list.iter().map(|l| out.extend(*l));
    Ok(out)
}

// TODO remove pub
fn parse_line(line: &String) -> Vec<Identifier> {
    match line.as_str() {
        "@startuml " => return vec![Identifier::Startuml],
        "@enduml " => return vec![Identifier::Enduml],
        _ => (),
    };

    let mut out = Vec::new();
    let mut second_object_started = false;
    let mut currently_inheriting = false;
    let mut ident = String::new();
    let chars = line.chars();
    for char in chars {
        match char {
            '\'' => return Vec::new(),
            ' ' => match ident.as_str() {
                "class" => out.push(Identifier::Class),
                "interface" => out.push(Identifier::Interface),
                "abstract" => out.push(Identifier::Abstract),
                _ => {
                    match out.last() {
                        Some(Identifier::InheritesLeft) => currently_inheriting = false,
                        _ => (),
                    }
                    if ident.trim().len() > 0 {
                        match out.last() {
                            Some(Identifier::Variable(_) | Identifier::EndMethod) => {
                                out.push(Identifier::Type(ident.clone()))
                            }
                            Some(Identifier::Class) | Some(Identifier::Interface) => continue,
                            _ => {
                                out.push(Identifier::Name(ident.clone()));
                                continue;
                            }
                        }
                    } else {
                        continue;
                    }
                }
            },
            '+' => out.push(Identifier::Public),
            '#' => out.push(Identifier::Protected),
            '-' => {
                if !currently_inheriting {
                    out.push(Identifier::Private);
                }
            }
            ':' => {
                if ident.trim().len() > 0 {
                    match out.last() {
                        Some(Identifier::Name(_)) => {out.pop();},
                        _ => (),
                    }
                    out.push(Identifier::Variable(ident.clone()))
                }
            }
            '\n' => {

                // if ident.trim().len() > 0 {
                // }
            }
            '{' => {
                if ident.trim().len() > 0 {
                    out.push(Identifier::Name(ident.clone()));
                    out.push(Identifier::StartObject);
                } else {
                    second_object_started = true;
                }
            }
            '(' => {
                out.push(Identifier::Name(ident.clone()));
                out.push(Identifier::StartMethod);
            }
            '}' => {
                if second_object_started {
                    match ident.as_str() {
                        "abstract" => out.push(Identifier::Abstract),
                        "static" => out.push(Identifier::Static),
                        _ => (),
                    }
                    second_object_started = false;
                } else {
                    out.push(Identifier::EndObject);
                }
            }
            ')' => {
                if ident.trim().len() > 0 {
                    out.push(Identifier::Type(ident.clone()));
                }
                out.push(Identifier::EndMethod)
            }
            ',' => out.push(Identifier::Type(ident.clone())),
            '>' => {
                loop {
                    match out.last() {
                        Some(Identifier::Private) => {
                            out.pop();
                        }
                        _ => break,
                    }
                }
                out.push(Identifier::InheritesRight);
            }
            '<' => {
                out.push(Identifier::InheritesLeft);
                currently_inheriting = true;
            }
            _ => {
                ident.push(char.clone());
                continue;
            }
        }
        ident.clear();
    }

    out
}
