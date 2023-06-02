#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // NOTE(mickvangelderen): The non-determinism multi-threading can cause the deadlock to occur or
    // not, the input data is irrelevant. Obviously non-determinism is bad when fuzzing. I just want to
    // know how the fuzzer deals with deadlocks should they occur.
    let _ = data;
    cargo_fuzz_example::can_deadlock();
});
