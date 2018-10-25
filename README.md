# Danger in Rust

WIP

### Build Danger


```sh
cargo run --bin danger-rust
```

### Running Danger in Dev

This is how you can test out how things work

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
