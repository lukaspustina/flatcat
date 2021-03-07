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
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use crate::{Error, Format, FormatHint, Result};

pub enum Input {
    Path(PathBuf, FormatHint),
    Read(Box<dyn Read>, FormatHint),
}

impl Input {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        Input::Path(path, FormatHint::Empty)
    }

    pub fn from_read<R: Read + 'static>(reader: R) -> Input {
        Input::Read(Box::new(reader), FormatHint::Empty)
    }

    pub fn with_format_hint(self, hint: FormatHint) -> Self {
        match self {
            Input::Path(path, _) => Input::Path(path, hint),
            Input::Read(reader, _) => Input::Read(reader, hint),
        }
    }

    pub fn format(&self) -> Result<Format> {
        match self {
            Input::Path(_, FormatHint::Hint(format)) => Ok(*format),
            Input::Path(p, FormatHint::Empty) => Format::guess_from_file_extension(p),
            Input::Read(_, FormatHint::Hint(format)) => Ok(*format),
            Input::Read(_, FormatHint::Empty) => Err(Error::UnknownFormatError {
                msg: "cannot determine format from stream",
            }),
        }
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Input::Path(path, hint) => f.write_fmt(format_args!("Input::Path(path={:?}, hint={:?})", path, hint)),
            Input::Read(_, hint) => f.write_fmt(format_args!("Input::Read(hint={:?})", hint)),
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
            Input::Path(p, _) => {
                let file = File::open(p)?;
                let buf_reader = BufReader::new(file);
                Ok(InputReader {
                    inner: Box::new(buf_reader),
                })
            }
            Input::Read(inner, _) => Ok(InputReader { inner }),
        }
    }
}
