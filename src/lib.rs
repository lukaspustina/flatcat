// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub use error::Error;

pub use crate::input::Input;
use crate::input::InputReader;
pub use crate::output::{Output, OutputOpts};
use std::convert::TryInto;

pub mod catter;
pub mod error;
pub mod file_extension;
pub mod input;
pub mod output;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    Json,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FlatCatOpts {}

impl FlatCatOpts {
    pub fn new() -> FlatCatOpts {
        Default::default()
    }
}

impl Default for FlatCatOpts {
    fn default() -> Self {
        FlatCatOpts {}
    }
}

#[derive(Debug)]
pub struct FlatCat {
    opts: FlatCatOpts,
    output: Output,
}

impl FlatCat {
    pub fn new(opts: FlatCatOpts, output: Output) -> FlatCat {
        FlatCat { opts, output }
    }

    pub fn cat(&self, input: Input) -> Result<()> {
        use crate::catter::Catter;

        match input.format() {
            Format::Json => {
                let opts = self.opts.clone().into();
                let catter = catter::json::JsonCatter::new(opts, &self.output);
                let mut reader: InputReader = input.try_into()?;
                catter.cat(&mut reader)
            }
            Format::Unknown => Err(Error::UnknownFormatError {}),
        }
    }
}
