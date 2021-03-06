// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use crate::output::Output;
pub use error::Error;

pub mod catter;
pub mod error;
pub mod output;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Copy, Clone)]
pub enum Format {
    Json,
}

#[derive(Debug, Clone)]
pub struct FlatCatOpts {
    format: Format,
}

impl FlatCatOpts {
    pub fn new(format: Format) -> FlatCatOpts {
        FlatCatOpts { format }
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

    pub fn cat<R: Read>(&self, read: &mut R) -> Result<()> {
        use crate::catter::Catter;

        match self.opts.format {
            Format::Json => {
                let opts = self.opts.clone().into();
                let catter = catter::json::JsonCatter::new(opts, &self.output);
                catter.cat(read)
            }
        }
    }
}
