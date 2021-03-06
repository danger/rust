# Danger in Rust

[![Build Status](https://travis-ci.org/danger/rust/rust.svg?branch=master)](https://travis-ci.org/danger/rust)

This is the minimal version of running Danger in Rust. It is a cargo module which expects the [JSON DSL from
Danger JS](https://danger.systems/js/usage/danger-process.html) and provides a type-safe way to interact with it 
from inside the app.

![screenshots/wip1.png](screenshots/wip1.png)

## Next Steps

- Figure out the distribution method to get it running on other people's CI
- Compiling a Rust dangerfile `danger.rs` and injecting that with the additional runtime work (passing data in/out)
- Creating commands for `ci`, `pr`, `local`
- Error handling

I'm learning everything from scratch, so, some of these may take quite a while.

# Contributing

### Building Danger

```sh
cargo run --bin danger-rust
```

### Running Danger in Dev

This is how you can test out how things work:

```sh
# Pipe the JSON in and potentially compile + exec
cat fixtures/danger-js-697.json | cargo run --bin danger-rust

# Or, build then compile

# build the binary
cargo build --bin danger-rust
# Run it and pipe in the JSON to STDIN
cat fixtures/danger-js-697.json | ./target/debug/danger-rust
```

### Scripts

Update the Rust Types from the JSON schema of Danger's DSL

```sh
cargo run --bin update_types
```

Grab a Danger JS incoming JSON example

```sh
# uses danger-js the `--dangerfile xxyy` is a bug
danger pr https://github.com/danger/danger-js/pull/697 --json > fixtures/danger-js-697.json --dangerfile LICENSE
```
