// Copyright (c) 2023, Oskar Ohlenmacher
// All rights reserved
//
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

//TODO
// see: https://plantuml.com/class-diagram
//
// {method} and {field} for specific declaration
// . in class names indicate file structure
//      example: net.beans.ClassName is in ./net/beans/ClassName.java
//
// add more errors
use std::error::Error;
use std::fs::read_to_string;
use std::{fs::File, io::Read, path::Path};

use log::info;
/* use std::{
    error::Error,
    fmt::Display,
    fs::{File, read_to_string},
    io::{self, BufRead, BufReader, ErrorKind, Read},
    path::Path,
}; */

#[derive(Debug)]
pub enum Token {
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

enum FunctionType {
    Java,       // ReturnType Name(ParameterType Parameter) { ... }
    Rust,       // Name(Parameter: ParameterType) -> ReturnType { ... }
    Typescript, // Name(Parameter: ParameterType): ReturnType { ... }
}

enum AttributeType {
    Java,       // Type Name
    Typescript, // Name: Type
}

// TODO merge lists ot one
pub fn get_identifiers<'a>(filepath: &Path) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    info!("Opened {:?} to parse from", filepath);

    let mut searcher = Searcher::new(file, FunctionType::Java, AttributeType::Java);
    searcher.search()?;

    Ok(searcher.tokens)
}

struct Searcher {
    file: File,
    function_type: FunctionType,
    attribute_type: AttributeType,

    tokens: Vec<Token>,
    buffer: String,
}

impl Searcher {
    fn new(file: File, function_type: FunctionType, attribute_type: AttributeType) -> Self {
        Self {
            file,
            function_type,
            attribute_type,
            tokens: Vec::new(),
            buffer: String::new(),
        }
    }

    fn search(&mut self) -> Result<(), SearchError> {
        // 1. @startyaml
        // 2. class, enum, etc.
        // 3. attributes/functions/relations
        //    3.1. functions: parameters, returntype
        //    3.2. attributes: publicity, name, type
        //    3.3. relations e.g. inherits
        // 4. search for @endyaml
        if let Err(e) = self.file.read_to_string(&mut self.buffer) {
            return Err(SearchError::Error(Box::new(e)));
        }

        self.buffer = self.buffer.lines().map(|l| l.trim_start()).filter(|l| !l.starts_with('\'')).intersperse("\n").collect();
        log::debug!("{}", self.buffer);

        for (line_number, line) in self.buffer.lines().enumerate() {
            // println!("{}", line);
            if line.starts_with("@startuml") {
                self.tokens.push(Token::Startuml);
                return self.start_global(line_number);
            }
        }
        Err(SearchError::NoStartYaml)
    }

    fn start_global(&mut self, line_number: usize) -> Result<(), SearchError> {
        for (line_number, line) in self.buffer.lines().enumerate().skip(line_number + 1) {
            log::debug!("{} -> ({})", line_number, line);
            if line.starts_with("class") {
                self.tokens.push(Token::Class);
                return self.search_class(line_number, false);
            } else if line.starts_with("abstract") {
                self.tokens.push(Token::Abstract);
                return self.search_class(line_number, true);
            } else if line.starts_with("interface") {
                // return self.search_class(line_number);
                self.tokens.push(Token::Interface);
                todo!("Impmlement search_interface");
            } else if line.starts_with("@enduml") {
                return Ok(());
            }
        }
        Err(SearchError::NoEndYaml)
    }

    fn search_class(&mut self, line_number: usize, is_abstract: bool) -> Result<(), SearchError> {
        // TODO
        let top_line = self.buffer.lines().nth(line_number).unwrap();
        let words: Vec<&str> = top_line.split_whitespace().skip(1).collect();
        let mut name = if is_abstract {
            if words[0] == "class" {
                words[1]
            } else {
                words[0]
            }
        } else {
            words[0]
        };

        if let Some(s) = name.strip_suffix("{") {
            name = s;
        }
        self.tokens.push(Token::Name(name.to_string()));
        if words.last().unwrap().ends_with("{") {
            self.tokens.push(Token::StartObject);
        }

        log::debug!("{:?} -> name: {}", words, name);
        // log::debug!("\"{}\" -> {:#?}", top_line, words);
        Ok(())
    }
}

#[derive(Debug)]
enum SearchError {
    Error(Box<dyn Error>),
    NoStartYaml,
    NoEndYaml,
}

impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /* fn type_id(&self, _: private::Internal) -> std::any::TypeId
    where
        Self: 'static,
    {
        std::any::TypeId::of::<Self>()
    } */

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    // fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Error(e) => write!(f, "Error: {}", e),
            SearchError::NoEndYaml => write!(f, "No @endyaml found"),
            SearchError::NoStartYaml => write!(f, "No @endstart found"),
        }
    }
}

// struct Searcher<'a> {
//     tokens: Vec<Token>,
//     file_reader: BufReader<File>,
//
//     line: &'a str,
//     buffer: String,
//     line_number: usize,
//
//     uml_started: bool,
// }
//
// impl Searcher<'_> {
//     fn new(file_reader: BufReader<File>) -> Self {
//         Self {
//             tokens: Vec::new(),
//             file_reader,
//             buffer: String::new(),
//             line: "",
//             line_number: 0,
//             uml_started: false,
//         }
//     }
//
//     fn next_line<'a>(&'a mut self) -> Result<(), SearchError> {
//         self.buffer.clear();
//         if let Err(e) = self.file_reader.read_line(&mut self.buffer) {
//             // TODO if line ended before uml
//             return Err(SearchError::IOError(e));
//         }
//         self.line_number += 1;
//         let buf = self.buffer.trim_start();
//         // self.buffer.clear();
//
//         if !self.uml_started {
//             if buf.starts_with("@startuml") {
//                 self.uml_started = true;
//             } else {
//                 log::info!("Skipping line to search for @startuml {}", self.line_number);
//                 // return self.next_line();
//                 return Err(SearchError::SkipLine {
//                     reason: String::from("Skipping line to search for @startuml"),
//                     line_number: self.line_number,
//                 });
//             }
//         } else {
//             if buf.starts_with("@enduml") {
//                 return Err(SearchError::UmlEnded);
//             }
//
//             if buf.starts_with("'") {
//                 log::info!("Skipping line {}", self.line_number);
//                 // return self.next_line();
//                 return Err(SearchError::SkipLine {
//                     reason: String::from("Commented line"),
//                     line_number: self.line_number,
//                 });
//             }
//         }
//
//         self.line = self.buffer.trim_start();
//         Ok(())
//     }
//
//     fn search_global(&mut self) -> Result<(), SearchError> {
//         loop {
//             // FIXME how to access self in match statement
//             let line = self.line_number;
//             match self.next_line() {
//                 Ok(_) => {
//                     info!("Chekcing line {}", line + 1);
//
//                     for ignore_string in ["skinparam"].iter() {
//                         if self.line.starts_with(ignore_string) {
//                             continue;
//                         }
//                     }
//
//                     match self.line.split_whitespace().nth(0) {
//                         Some("class") => {
//                             self.search_class()?;
//                         }
//                         Some("enum") => {}
//                         Some("interface") => {}
//                         _ => (),
//                     }
//
//                     continue;
//                 }
//                 Err(e) => match e {
//                     SearchError::SkipLine { .. } => continue,
//                     SearchError::UmlEnded => break,
//                     e => return Err(e),
//                 },
//             }
//         }
//         Ok(())
//     }
//
//     fn search_class(&mut self) -> Result<(), SearchError> {
//         // search for name than { "class Name {" or "class Name{"
//         println!("{}", self.line);
//
//         Ok(())
//     }
//
//     fn start_search(&mut self) -> Result<(), SearchError> {
//         self.search_global()
//     }
// }
//
// #[derive(Debug)]
// enum SearchError {
//     IOError(std::io::Error),
//     SkipLine { reason: String, line_number: usize },
//     UmlEnded,
// }
//
// impl Display for SearchError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::UmlEnded => write!(f, "The UML part of the file ended"),
//             Self::SkipLine {
//                 reason,
//                 line_number,
//             } => write!(f, "Line {} skipped, reason: \"{}\"", line_number, reason),
//             Self::IOError(e) => write!(f, "IOError: \"{}\"", e),
//         }
//     }
// }
//
// impl std::error::Error for SearchError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
//
//     // fn type_id(&self, _: private::Internal) -> std::any::TypeId
//     // where
//     //     Self: 'static,
//     // {
//     //     std::any::TypeId::of::<Self>()
//     // }
//
//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }
//
//     fn cause(&self) -> Option<&dyn Error> {
//         self.source()
//     }
//
//     // fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
// }

// TODO remove pub
// fn parse_line(line: &String) -> Vec<Identifier> {
//     match line.as_str() {
//         "@startuml " => return vec![Identifier::Startuml],
//         "@enduml " => return vec![Identifier::Enduml],
//         _ => (),
//     };
//
//     let mut out = Vec::new();
//     let mut second_object_started = false;
//     let mut currently_inheriting = false;
//     let mut ident = String::new();
//     let chars = line.chars();
//     for char in chars {
//         match char {
//             '\'' => return Vec::new(),
//             ' ' => match ident.as_str() {
//                 "class" => out.push(Identifier::Class),
//                 "interface" => out.push(Identifier::Interface),
//                 "abstract" => out.push(Identifier::Abstract),
//                 _ => {
//                     match out.last() {
//                         Some(Identifier::InheritesLeft) => currently_inheriting = false,
//                         _ => (),
//                     }
//                     if ident.trim().len() > 0 {
//                         match out.last() {
//                             Some(Identifier::Variable(_) | Identifier::EndMethod) => {
//                                 out.push(Identifier::Type(ident.clone()))
//                             }
//                             Some(Identifier::Class) | Some(Identifier::Interface) => continue,
//                             _ => {
//                                 out.push(Identifier::Name(ident.clone()));
//                                 continue;
//                             }
//                         }
//                     } else {
//                         continue;
//                     }
//                 }
//             },
//             '+' => out.push(Identifier::Public),
//             '#' => out.push(Identifier::Protected),
//             '-' => {
//                 if !currently_inheriting {
//                     out.push(Identifier::Private);
//                 }
//             }
//             ':' => {
//                 if ident.trim().len() > 0 {
//                     match out.last() {
//                         Some(Identifier::Name(_)) => {out.pop();},
//                         _ => (),
//                     }
//                     out.push(Identifier::Variable(ident.clone()))
//                 }
//             }
//             '\n' => {
//
//                 // if ident.trim().len() > 0 {
//                 // }
//             }
//             '{' => {
//                 if ident.trim().len() > 0 {
//                     out.push(Identifier::Name(ident.clone()));
//                     out.push(Identifier::StartObject);
//                 } else {
//                     second_object_started = true;
//                 }
//             }
//             '(' => {
//                 out.push(Identifier::Name(ident.clone()));
//                 out.push(Identifier::StartMethod);
//             }
//             '}' => {
//                 if second_object_started {
//                     match ident.as_str() {
//                         "abstract" => out.push(Identifier::Abstract),
//                         "static" => out.push(Identifier::Static),
//                         _ => (),
//                     }
//                     second_object_started = false;
//                 } else {
//                     out.push(Identifier::EndObject);
//                 }
//             }
//             ')' => {
//                 if ident.trim().len() > 0 {
//                     out.push(Identifier::Type(ident.clone()));
//                 }
//                 out.push(Identifier::EndMethod)
//             }
//             ',' => out.push(Identifier::Type(ident.clone())),
//             '>' => {
//                 loop {
//                     match out.last() {
//                         Some(Identifier::Private) => {
//                             out.pop();
//                         }
//                         _ => break,
//                     }
//                 }
//                 out.push(Identifier::InheritesRight);
//             }
//             '<' => {
//                 out.push(Identifier::InheritesLeft);
//                 currently_inheriting = true;
//             }
//             _ => {
//                 ident.push(char.clone());
//                 continue;
//             }
//         }
//         ident.clear();
//     }
//
//     out
// }
