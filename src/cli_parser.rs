/*
 * Copyright 2021 Lukas Pustina <lukas@pustina.de>
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 *
 */

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "fcat",
author = env!("CARGO_PKG_AUTHORS"),
about = env!("CARGO_PKG_DESCRIPTION"),
global_setting(structopt::clap::AppSettings::UnifiedHelpMessage),
)]
pub struct Opts {
    /// Files to flatten, print, and concat; if single '-' or absent, fcat reads from stdin
    #[structopt(name = "FILE")]
    pub files: Vec<String>,
    /// Disables colorful output
    #[structopt(long = "no-color")]
    pub no_color: bool,
    /// Ignores null values, if applicable for file type
    #[structopt(long = "no-null")]
    pub no_null: bool,
    /// Flattens files with identified, hierarchically structured format
    #[structopt(short = "f", long = "flatten")]
    pub flatten: bool,
    /// Disables quoting ("text") strings
    #[structopt(long = "no-quotes")]
    pub no_quotes: bool,
    /// Numbers the output values, starting at 1
    #[structopt(short = "n", long = "numbers")]
    pub numbers: bool,
    /// Displays a dollar sign ('$') at the end of each line
    #[structopt(short = "e", long = "line-end")]
    pub end_of_line: bool,
    /// Sets file type instead of guessing
    #[structopt(name = "TYPE", short = "t", long = "type", possible_values(& ["json", "toml", "yaml"]))]
    pub format: Option<String>,
    /// Lists known file types / extensions for supported formats
    #[structopt(long = "type-list")]
    pub type_list: bool,
}
