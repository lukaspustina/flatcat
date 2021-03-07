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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "fcat",
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_setting(structopt::clap::AppSettings::UnifiedHelpMessage),
)]
struct Opts {
    /// File to flatten and print
    file: String,
    /// Disables colorful output
    #[structopt(long = "no-color")]
    no_color: bool,
    /// Disables quoting ("text") strings
    #[structopt(long = "no-quotes")]
    no_quotes: bool,
    /// Numbers the output values, starting at 1
    #[structopt(long = "value-counter")]
    value_counter: bool,
    /// Sets file type instead of guessing
    #[structopt(short = "t", long = "type", possible_values(& ["json", "toml", "yaml"]), validator(| str | Format::from_str(& str).map(| _ | ()).map_err(| _ | "invalid file type".to_string())))]
    format: Option<Format>,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let mut input = Input::from_path(opts.file);
    if let Some(format) = opts.format {
        input = input.with_format_hint(FormatHint::hint(format));
    }

    let opts = OutputOpts::new()
        .with_color(!opts.no_color)
        .with_quotes(!opts.no_quotes)
        .with_value_counter(opts.value_counter);
    let output = Output::new(opts);

    let opts = Default::default();
    let mut flat_cat = FlatCat::new(opts, output);

    flat_cat.cat(input).context("Failed to cat file")?;

    Ok(())
}
