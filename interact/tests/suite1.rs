use serde::{Deserialize, Serialize};

use sp_keyring::AccountKeyring;
use subxt::{
    rpc::Subscription,
    sp_runtime::{generic::Header, traits::BlakeTwo256},
    ClientBuilder, DefaultConfig, PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../../../metadata.scale")]
pub mod polkadot {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    full_node_url: String,
    account_public: String,
    account_private: String,
    wasm_binary_path: String,
    // and so on
}

impl Config {
    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string(
                std::env::var("TEST_CONFIG")
                    .expect("Environment variable for the config file path is missing"),
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

#[tokio::test]
async fn check_connection() {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .set_url("wss://rococo-contracts-rpc.polkadot.io:443")
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let _block_hash = api.client.rpc().block_hash(None).await.unwrap();
}

#[tokio::test]
async fn check_block_number() {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .set_url("wss://rococo-contracts-rpc.polkadot.io:443")
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let mut blocks: Subscription<Header<u32, BlakeTwo256>> =
        api.client.rpc().subscribe_finalized_blocks().await.unwrap();

    let mut first_block_number: u32 = 0;
    let mut second_block_number: u32 = 0;
    let mut count = 0;

    while count != 2 {
        let block = if let Some(Ok(block)) = blocks.next().await {
            block
        } else {
            todo!()
        };
        if first_block_number == 0_u32 {
            first_block_number = block.number;
        } else {
            second_block_number = block.number;
        }
        count += 1;
    }
    assert!(second_block_number > first_block_number);
}

#[tokio::test]
async fn check_account() {
    tracing_subscriber::fmt::init();

    let alice = AccountKeyring::Alice.to_account_id();

    let api = ClientBuilder::new()
        .set_url("wss://rococo-contracts-rpc.polkadot.io:443")
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let account = api.storage().system().account(&alice, None).await.unwrap();

    assert!(account.data.free == account.data.fee_frozen); //Don't have any balances to pay gas fee
}

#[tokio::test]
#[ignore]
async fn upload_modify_and_query() {
    let _config = Config::read_from_env();
    // upload the contract, submit a transaction that modifies its state, and query the state
    unimplemented!();
}
