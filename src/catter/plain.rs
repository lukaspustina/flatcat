// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{BufRead, BufReader, Read};

use crate::catter::Catter;
use crate::output::OutputWriter;
use crate::{FlatCatOpts, Result};

#[derive(Debug)]
pub struct PlainCatter<'a> {
    #[allow(dead_code)]
    opts: &'a FlatCatOpts,
    output: &'a mut OutputWriter,
}

impl<'a> PlainCatter<'a> {
    pub fn new<'b>(opts: &'b FlatCatOpts, output: &'b mut OutputWriter) -> PlainCatter<'b> {
        PlainCatter { opts, output }
    }

    fn plain<R: Read>(&mut self, read: R) -> Result<()> {
        let buf_reader = BufReader::new(read);
        for line in buf_reader.lines() {
            let line = line?;
            self.output.plain(line);
        }

        Ok(())
    }
}

impl<'a> Catter for PlainCatter<'a> {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()> {
        self.plain(read)?;
        Ok(())
    }
}
