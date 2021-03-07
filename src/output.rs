// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt::Display;
use yansi::Color::{Blue, Green, Red, White, Yellow};

#[derive(Debug, Clone)]
pub struct OutputOpts {
    colorful: bool,
}

impl Default for OutputOpts {
    fn default() -> Self {
        OutputOpts { colorful: true }
    }
}

impl OutputOpts {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_color(self, colorful: bool) -> Self {
        OutputOpts { colorful }
    }
}

#[derive(Debug)]
pub struct Output {
    opts: OutputOpts,
}

impl Output {
    pub fn new(opts: OutputOpts) -> Self {
        if !opts.colorful {
            yansi::Paint::disable();
        }
        Output { opts }
    }

    pub fn array(&self, i: usize) -> String {
        format!("{}{}{}", Green.paint("["), i, Green.paint("]"))
    }

    pub fn bool<T: Display>(&self, path: &str, b: T) {
        println!("{}: {}", path, Red.paint(b));
    }

    pub fn datetime<T: Display>(&self, path: &str, datetime: T) {
        println!("{}: {}", path, Green.paint(datetime));
    }

    pub fn number<T: Display>(&self, path: &str, number: T) {
        println!("{}: {}", path, Blue.paint(number));
    }

    pub fn string<T: Display>(&self, path: &str, str: T) {
        println!("{}: \"{}\"", path, Yellow.paint(str));
    }

    pub fn special<T: Display>(&self, path: &str, str: T) {
        println!("{}: {}", path, White.paint(str).italic());
    }
}
