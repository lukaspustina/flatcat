// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use lazy_static::lazy_static;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::{fmt, io};
use yansi::{Color, Style};

static EMPTY_STR: &str = "";
lazy_static! {
    static ref STYLE_ARRAY: Style = Style::new(Color::Green);
    static ref STYLE_BOOL: Style = Style::new(Color::Red);
    static ref STYLE_DATETIME: Style = Style::new(Color::Green);
    static ref STYLE_NUMBER: Style = Style::new(Color::Blue);
    static ref STYLE_PLAIN: Style = Default::default();
    static ref STYLE_SPECIAL: Style = Style::new(Color::White).italic();
    static ref STYLE_STRING: Style = Style::new(Color::Yellow);
    static ref STYLE_VALUE_COUNT: Style = Style::new(Color::Yellow);
}

#[derive(Debug, Clone)]
pub struct OutputOpts {
    colorful: bool,
    null: bool,
    quotes: bool,
    numbers: bool,
    end_of_line: bool,
}

impl Default for OutputOpts {
    fn default() -> Self {
        OutputOpts {
            colorful: true,
            null: true,
            quotes: true,
            numbers: false,
            end_of_line: false,
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

    pub fn with_null(self, null: bool) -> Self {
        OutputOpts { null, ..self }
    }

    pub fn with_numbers(self, numbers: bool) -> Self {
        OutputOpts { numbers, ..self }
    }

    pub fn with_end_of_lines(self, end_of_line: bool) -> Self {
        OutputOpts { end_of_line, ..self }
    }
}

pub enum Output {
    Write(Box<dyn Write>, OutputOpts),
    StdOut(OutputOpts),
}

impl Output {
    pub fn from_writer<W: Write + 'static>(writer: W, opts: OutputOpts) -> Self {
        Output::Write(Box::new(writer), opts)
    }

    pub fn from_stdout(opts: OutputOpts) -> Self {
        Output::StdOut(opts)
    }
}

impl Debug for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Output::Write(_, opts) => f.write_fmt(format_args!("Output::Write(opts={:?})", opts)),
            Output::StdOut(opts) => f.write_fmt(format_args!("Output::StdOut(opts={:?})", opts)),
        }
    }
}

pub struct OutputWriter {
    inner: Box<dyn Write>,
    opts: OutputOpts,
    value_counter: usize,
}

impl OutputWriter {
    pub fn from_writer(inner: Box<dyn Write>, opts: OutputOpts) -> Self {
        if !opts.colorful {
            yansi::Paint::disable();
        }
        OutputWriter {
            inner,
            opts,
            value_counter: 1,
        }
    }

    pub fn array(&self, i: usize) -> String {
        format!("{}{}{}", STYLE_ARRAY.paint("["), i, STYLE_ARRAY.paint("]"))
    }

    pub fn bool<T: Display>(&mut self, path: &str, b: T) {
        self.writeln(*STYLE_BOOL, path, b);
    }

    pub fn datetime<T: Display>(&mut self, path: &str, datetime: T) {
        self.writeln(*STYLE_DATETIME, path, datetime);
    }

    pub fn number<T: Display>(&mut self, path: &str, number: T) {
        self.writeln(*STYLE_NUMBER, path, number);
    }

    pub fn string<T: Display>(&mut self, path: &str, str: T) {
        if self.opts.quotes {
            self.writeln(*STYLE_STRING, path, format!("\"{}\"", str));
        } else {
            self.writeln(*STYLE_STRING, path, str);
        }
    }

    pub fn null(&mut self, path: &str) {
        if self.opts.null {
            self.special(path, "null")
        }
    }

    pub fn special<T: Display>(&mut self, path: &str, str: T) {
        self.writeln(*STYLE_SPECIAL, path, str);
    }

    pub fn plain<T: Display>(&mut self, str: T) {
        let prefix = prefix(self.opts.numbers, self.value_counter);
        let suffix = suffix(self.opts.end_of_line);
        let _ = self.inner.write_fmt(format_args!(
            "{prefix}{str}{suffix}\n",
            prefix = prefix,
            str = STYLE_PLAIN.paint(str),
            suffix = suffix,
        ));
        self.value_counter += 1;
    }

    pub fn reset_value_counter(&mut self) {
        self.value_counter = 0;
    }

    fn writeln<T: Display>(&mut self, style: Style, path: &str, value: T) {
        let prefix = prefix(self.opts.numbers, self.value_counter);
        let suffix = suffix(self.opts.end_of_line);
        let _ = self.inner.write_fmt(format_args!(
            "{prefix}{path}: {value}{suffix}\n",
            prefix = prefix,
            path = path,
            value = style.paint(value),
            suffix = suffix,
        ));
        self.value_counter += 1;
    }
}

fn prefix(numbers: bool, value_counter: usize) -> Cow<'static, str> {
    if !numbers {
        return Cow::Borrowed(EMPTY_STR);
    }

    let prefix = if numbers {
        format!("{:>5}  ", STYLE_VALUE_COUNT.paint(value_counter))
    } else {
        String::new()
    };

    Cow::Owned(prefix)
}

fn suffix(end_of_line: bool) -> Cow<'static, str> {
    if end_of_line {
        Cow::Borrowed("$")
    } else {
        Cow::Borrowed(EMPTY_STR)
    }
}

impl Debug for OutputWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "OutputWriter(opts={:?}, value_counter={:?})",
            self.opts, self.value_counter
        ))
    }
}

impl TryFrom<Output> for OutputWriter {
    type Error = crate::error::Error;

    fn try_from(value: Output) -> std::result::Result<Self, Self::Error> {
        match value {
            Output::Write(inner, opts) => Ok(OutputWriter::from_writer(inner, opts)),
            Output::StdOut(opts) => Ok(OutputWriter::from_writer(Box::new(io::stdout()), opts)),
        }
    }
}
