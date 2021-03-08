// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub use error::Error;

pub use crate::format::{Format, FormatHint};
pub use crate::input::Input;
use crate::input::InputReader;
use crate::output::OutputWriter;
pub use crate::output::{Output, OutputOpts};

use std::convert::TryInto;

pub mod catter;
pub mod cli_parser;
pub mod error;
pub mod file_extension;
pub mod format;
pub mod input;
pub mod output;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Clone)]
pub struct FlatCatOpts {
    /// If set, all inputs without identified format will be printed plainly like classic `cat` would do.
    cat_plain: bool,
}

impl FlatCatOpts {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_plain(self, cat_plain: bool) -> Self {
        FlatCatOpts { cat_plain }
    }
}

impl Default for FlatCatOpts {
    fn default() -> Self {
        FlatCatOpts { cat_plain: true }
    }
}

#[derive(Debug)]
pub struct FlatCat {
    opts: FlatCatOpts,
    output: OutputWriter,
}

impl FlatCat {
    pub fn new(opts: FlatCatOpts, output: Output) -> Result<FlatCat> {
        let output = output.try_into()?;
        Ok(FlatCat { opts, output })
    }

    pub fn cat(&mut self, input: Input) -> Result<()> {
        use crate::catter::Catter;

        let format = input.format();
        let mut reader: InputReader = input.try_into()?;

        match format {
            Ok(Format::Json) => {
                let mut catter = catter::JsonCatter::new(&self.opts, &mut self.output);
                catter.cat(&mut reader)
            }
            Ok(Format::Toml) => {
                let mut catter = catter::TomlCatter::new(&self.opts, &mut self.output);
                catter.cat(&mut reader)
            }
            Ok(Format::Yaml) => {
                let mut catter = catter::YamlCatter::new(&self.opts, &mut self.output);
                catter.cat(&mut reader)
            }
            Err(_) if self.opts.cat_plain => {
                let mut catter = catter::PlainCatter::new(&self.opts, &mut self.output);
                catter.cat(&mut reader)
            }
            Err(err) => Err(err),
        }
    }
}
