// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

use serde_json::{de::from_reader, Value};

use crate::catter::Catter;
use crate::output::Output;
use crate::{FlatCatOpts, Result};

#[derive(Debug, Clone)]
pub struct JsonCatterOpts {}

impl JsonCatterOpts {
    pub fn new() -> JsonCatterOpts {
        JsonCatterOpts {}
    }
}

impl Default for JsonCatterOpts {
    fn default() -> Self {
        JsonCatterOpts {}
    }
}

impl From<FlatCatOpts> for JsonCatterOpts {
    fn from(_: FlatCatOpts) -> Self {
        JsonCatterOpts {}
    }
}

#[derive(Debug)]
pub struct JsonCatter<'a> {
    #[allow(dead_code)]
    opts: JsonCatterOpts,
    output: &'a mut Output,
}

impl<'a> JsonCatter<'a> {
    pub fn new(opts: JsonCatterOpts, output: &mut Output) -> JsonCatter {
        JsonCatter { opts, output }
    }

    fn json(&mut self, json: Value) -> Result<()> {
        let mut path = String::new();

        self.do_json(&mut path, json)
    }

    fn do_json(&mut self, path: &mut String, json: Value) -> Result<()> {
        match json {
            Value::Null => self.output.null(&path),
            Value::Bool(x) => self.output.bool(&path, &x),
            Value::Number(x) => self.output.number(&path, &x),
            Value::String(x) => self.output.string(&path, &x),
            Value::Array(x) => {
                for (i, value) in x.into_iter().enumerate() {
                    let key = self.output.array(i);
                    path.push_str(&key);
                    self.do_json(path, value)?;
                    path.truncate(path.len() - key.len());
                }
            }
            Value::Object(x) => {
                for (key, value) in x {
                    let key = format!(".{}", key);
                    path.push_str(&key);
                    self.do_json(path, value)?;
                    path.truncate(path.len() - key.len());
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
