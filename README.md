An example [fuzz target](./fuzz/fuzz_targets/fuzz_target_1.rs).

More information about fuzz testing Rust projects can be found in the [rust-fuzz book](https://rust-fuzz.github.io/book/introduction.html).

## Running the fuzzer

Install [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and a nightly Rust toolchain.

Run

```sh
cargo fuzz run fuzz_target_1
```

On my machine I got bored of waiting before it found the panic. Adding an input corpus helps:

```sh
mkdir -p fuzz/corpus/fuzz_target_1
cargo +nightly fuzz run fuzz_target_1 fuzz/corpus/fuzz_target_1 fuzz/fuzz_targets/fuzz_target_1/inputs
```

Notice that I passed two corpus directories. The first one will be used to store intermediate results by the fuzzer. I did not want to add those to the repository.

## Coverage

Coverage information can be obtained for a fixed set of inputs with:

```sh
cargo +nightly fuzz coverage fuzz_target_1 fuzz/corpus/fuzz_target_1 fuzz/fuzz_targets/fuzz_target_1/inputs
```
