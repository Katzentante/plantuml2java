use std::{
    env::VarError,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
pub enum Indentifier {
    Class,
    Interface,
    Enum,
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
pub fn get_identifiers<'a>(filename: &'a str) -> std::io::Result<Vec<Indentifier>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let list: Vec<Vec<Indentifier>> = io::BufReader::new(file)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        /* .map(|mut line| {
            line.push(' ');
            line
        }) */
        .map(|line| parse_line(line))
        .collect();
    let mut out = Vec::new();
    list.iter().map(|l| out.extend(*l));
    Ok(out)
}

// TODO remove pub
// - remove pub
// - use single lines to parse see commeted above
// - impl parsing of extends
pub fn parse_line(mut line: String) -> Vec<Indentifier> {
    // ignore comments
    match line.chars().nth(0) {
        Some('\'') => return Vec::new(),
        _ => (),
    };
    println!("{:?}", line);
    match line.as_str() {
        "@startuml" => return vec![Indentifier::Startuml],
        "@enduml" => return vec![Indentifier::Enduml],
        _ => (),
    };

    line.push(' ');

    let mut out = Vec::new();
    let mut object_started = false;
    let mut second_object_started = false;
    let mut ident = String::new();
    let chars = line.chars();
    for char in chars {
        match char {
            ' ' => match ident.as_str() {
                "class" => out.push(Indentifier::Class),
                "interface" => out.push(Indentifier::Interface),
                _ => {
                    if ident.trim().len() > 0 {
                        match out.last() {
                            Some(Indentifier::Variable(_) | Indentifier::EndMethod) => {
                                out.push(Indentifier::Type(ident.clone()))
                            }
                            _ => {
                                out.push(Indentifier::Name(ident.clone()));
                                continue;
                            }
                        }
                    } else {
                        continue;
                    }
                }
            },
            '+' => out.push(Indentifier::Public),
            '#' => out.push(Indentifier::Protected),
            '-' => out.push(Indentifier::Private),
            ':' => {
                if ident.trim().len() > 0 {
                    out.pop();
                    out.push(Indentifier::Variable(ident.clone()))
                }
            }
            '\n' => {
                // if ident.trim().len() > 0 {
                // }
            }
            '{' => {
                if !object_started {
                    out.push(Indentifier::Name(ident.clone()));
                    out.push(Indentifier::StartObject);
                    object_started = true;
                } else {
                    second_object_started = true;
                }
            }
            '(' => {
                out.push(Indentifier::Name(ident.clone()));
                out.push(Indentifier::StartMethod)
            }
            '}' => {
                if second_object_started {
                    match ident.as_str() {
                        "abstract" => out.push(Indentifier::Abstract),
                        "static" => out.push(Indentifier::Static),
                        _ => (),
                    }
                    second_object_started = false;
                } else {
                    out.push(Indentifier::EndObject);
                    object_started = false;
                }
            }
            ')' => {
                if ident.trim().len() > 0 {
                    out.push(Indentifier::Type(ident.clone()));
                }
                out.push(Indentifier::EndMethod)
            }
            ',' => out.push(Indentifier::Type(ident.clone())),
            '>' => out.push(Indentifier::InheritesRight),
            '<' => out.push(Indentifier::InheritesLeft),
            _ => {
                ident.push(char.clone());
                continue;
            }
        }
        ident.clear();
    }

    out
}
