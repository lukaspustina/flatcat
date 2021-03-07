/*
 * Copyright 2021 Lukas Pustina <lukas@pustina.de>
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 *
 */

use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use crate::file_extension::FileExtension;
use crate::Format;

#[derive(Debug)]
pub enum Input {
    Path(PathBuf),
}

impl Input {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        Input::Path(path)
    }

    pub fn format(&self) -> Format {
        match self {
            Input::Path(p) => p
                .extension()
                .map(|ext| FileExtension::format(ext))
                .unwrap_or(Format::Unknown),
        }
    }
}

pub struct InputReader {
    inner: Box<dyn Read>,
}

impl Read for InputReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl TryFrom<Input> for InputReader {
    type Error = crate::error::Error;

    fn try_from(value: Input) -> std::result::Result<Self, Self::Error> {
        match value {
            Input::Path(p) => {
                let file = File::open(p).expect("Failed to open file");
                let buf_reader = BufReader::new(file);
                Ok(InputReader {
                    inner: Box::new(buf_reader),
                })
            }
        }
    }
}
