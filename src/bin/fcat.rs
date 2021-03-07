// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use anyhow::{Context, Result};
use flatcat::{FlatCat, Format, FormatHint, Input, Output, OutputOpts};
use std::str::FromStr;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = flatcat::cli_parser::Opts::from_args();

    let output_opts = OutputOpts::new()
        .with_color(!opts.no_color)
        .with_null(!opts.no_null)
        .with_quotes(!opts.no_quotes)
        .with_value_counter(opts.value_counter);
    let output = Output::new(output_opts);

    let mut flat_cat = FlatCat::new(Default::default(), output);
    for file in opts.files {
        let mut input = Input::from_path(file);
        if let Some(ref format) = opts.format {
            let format = Format::from_str(format).context("failed to parse format option")?;
            input = input.with_format_hint(FormatHint::hint(format));
        }
        flat_cat.cat(input).context("failed to cat file")?;
    }

    Ok(())
}
