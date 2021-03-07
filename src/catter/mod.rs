// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::Read;

pub use crate::catter::toml::TomlCatter;
pub use json::JsonCatter;
pub use yaml::YamlCatter;

use crate::Result;

pub mod json;
pub mod toml;
pub mod yaml;

pub trait Catter {
    fn cat<R: Read>(&mut self, read: &mut R) -> Result<()>;
}
