// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use lazy_static::lazy_static;
use std::borrow::Cow;
use std::fmt::Display;
use yansi::{Color, Style};

static EMPTY_PREFIX: &str = "";
lazy_static! {
    static ref STYLE_ARRAY: Style = Style::new(Color::Green);
    static ref STYLE_BOOL: Style = Style::new(Color::Red);
    static ref STYLE_DATETIME: Style = Style::new(Color::Green);
    static ref STYLE_NUMBER: Style = Style::new(Color::Blue);
    static ref STYLE_SPECIAL: Style = Style::new(Color::White).italic();
    static ref STYLE_STRING: Style = Style::new(Color::Yellow);
    static ref STYLE_VALUE_COUNT: Style = Style::new(Color::Yellow);
}

#[derive(Debug, Clone)]
pub struct OutputOpts {
    colorful: bool,
    quotes: bool,
    show_value_counter: bool,
}

impl Default for OutputOpts {
    fn default() -> Self {
        OutputOpts {
            colorful: true,
            quotes: true,
            show_value_counter: false,
        }
    }
}

impl OutputOpts {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_color(self, colorful: bool) -> Self {
        OutputOpts { colorful, ..self }
    }

    pub fn with_quotes(self, quotes: bool) -> Self {
        OutputOpts { quotes, ..self }
    }

    pub fn with_value_counter(self, show_value_counter: bool) -> Self {
        OutputOpts {
            show_value_counter,
            ..self
        }
    }
}

#[derive(Debug)]
pub struct Output {
    opts: OutputOpts,
    value_counter: usize,
}

impl Output {
    pub fn new(opts: OutputOpts) -> Self {
        if !opts.colorful {
            yansi::Paint::disable();
        }
        Output { opts, value_counter: 1 }
    }

    pub fn array(&self, i: usize) -> String {
        format!("{}{}{}", STYLE_ARRAY.paint("["), i, STYLE_ARRAY.paint("]"))
    }

    pub fn bool<T: Display>(&mut self, path: &str, b: T) {
        self.println(*STYLE_BOOL, path, b);
    }

    pub fn datetime<T: Display>(&mut self, path: &str, datetime: T) {
        self.println(*STYLE_DATETIME, path, datetime);
    }

    pub fn number<T: Display>(&mut self, path: &str, number: T) {
        self.println(*STYLE_NUMBER, path, number);
    }

    pub fn string<T: Display>(&mut self, path: &str, str: T) {
        if self.opts.quotes {
            self.println(*STYLE_STRING, path, format!("\"{}\"", str));
        } else {
            self.println(*STYLE_STRING, path, str);
        }
    }

    pub fn special<T: Display>(&mut self, path: &str, str: T) {
        self.println(*STYLE_SPECIAL, path, str);
    }

    pub fn reset_value_counter(&mut self) {
        self.value_counter = 0;
    }

    fn prefix(&self) -> Cow<str> {
        if !self.opts.show_value_counter {
            return Cow::Borrowed(EMPTY_PREFIX);
        }

        let prefix = if self.opts.show_value_counter {
            format!("{:>5}  ", STYLE_VALUE_COUNT.paint(self.value_counter))
        } else {
            String::new()
        };

        Cow::Owned(prefix)
    }

    fn println<T: Display>(&mut self, style: Style, path: &str, value: T) {
        let prefix = self.prefix();
        println!(
            "{prefix}{path}: {value}",
            prefix = prefix,
            path = path,
            value = style.paint(value)
        );
        self.value_counter += 1;
    }
}
