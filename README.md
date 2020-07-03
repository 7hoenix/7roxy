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

## Client Usage

To listen:
```bash
cargo run -- --local-address "127.0.0.1:19324" --remote-address "127.0.0.1:19325"
```

To send:
```bash
cargo run -- -i --local-address "127.0.0.1:19325" --remote-address "127.0.0.1:19324"
```
