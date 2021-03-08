// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_yaml::{from_reader, Value};

use crate::catter::{Catter, KeyPath};
use crate::output::OutputWriter;
use crate::{FlatCatOpts, Result};

#[derive(Debug)]
pub struct YamlCatter<'a> {
    #[allow(dead_code)]
    opts: &'a FlatCatOpts,
    output: &'a mut OutputWriter,
}

impl<'a> YamlCatter<'a> {
    pub fn new<'b>(opts: &'b FlatCatOpts, output: &'b mut OutputWriter) -> YamlCatter<'b> {
        YamlCatter { opts, output }
    }

    fn yaml(&mut self, yaml: Value) -> Result<()> {
        let mut path = KeyPath::new();

        self.do_yaml(&mut path, yaml)
    }

    fn do_yaml(&mut self, path: &mut KeyPath, yaml: Value) -> Result<()> {
        match yaml {
            Value::Null => self.output.null(&path.path()),
            Value::Bool(x) => self.output.bool(&path.path(), &x),
            Value::Number(x) => self.output.number(&path.path(), &x),
            Value::String(x) => self.output.string(&path.path(), &x),
            Value::Sequence(x) => {
                for (i, value) in x.into_iter().enumerate() {
                    let key = self.output.array(i);
                    path.push_no_sep(&key);
                    self.do_yaml(path, value)?;
                    path.pop()
                }
            }
            Value::Mapping(x) => {
                for (key, value) in x {
                    path.push(&key.as_str().unwrap());
                    self.do_yaml(path, value)?;
                    path.pop();
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
