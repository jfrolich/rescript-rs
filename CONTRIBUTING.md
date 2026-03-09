# Contributing
If you are unsure what to work on or want to discuss your idea, feel free to open an issue.

### Documentation
After implementing a new feature, please document it in the doc comment on `TS` in `rescript-rs/src/lib.rs`.

### Tests
Please remember to write tests - If you are fixing a bug, write a test first to reproduce it.

### Building
There is nothing special going on here - just run `cargo build`.
To run the test suite, just run `cargo test` in the root directory.

### Formatting
To ensure proper formatting, please make sure you have the nightly toolchain installed.
After that, in the project's root directory, create a file called `.git/hooks/pre-commit` without a file extension and add the following two lines:
```sh
#!/bin/sh
cargo +nightly fmt
```

This will make sure your files are formatted before your commit is sent, so you don't have to manually run `cargo +nightly fmt`
