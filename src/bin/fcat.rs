// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use anyhow::{Context, Result};
use flatcat::cli_parser::Opts;
use flatcat::output::Output;
use flatcat::{FlatCat, FlatCatOpts, Format, FormatHint, Input, OutputOpts};
use std::str::FromStr;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let output_opts = OutputOpts::new()
        .with_color(!opts.no_color)
        .with_null(!opts.no_null)
        .with_quotes(!opts.no_quotes)
        .with_numbers(opts.numbers);
    let output = Output::from_stdout(output_opts);

    let flatcat_opts = FlatCatOpts::new().with_plain(!opts.no_plain);
    let mut flatcat = FlatCat::new(flatcat_opts, output).context("failed to instantiate FlatCat")?;

    for file in opts.files {
        let mut input = if file == "-" {
            Input::from_stdout()
        } else {
            Input::from_path(file)
        };

        if let Some(ref format) = opts.format {
            let format = Format::from_str(format).context("failed to parse format option")?;
            input = input.with_format_hint(FormatHint::hint(format));
        }
        flatcat.cat(input).context("failed to cat file")?;
    }

    Ok(())
}
