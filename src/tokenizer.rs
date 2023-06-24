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
use std::{fs::File, io::Read, path::Path};

use log::info;

#[derive(Debug)]
pub enum Token {
    Class,
    AbstractClass,
    Interface,
    Enum,
    StartObject,
    EndObject,
    StartMethod,
    EndMethod,

    Public,
    Protected,
    PackagePrivate,
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
    // Line(usize),
}

enum AttributeType {
    Java,       // Type Name
    Typescript, // Name: Type
}

// TODO merge lists ot one
pub fn get_identifiers<'a>(filepath: &Path) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    info!("Opened {:?} to parse from", filepath);

    let mut searcher = Searcher::new(file, AttributeType::Typescript);
    searcher.search()?;

    Ok(searcher.tokens)
}

struct Searcher {
    file: File,
    attribute_type: AttributeType,

    tokens: Vec<Token>,
    buffer: String,
}

impl Searcher {
    fn new(file: File, attribute_type: AttributeType) -> Self {
        Self {
            file,
            attribute_type,
            tokens: Vec::new(),
            buffer: String::new(),
        }
    }

    fn search(&mut self) -> Result<(), SearchError> {
        if let Err(e) = self.file.read_to_string(&mut self.buffer) {
            return Err(SearchError::Error(Box::new(e)));
        }

        // FIXME use &str instead of String
        // filter out comemnts and empty lines
        self.buffer = self
            .buffer
            .lines()
            .map(|l| {
                l.trim_start()
                    .chars()
                    .take_while(|c| *c != '\'')
                    .collect::<String>()
            })
            .filter(|l| !l.starts_with('\'') && !l.is_empty())
            .intersperse("\n".to_string())
            .collect();
        // log::debug!("{}", self.buffer);

        for (line_number, line) in self.buffer.lines().enumerate() {
            // self.tokens.push(Token::Line(line_number+1));
            // println!("{}", line);
            if line.starts_with("@startuml") {
                self.tokens.push(Token::Startuml);
                return self.search_global(line_number);
            }
        }
        Err(SearchError::NoStartYaml)
    }

    fn search_global(&mut self, line_number: usize) -> Result<(), SearchError> {
        // FIXME borrow checker issue To not return search_class, instead just call it
        for (line_number, line) in self.buffer.lines().enumerate().skip(line_number + 1) {
            // self.tokens.push(Token::Line(line_number+1));
            // log::debug!("{} -> ({})", line_number, line);
            if line.starts_with("class") {
                self.tokens.push(Token::Class);
                return self.search_class(line_number, false);
            } else if line.starts_with("abstract") {
                self.tokens.push(Token::AbstractClass);
                return self.search_class(line_number, true);
            } else if line.starts_with("interface") {
                // return self.search_class(line_number);
                self.tokens.push(Token::Interface);
                todo!("Impmlement search_interface");
            } else if line.starts_with("@enduml") {
                self.tokens.push(Token::Enduml);
                return Ok(());
            } else {
                // if second word is a inherit push neccessary things into tokens
                let words: Vec<&str> = line.split_whitespace().collect();
                match words.get(1) {
                    Some(&"<|--") | Some(&"<|..") => {
                        self.tokens.push(Token::Name(words[0].to_string()));
                        self.tokens.push(Token::InheritesLeft);
                        // FIXME if words[2] is None crashes aka if nothing inherits words[0] ->
                        // "bla <|-- "
                        self.tokens.push(Token::Name(words[2].to_string()));
                    }
                    Some(&"--|>") | Some(&"..|>") => {
                        self.tokens.push(Token::Name(words[0].to_string()));
                        self.tokens.push(Token::InheritesRight);
                        // FIXME if words[2] is None crashes aka if nothing inherits words[0] ->
                        // "bla <|-- "
                        self.tokens.push(Token::Name(words[2].to_string()));
                    }
                    Some(_) | None => (),
                }
            }
        }
        Err(SearchError::NoEndYaml)
    }

    fn search_class(&mut self, line_number: usize, is_abstract: bool) -> Result<(), SearchError> {
        // TODO
        let top_line = self.buffer.lines().nth(line_number).unwrap();
        let words: Vec<&str> = top_line.split_whitespace().skip(1).collect();
        // set name if abstract check if only "abstract" or "abstract class" is written
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
            // todo!("start search for attributes, methods etc.");

            for (line_number, line) in self.buffer.lines().enumerate().skip(line_number + 1) {
                // self.tokens.push(Token::Line(line_number+1));
                log::debug!("{} .. {:?}", line_number, line);
                if line == "}" {
                    self.tokens.push(Token::EndObject);
                    break;
                }

                // self.search_inner_class(line_number);
                // TODO hwo to use function instead of very long match statement
                match self.attribute_type {
                    AttributeType::Typescript => {
                        let mut skip = 1;
                        match line.chars().nth(0) {
                            Some('+') => self.tokens.push(Token::Public),
                            Some('-') => self.tokens.push(Token::Private),
                            Some('~') => self.tokens.push(Token::PackagePrivate),
                            Some('#') => self.tokens.push(Token::Protected),
                            Some(_) => skip = 0,
                            _ => (),
                        }
                        let mut buf = String::new();
                        let mut skip_next = false;
                        let mut in_method = false;
                        for c in line.chars().filter(|c| !c.is_whitespace()).skip(skip) {
                            if skip_next {
                                skip_next = false;
                                continue;
                            }

                            // log::debug!("{}", c);
                            // TODO use less clone()
                            //      {abstract} {static} {method} etc.
                            match c {
                                '\\' => skip_next = true,
                                '(' => {
                                    self.tokens.push(Token::Name(buf.clone()));
                                    self.tokens.push(Token::StartMethod);
                                    in_method = true;
                                    buf.clear();
                                }
                                ')' => {
                                    if !buf.is_empty() {
                                        self.tokens.push(Token::Type(buf.clone()));
                                    }
                                    self.tokens.push(Token::EndMethod);
                                    in_method = false;
                                    buf.clear();
                                }
                                ':' => {
                                    if !buf.is_empty() {
                                        self.tokens.push(Token::Variable(buf.clone()));
                                        buf.clear();
                                    }
                                }
                                ',' => {
                                    if in_method {
                                        self.tokens.push(Token::Type(buf.clone()));
                                        buf.clear();
                                    }
                                }
                                '{' => buf.clear(),
                                '}' => {
                                    self.tokens.push(match buf.as_str() {
                                        "static" => Token::Static,
                                        "classifier" => Token::Static,
                                        "abstract" => Token::Abstract,
                                        _ => {
                                            return Err(SearchError::UnknwonInCurlyBraces(
                                                line_number + 1,
                                                buf.clone(),
                                            ))
                                        }
                                    });
                                    buf.clear();
                                }
                                x => buf.push(x),
                            }
                        }
                        if !buf.is_empty() {
                            self.tokens.push(Token::Type(buf.clone()));
                        }
                    }
                    AttributeType::Java => {}
                }
            }
        }

        // log::debug!("top line: {} -> {:?} -> name: {}", top_line, words, name);
        // log::debug!("\"{}\" -> {:#?}", top_line, words);
        return self.search_global(line_number);
    }

    // fn search_line_in_class_ts(&mut self, line: &str) {}
}

#[derive(Debug)]
enum SearchError {
    Error(Box<dyn Error>),
    NoStartYaml,
    NoEndYaml,
    UnknwonInCurlyBraces(usize, String),
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
            SearchError::UnknwonInCurlyBraces(line_number, s) => write!(
                f,
                "Unknown word in Brace on line {}: \"{}\"",
                line_number, s
            ),
        }
    }
}
