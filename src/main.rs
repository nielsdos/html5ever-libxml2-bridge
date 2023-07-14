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

use std::io;

use crate::sink::Sink;
use html5ever::interface::QuirksMode;
use html5ever::tendril::TendrilSink;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};

fn main() {
    let sink = Sink::new();
    let stdin = io::stdin();
    parse_document(
        sink,
        ParseOpts {
            tokenizer: TokenizerOpts {
                exact_errors: true,
                discard_bom: false,
                profile: false,
                initial_state: None,
                last_start_tag_name: None,
            },
            tree_builder: TreeBuilderOpts {
                exact_errors: true,
                scripting_enabled: false,
                iframe_srcdoc: false,
                drop_doctype: false,
                ignore_missing_rules: false,
                quirks_mode: QuirksMode::NoQuirks,
            },
        },
    )
    .from_utf8()
    .read_from(&mut stdin.lock())
    .unwrap();
}
