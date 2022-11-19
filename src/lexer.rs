use std::{
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
fn parse_line(line: &String) -> Vec<Indentifier> {
    // ignore comments
    match line.chars().nth(0) {
        Some('\'') => return Vec::new(),
        _ => (),
    };
    // println!("{:?}", line);
    match line.as_str() {
        "@startuml " => return vec![Indentifier::Startuml],
        "@enduml " => return vec![Indentifier::Enduml],
        _ => (),
    };

    let mut out = Vec::new();
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
                if ident.trim().len() > 0 {
                    out.push(Indentifier::Name(ident.clone()));
                    out.push(Indentifier::StartObject);
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
