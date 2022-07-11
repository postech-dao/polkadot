use runtime_api;

type Balance = u128;
type CodeHash = <DefaultConfig as Config>::Hash;
type PairSigner = subxt::PairSigner<DefaultConfig, sp_core::sr25519::Pair>;
type RuntimeApi = runtime_api::api::RuntimeApi<DefaultConfig, SignedExtra>;

use anyhow::{
    Context,
    Result,
};
use jsonrpsee::{
    core::client::ClientT,
    rpc_params,
    ws_client::WsClientBuilder,
};
use serde::Serialize;
use sp_core::Bytes;
use std::{
    fmt::Debug,
    path::PathBuf,
    result,
};
use subxt::{
    rpc::NumberOrHex,
    ClientBuilder,
    Config,
    DefaultConfig,
};

fn main() {
    println!("hi");
}