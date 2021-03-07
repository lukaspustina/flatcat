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
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum FormatHint {
    Hint(Format),
    Empty,
}

impl FormatHint {
    pub fn json() -> Self {
        FormatHint::Hint(Format::Json)
    }

    pub fn toml() -> Self {
        FormatHint::Hint(Format::Toml)
    }

    pub fn yaml() -> Self {
        FormatHint::Hint(Format::Yaml)
    }

    pub fn hint(format: Format) -> Self {
        FormatHint::Hint(format)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    Json,
    Toml,
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

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(Error::ParserError {
                what: s.to_string(),
                to: "Format",
                why: "unknown format".to_string(),
            }),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        };
        f.write_str(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use spectral::prelude::*;

    #[test]
    fn json_from_str() {
        let format = Format::from_str(&Format::Json.to_string());

        asserting("json is parsed successfully")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Json);
    }

    #[test]
    fn toml_from_str() {
        let format = Format::from_str(&Format::Toml.to_string());

        asserting("toml is parsed successfully")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Toml);
    }

    #[test]
    fn yaml_from_str() {
        let format = Format::from_str(&Format::Yaml.to_string());

        asserting("yaml is parsed successfully")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Yaml);
    }
}
