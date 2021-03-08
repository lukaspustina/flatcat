// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::str::FromStr;

use anyhow::{Context, Result};
use structopt::StructOpt;

use flatcat::cli_parser::Opts;
use flatcat::output::Output;
use flatcat::{FlatCat, FlatCatOpts, Format, FormatHint, Input, OutputOpts};

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let output_opts = OutputOpts::new()
        .with_color(!opts.no_color)
        .with_null(!opts.no_null)
        .with_quotes(!opts.no_quotes)
        .with_numbers(opts.numbers)
        .with_end_of_lines(opts.end_of_line);
    let output = Output::from_stdout(output_opts);

    let flatcat_opts = FlatCatOpts::new().with_flatten(opts.flatten);
    let mut flatcat = FlatCat::new(flatcat_opts, output).context("failed to instantiate FlatCat")?;

    cats(&opts, &mut flatcat).context("failed to cat file")?;

    Ok(())
}

/// Cat all given files; if non given, read from stdin
fn cats(opts: &Opts, mut flatcat: &mut FlatCat) -> Result<()> {
    if opts.files.is_empty() {
        let input = create_input("-");
        cat(opts.format.as_ref(), &mut flatcat, input)?
    } else {
        for file in &opts.files {
            let input = create_input(file);
            cat(opts.format.as_ref(), &mut flatcat, input)?
        }
    }

    Ok(())
}

/// Create `Input` based on path; if single '-' use stdin
fn create_input(path: &str) -> Input {
    if path == "-" {
        Input::from_stdin()
    } else {
        Input::from_path(path)
    }
}

/// Cat single file
fn cat(format: Option<&String>, flatcat: &mut FlatCat, mut input: Input) -> Result<()> {
    if let Some(format) = format {
        let format = Format::from_str(format).context("failed to parse format option")?;
        input = input.with_format_hint(FormatHint::hint(format));
    }
    flatcat.cat(input)?;

    Ok(())
}
