/// This code is based on the sample code: https://github.com/servo/html5ever/blob/master/html5ever/examples/noop-tree-builder.rs
/// Its license is as follows:
/// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
/// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
/// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
/// option. This file may not be copied, modified, or distributed
/// except according to those terms.

pub mod handle;
pub mod libxml2;
pub mod sink;

extern crate html5ever;

use std::default::Default;

use std::io;

use crate::sink::Sink;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;

fn main() {
    let sink = Sink::new();
    let stdin = io::stdin();
    parse_document(sink, Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
}
