/*
 * Copyright 2021 Lukas Pustina <lukas@pustina.de>
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 *
 */

use crate::Format;
use std::ffi::OsStr;

pub struct FileExtension {}

impl FileExtension {
    pub fn format(ext: &OsStr) -> Format {
        match ext.to_str() {
            Some(ext) => str_ext_to_format(ext),
            _ => Format::Unknown,
        }
    }
}

fn str_ext_to_format(ext: &str) -> Format {
    match ext {
        "json" => Format::Json,
        _ => Format::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use spectral::prelude::*;

    #[test]
    fn json() {
        let ext = OsStr::new("json");

        let format = FileExtension::format(ext);

        asserting("json extension").that(&format).is_equal_to(&Format::Json);
    }
}
