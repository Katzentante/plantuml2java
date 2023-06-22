// Copyright (c) 2023, Oskar Ohlenmacher
// All rights reserved
//
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use clap::Parser;
use log::error;

mod generate;
mod tokenizer;
mod model;

// TODO
// impl interfaces / enums -> evt. trait object
// auto inflict interface methods
// fix lexer -> fix comments
// finish file checks aka concat files for env::current_dir()

/// Convert .puml files to java classes / interfaces not jet implemented
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file (.puml)
    #[arg(short, long)]
    input: String,

    /// The output folder - is created if non existend
    #[arg(short, long)]
    output: String,
}

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();
    let args = Args::parse();
    if let Err(e) = generate::generate_files(&args.input, &args.output) {
        error!("{}", e);
    }
}
