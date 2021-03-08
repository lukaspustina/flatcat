// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

pub use crate::catter::toml::TomlCatter;
pub use json::JsonCatter;
pub use plain::PlainCatter;
pub use yaml::YamlCatter;

use crate::Result;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod json;
pub mod plain;
pub mod toml;
pub mod yaml;

static KEY_SEPARATOR: char = '.';

pub trait Catter {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct KeyPath {
    path: String,
    elements: Vec<usize>,
}

impl KeyPath {
    pub fn new() -> KeyPath {
        KeyPath::new_with_capacity(1024)
    }

    pub fn new_with_capacity(capacity: usize) -> KeyPath {
        let path = String::with_capacity(capacity);
        KeyPath {
            path,
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, key: &str) {
        self.path.push(KEY_SEPARATOR);
        self.path.push_str(key);
        self.elements.push(key.len() + 1)
    }

    /// Push the key without prefixing with the separator
    pub fn push_no_sep(&mut self, key: &str) {
        self.path.push_str(key);
        self.elements.push(key.len())
    }

    pub fn pop(&mut self) {
        if let Some(key_len) = self.elements.pop() {
            self.path.truncate(self.path.len() - key_len);
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl Display for KeyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use spectral::prelude::*;

    mod path {
        use super::*;

        #[test]
        fn new() {
            let path = KeyPath::new();

            asserting("empty path").that(&path.to_string().as_str()).is_equal_to("");
        }

        #[test]
        fn push() {
            let mut path = KeyPath::new();

            path.push("next_level");

            asserting("push 1 level")
                .that(&path.to_string().as_str())
                .is_equal_to(".next_level");
        }

        #[test]
        fn push_no_sep() {
            let mut path = KeyPath::new();

            path.push_no_sep("[0]");

            asserting("push 1 level omitting the separator")
                .that(&path.to_string().as_str())
                .is_equal_to("[0]");
        }

        #[test]
        fn pop() {
            let mut path = KeyPath::new();

            path.push("next_level");
            path.pop();

            asserting("push and pop results in empty path")
                .that(&path.to_string().as_str())
                .is_equal_to("");
        }

        #[test]
        fn too_many_pop_are_okay() {
            let mut path = KeyPath::new();

            path.push("next_level");
            path.pop();
            path.pop();

            asserting("too many pops don't panic")
                .that(&path.to_string().as_str())
                .is_equal_to("");
        }
    }
}
