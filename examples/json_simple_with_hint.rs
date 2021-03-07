// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use flatcat::{FlatCat, FormatHint, Input, Output, OutputOpts};

fn main() {
    let input = Input::from_path("tests/files/simple.json").with_format_hint(FormatHint::json());

    let opts = OutputOpts::new().with_color(true);
    let output = Output::new(opts);

    let opts = Default::default();
    let mut flat_cat = FlatCat::new(opts, output);

    flat_cat.cat(input).expect("Failed to cat file");
}
