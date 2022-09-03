# polkadot

## Reference

[Astar](https://docs.astar.network/) [Polkadot](https://wiki.polkadot.network/)
[Substrate](https://docs.substrate.io/quick-start/)

## Run

### Build

```
cargo build --all
```

### Format

```
cargo +nightly fmt
```

### Lint

```
cargo clippy --all --all-targets --release
```

### Test

```
TEST_CONFIG=test_config_example.json cargo test --all
```

### Create new contract

```
cargo contract new [contract name]
```

### Build contract

To build a contract, execute this command with the path of `Cargo.toml` in the
contract folder to build. Build `simple_counter` with the cargo command and then
upload and instantiate `.contract` file in `target/ink/your_contract/` on
[Polkadot js Apps](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-contracts-rpc.polkadot.io#/)

```
cargo +nightly contract build --manifest-path ./simple_counter/Cargo.toml
```

### Test specific contract

To test specific contract, execute `cargo +nightly contract test` with
`--manifest-path` with the path of `Cargo.toml`.
<br/> e.g. Test `simple_counter` with this command.

```
cargo +nightly contract test --manifest-path ./simple_counter/Cargo.toml
```
