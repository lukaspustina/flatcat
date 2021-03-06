// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_json::{de::from_reader, Value};

use crate::catter::{Catter, KeyPath};
use crate::output::OutputWriter;
use crate::{FlatCatOpts, Result};

#[derive(Debug)]
pub struct JsonCatter<'a> {
    #[allow(dead_code)]
    opts: &'a FlatCatOpts,
    output: &'a mut OutputWriter,
}

impl<'a> JsonCatter<'a> {
    pub fn new<'b>(opts: &'b FlatCatOpts, output: &'b mut OutputWriter) -> JsonCatter<'b> {
        JsonCatter { opts, output }
    }

    fn json(&mut self, json: Value) -> Result<()> {
        let mut path = KeyPath::new();

        self.do_json(&mut path, json)
    }

    fn do_json(&mut self, path: &mut KeyPath, json: Value) -> Result<()> {
        match json {
            Value::Null => self.output.null(&path.to_string()),
            Value::Bool(x) => self.output.bool(&path.path(), &x),
            Value::Number(x) => self.output.number(&path.path(), &x),
            Value::String(x) => self.output.string(&path.path(), &x),
            Value::Array(x) => {
                for (i, value) in x.into_iter().enumerate() {
                    let key = self.output.array(i);
                    path.push_no_sep(&key);
                    self.do_json(path, value)?;
                    path.pop();
                }
            }
            Value::Object(x) => {
                for (key, value) in x {
                    path.push(&key);
                    self.do_json(path, value)?;
                    path.pop();
                }
            }
        }

        Ok(())
    }
}

impl<'a> Catter for JsonCatter<'a> {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()> {
        let json: Value = from_reader(read)?;

        self.json(json)?;

        Ok(())
    }
}
