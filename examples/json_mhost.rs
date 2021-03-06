// Copyright 2021 Lukas Pustina <lukas@pustina.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use flatcat::output::{Output, OutputOpts};
use flatcat::{FlatCat, FlatCatOpts, Format};
use std::fs::File;
use std::io::BufReader;

fn main() {
    //let file = File::open("tests/files/mhost.lookup.pustina_de.json")
    let file = File::open("tests/files/simple.json").expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let opts = OutputOpts::new().with_color(true);
    let output = Output::new(opts);

    let opts = FlatCatOpts::new(Format::Json);
    let flat_cat = FlatCat::new(opts, output);

    flat_cat.cat(&mut buf_reader).expect("Failed to cat file");
}
