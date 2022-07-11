# polkadot

## Reference
[Astar](https://docs.astar.network/)
[Polkadot](https://wiki.polkadot.network/)
[Substrate](https://docs.substrate.io/quick-start/)

## Run
### Build
```
cargo build --all
```
### Build Contract
To build a contract, execute this command with the path of `Cargo.toml` in the contract folder to build.
Build `simple_counter_with_hashmap` with the cargo command and then upload and instantiate `.contract` file in `target/ink/your_contract/` on [Polkadot js Apps](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-contracts-rpc.polkadot.io#/)
```
cargo +nightly contract build --manifest-path ./simple_counter_with_hashmap/Cargo.toml
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