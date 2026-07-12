#![no_main]

use feed_rs::parser::parse;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = parse(data);
});
