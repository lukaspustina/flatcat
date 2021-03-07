// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use thiserror::Error;

#[derive(Debug, Error)]
/// Main Error type of this crate.
///
/// Must be `Send` because it used by async function which might run on different threads.
pub enum Error {
    #[error("failed to parse '{what}' to {to} because {why}")]
    ParserError {
        what: String,
        to: &'static str,
        why: String,
    },
    #[error("failed to identify input format because {msg}")]
    UnknownFormatError { msg: &'static str },
    #[error("failed to identify input format of file with extension '{ext}'")]
    UnknownFormatExtError { ext: String },
    #[error("failed to execute IO operation for")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("failed to deserialize to JSON")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
    #[error("failed to deserialize to Toml")]
    TomlError {
        #[from]
        source: toml::de::Error,
    },
    #[error("failed to deserialize to Yaml")]
    YamlError {
        #[from]
        source: serde_yaml::Error,
    },
}
