#![no_main]

use libfuzzer_sys::fuzz_target;
use feed_rs::parser::parse;

fuzz_target!(|data: &[u8]| {
    let _ = parse(data);
});