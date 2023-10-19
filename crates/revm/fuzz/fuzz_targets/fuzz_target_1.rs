#![no_main]

use libfuzzer_sys::fuzz_target;
use revm::{test_harness, DummyContract};


fuzz_target!(|data: DummyContract| {
    // fuzzed code goes here
    test_harness(data);
});
