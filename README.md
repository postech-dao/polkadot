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
