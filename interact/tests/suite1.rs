use std::str::FromStr;

use serde::{Deserialize, Serialize};

use sp_core::crypto;
use sp_keyring::AccountKeyring;
use subxt::{
    rpc::Subscription,
    sp_runtime::{generic::Header, traits::BlakeTwo256},
    ClientBuilder, DefaultConfig, PairSigner, PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    test_local_node_url: String,
    test_node_url: String,
    account_public: String,
    account_password: String,
    account_seed: String,
    // and so on
}

impl Config {
    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string("./test_config_example.json")
                .expect("Environment variable for the config file path is missing"),
        )
        .expect("Failed to parse the config")
    }
}

#[tokio::test]
async fn check_connection() {
    tracing_subscriber::fmt::init();

    let config = Config::read_from_env();

    let api = ClientBuilder::new()
        .set_url(&config.test_node_url)
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let block_hash = api.client.rpc().block_hash(None).await;

    assert!(block_hash.is_ok());
}

#[tokio::test]
async fn check_block_number() {
    let config = Config::read_from_env();

    let api = ClientBuilder::new()
        .set_url(&config.test_node_url)
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let mut blocks: Subscription<Header<u32, BlakeTwo256>> =
        api.client.rpc().subscribe_finalized_blocks().await.unwrap();

    let first_block = blocks.next().await.unwrap().expect("failed to read block");

    let second_block = blocks.next().await.unwrap().expect("failed to read block");

    assert!(second_block.number > first_block.number);
}

/// Below test will fail since Account 'alice' doesn't have any balance.
/// It will work on local node or any accounts that have balance.
#[tokio::test]
#[ignore]
async fn check_account() {
    let config = Config::read_from_env();

    let user = crypto::AccountId32::from_str(&config.account_public).unwrap();

    let api = ClientBuilder::new()
        .set_url(&config.test_node_url)
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let account = api.storage().system().account(&user, None).await.unwrap();

    // account.data.free should be larger than account.data.fee_frozen to pay gas fee
    assert!(account.data.free >= account.data.fee_frozen);
}

/// Below test will fail since Account 'alice' doesn't have balance.
/// It will work on local node or any accounts that have balance.
#[tokio::test]
#[ignore]
async fn transfer() {
    let config = Config::read_from_env();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .set_url(&config.test_node_url)
        .build()
        .await
        .unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let dest = AccountKeyring::Bob.to_account_id().into();

    let tx_hash = api
        .tx()
        .balances()
        .transfer(dest, 123_456_789)
        .unwrap()
        .sign_and_submit_then_watch_default(&signer)
        .await;

    assert!(tx_hash.is_ok());
}
