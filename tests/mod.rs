/*
integration tests

```BASH
# normal testing
cargo test

# generate code coverage report (command line)
cargo install cargo-llvm-cov
cargo llvm-cov

# generate code coverage report (IDE extensions)
cargo install cargo-nextest
cargo llvm-cov nextest --lcov --output-path ./target/lcov.info
```
*/

mod emc2101;
mod emc2101_hw;
mod ht16k33;
mod ht16k33_hw;
