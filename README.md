An example [fuzz target](./fuzz/fuzz_targets/can_panic.rs).

More information about fuzz testing Rust projects can be found in the [rust-fuzz book](https://rust-fuzz.github.io/book/introduction.html).

## Running the fuzzer

- Install [Rust](https://rustup.rs/) and add a nightly toolchain.
- Install [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

Run

```sh
cargo fuzz run can_panic
```

On my machine I got bored of waiting before it found the panic. Adding an input corpus helps:

```sh
mkdir -p fuzz/corpus/can_panic
cargo +nightly fuzz run can_panic fuzz/corpus/can_panic fuzz/fuzz_targets/can_panic/inputs
```

Notice that I passed two corpus directories. The first one will be used to store intermediate results by the fuzzer. I did not want to add those to the repository.

## Coverage

Coverage information can be obtained for a fixed set of inputs with:

```sh
cargo +nightly fuzz coverage can_panic fuzz/corpus/can_panic fuzz/fuzz_targets/can_panic/inputs
```
