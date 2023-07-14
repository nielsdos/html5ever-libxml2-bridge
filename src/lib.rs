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
pub mod error_container;
pub mod parse_result;

extern crate html5ever;

use std::io::Cursor;

use crate::sink::Sink;
use html5ever::interface::QuirksMode;
use html5ever::tendril::TendrilSink;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};
use crate::error_container::CError;
use crate::parse_result::ParseResult;

fn parse_from_bytes(bytes: &[u8]) -> Sink {
    let sink = Sink::new();
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
    .read_from(&mut Cursor::new(bytes))
    .unwrap() // TODO?
}

#[no_mangle]
pub unsafe extern "C" fn html5ever_libxml2_bridge_parse_from_bytes(bytes: *const libc::__u8, len: libc::size_t) -> *mut ParseResult {
    let bytes = std::slice::from_raw_parts(bytes.into(), len.into());
    let sink = parse_from_bytes(bytes);
    let parse_result = Box::new(sink.into_parse_result());
    Box::into_raw(parse_result).into()
}

#[no_mangle]
pub unsafe extern "C" fn html5ever_libxml2_bridge_get_doc(parse_result: *const ParseResult) -> *mut libc::c_void {
    (*parse_result).doc.as_raw()
}

#[no_mangle]
pub unsafe extern "C" fn html5ever_libxml2_bridge_count_errors(parse_result: *const ParseResult) -> libc::size_t {
    (*parse_result).error_container.count().into()
}

#[no_mangle]
pub unsafe extern "C" fn html5ever_libxml2_bridge_get_error(parse_result: *const ParseResult, index: libc::size_t) -> CError {
    (*parse_result).error_container.get(index).unwrap().as_c_error()
}

#[no_mangle]
pub unsafe extern "C" fn html5ever_libxml2_bridge_destroy_parse_result(parse_result: *mut ParseResult) {
    drop(Box::from_raw(parse_result));
}
