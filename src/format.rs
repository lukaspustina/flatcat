/*
 * Copyright 2021 Lukas Pustina <lukas@pustina.de>
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 *
 */

use crate::file_extension::FileExtension;
use crate::{Error, Result};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub enum FormatHint {
    Hint(Format),
    Empty,
}

impl FormatHint {
    pub fn json() -> Self {
        FormatHint::Hint(Format::Json)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    Json,
    Yaml,
}

impl Format {
    pub fn guess_from_file_extension<P: AsRef<Path>>(path: P) -> Result<Format> {
        let p = path.as_ref();
        let _ = p.file_name().ok_or(Error::UnknownFormatError { msg: "not a file" })?;
        let res = p
            .extension()
            .ok_or(Error::UnknownFormatError {
                msg: "no file extension",
            })
            .map(FileExtension::guess_format);
        res?
    }
}
