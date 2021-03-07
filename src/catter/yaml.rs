// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_yaml::{from_reader, Value};

use crate::catter::Catter;
use crate::output::Output;
use crate::{FlatCatOpts, Result};

#[derive(Debug, Clone)]
pub struct YamlCatterOpts {}

impl YamlCatterOpts {
    pub fn new() -> YamlCatterOpts {
        YamlCatterOpts {}
    }
}

impl Default for YamlCatterOpts {
    fn default() -> Self {
        YamlCatterOpts {}
    }
}

impl From<FlatCatOpts> for YamlCatterOpts {
    fn from(_: FlatCatOpts) -> Self {
        YamlCatterOpts {}
    }
}

#[derive(Debug)]
pub struct YamlCatter<'a> {
    #[allow(dead_code)]
    opts: YamlCatterOpts,
    output: &'a mut Output,
}

impl<'a> YamlCatter<'a> {
    pub fn new(opts: YamlCatterOpts, output: &mut Output) -> YamlCatter {
        YamlCatter { opts, output }
    }

    fn yaml(&mut self, yaml: Value) -> Result<()> {
        let mut path = String::new();

        self.do_yaml(&mut path, yaml)
    }

    fn do_yaml(&mut self, path: &mut String, yaml: Value) -> Result<()> {
        match yaml {
            Value::Null => self.output.special(&path, "null"),
            Value::Bool(x) => self.output.bool(&path, &x),
            Value::Number(x) => self.output.number(&path, &x),
            Value::String(x) => self.output.string(&path, &x),
            Value::Sequence(x) => {
                for (i, value) in x.into_iter().enumerate() {
                    let key = self.output.array(i);
                    path.push_str(&key);
                    self.do_yaml(path, value)?;
                    path.truncate(path.len() - key.len());
                }
            }
            Value::Mapping(x) => {
                for (key, value) in x {
                    // TODO: value is not a string?!
                    let key = format!(".{}", key.as_str().unwrap());
                    path.push_str(&key);
                    self.do_yaml(path, value)?;
                    path.truncate(path.len() - key.len());
                }
            }
        }

        Ok(())
    }
}

impl<'a> Catter for YamlCatter<'a> {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()> {
        let yaml: Value = from_reader(read)?;

        self.yaml(yaml)?;

        Ok(())
    }
}
