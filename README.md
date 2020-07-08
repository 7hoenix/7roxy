## Read the [Documentation](./Documentation.md)

## Local Setup

```bash
cargo install cargo-edit
```

Then you may just
```bash
cargo add <dependency>
```
cargo doc --all --open

```bash
cargo install cargo-watch
```

## Daemon Usage

To listen:
```bash
cargo run --bin daemon -- --local-address "127.0.0.1:19324"
// OR
./bin/7daemon
```

## Client Usage

To send:
```bash
cargo run --bin client -- --daemon-address "127.0.0.1:19324"
--set-directive "some directive"
// OR
./bin/7clieNt
```
