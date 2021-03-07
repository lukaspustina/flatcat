// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use toml::{from_slice, Value};

use crate::catter::Catter;
use crate::output::OutputWriter;
use crate::{FlatCatOpts, Result};

#[derive(Debug, Clone)]
pub struct TomlCatterOpts {}

impl TomlCatterOpts {
    pub fn new() -> TomlCatterOpts {
        TomlCatterOpts {}
    }
}

impl Default for TomlCatterOpts {
    fn default() -> Self {
        TomlCatterOpts {}
    }
}

impl From<FlatCatOpts> for TomlCatterOpts {
    fn from(_: FlatCatOpts) -> Self {
        TomlCatterOpts {}
    }
}

#[derive(Debug)]
pub struct TomlCatter<'a> {
    #[allow(dead_code)]
    opts: TomlCatterOpts,
    output: &'a mut OutputWriter,
}

impl<'a> TomlCatter<'a> {
    pub fn new(opts: TomlCatterOpts, output: &mut OutputWriter) -> TomlCatter {
        TomlCatter { opts, output }
    }

    fn toml(&mut self, toml: Value) -> Result<()> {
        let mut path = String::new();

        self.do_toml(&mut path, toml)
    }

    fn do_toml(&mut self, path: &mut String, toml: Value) -> Result<()> {
        match toml {
            Value::Boolean(x) => self.output.bool(&path, &x),
            Value::Integer(x) => self.output.number(&path, &x),
            Value::Float(x) => self.output.number(&path, &x),
            Value::String(x) => self.output.string(&path, &x),
            Value::Datetime(x) => self.output.datetime(&path, &x),
            Value::Array(x) => {
                for (i, value) in x.into_iter().enumerate() {
                    let key = self.output.array(i);
                    path.push_str(&key);
                    self.do_toml(path, value)?;
                    path.truncate(path.len() - key.len());
                }
            }
            Value::Table(x) => {
                for (key, value) in x {
                    let key = format!(".{}", key);
                    path.push_str(&key);
                    self.do_toml(path, value)?;
                    path.truncate(path.len() - key.len());
                }
            }
        }

        Ok(())
    }
}

impl<'a> Catter for TomlCatter<'a> {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()> {
        let mut buffer = Vec::new();
        read.read_to_end(buffer.as_mut())?;
        let toml: Value = from_slice(&buffer)?;

        self.toml(toml)?;

        Ok(())
    }
}
