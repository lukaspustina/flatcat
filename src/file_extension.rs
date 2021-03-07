/*
 * Copyright 2021 Lukas Pustina <lukas@pustina.de>
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 *
 */

use std::ffi::OsStr;

use crate::{Error, Format, Result};

pub struct FileExtension {}

impl FileExtension {
    pub fn guess_format(ext: &OsStr) -> Result<Format> {
        let str = ext.to_string_lossy();
        // see https://github.com/BurntSushi/ripgrep/blob/9c8d873a75ccb2a8d3ed692148becb2e72514732/crates/ignore/src/default_types.rs
        match str.as_ref() {
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => Err(Error::UnknownFormatExtError { ext: str.into_owned() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn json() {
        let ext = OsStr::new("json");

        let format = FileExtension::guess_format(ext);

        asserting("json extension")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Json);
    }

    #[test]
    fn yaml() {
        let ext = OsStr::new("yaml");

        let format = FileExtension::guess_format(ext);

        asserting("yaml extension")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Yaml);
    }

    #[test]
    fn yml() {
        let ext = OsStr::new("yml");

        let format = FileExtension::guess_format(ext);

        asserting("yml extension")
            .that(&format)
            .is_ok()
            .is_equal_to(&Format::Yaml);
    }
}
