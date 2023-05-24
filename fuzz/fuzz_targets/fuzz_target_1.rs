#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // If this function panics the fuzzing will report it.
    let maybe_len = cargo_fuzz_example::do_a_thing(data);

    // Custom invariants we want to check.
    if let Some(len) = maybe_len {
        assert_eq!(data.len(), len, "When a length is returned it should match the input length.")
    }
});
