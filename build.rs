// Copyright 2017-2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::env;
use std::fs;
use std::process;
use structopt::{clap::Shell, StructOpt};

#[allow(dead_code)]
#[path = "src/cli_parser.rs"]
mod cli_parser;

fn main() {
    // OUT_DIR is set by Cargo and is the only place which is allowed to be written to during compilation for 'cargo publish' and docs.rs.
    let output_dir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!("Cargo output directory environment variable is not set: Cannot continue. Aborting");
            process::exit(1);
        }
    };
    fs::create_dir_all(&output_dir).expect("failed to create output directory");

    // Create Shell completions
    let mut parser = cli_parser::Opts::clap();
    parser.gen_completions("fcat", Shell::Bash, &output_dir);
    parser.gen_completions("fcat", Shell::Fish, &output_dir);
    parser.gen_completions("fcat", Shell::Zsh, &output_dir);
}
