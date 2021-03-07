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
    /// Files to flatten and print
    #[structopt(name = "FILE")]
    pub files: Vec<String>,
    /// Disables colorful output
    #[structopt(long = "no-color")]
    pub no_color: bool,
    /// Ignores null values, if applicable for file type
    #[structopt(long = "no-null")]
    pub no_null: bool,
    /// Disables catting unrecognized file types as plain
    #[structopt(long = "no-plain")]
    pub no_plain: bool,
    /// Disables quoting ("text") strings
    #[structopt(long = "no-quotes")]
    pub no_quotes: bool,
    /// Numbers the output values, starting at 1
    #[structopt(long = "value-numbers")]
    pub value_numbers: bool,
    /// Sets file type instead of guessing
    #[structopt(name = "TYPE", short = "t", long = "type", possible_values(& ["json", "toml", "yaml"]))]
    pub format: Option<String>,
}
