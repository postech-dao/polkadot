use ink_prelude::vec::Vec;
use pdao_polkadot_interact::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Local http server for testing.
    test_http_server_url: String,
    /// Local node url for substrate-contract-node.
    test_local_node_url: String,
    /// Rococo testnet url.
    test_rococo_node_url: String,
    /// Shibuya testnet url.
    test_shibuya_node_url: String,
    /// Deployed contract address on shibuya or shiden.
    contract_address: String,
    /// Test account address.
    account_public: String,
    /// Native token decimal.
    planck_to_one: u8,
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

/// Return block hash and timestamp from the latest block height.
#[tokio::test]
async fn check_connection() {
    let config = Config::read_from_env();
    let height = get_current_height(&config.test_shibuya_node_url, &config.test_http_server_url)
        .await
        .unwrap()
        .unwrap();
    let _block = get_block(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        height,
    )
    .await
    .unwrap();

    // println!("{:?}", block);
}

/// Return block height of the latest finalized block.
#[tokio::test]
async fn check_block_number() {
    let config = Config::read_from_env();
    let first_block =
        get_current_height(&config.test_shibuya_node_url, &config.test_http_server_url)
            .await
            .unwrap()
            .unwrap();
    let second_block =
        get_current_height(&config.test_shibuya_node_url, &config.test_http_server_url)
            .await
            .unwrap()
            .unwrap();

    assert!(first_block < second_block);
}

/// Return native token, meme token, nft balances.
#[tokio::test]
async fn check_account() {
    let config = Config::read_from_env();
    let account = query_account(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        &config.account_public,
    )
    .await
    .unwrap();
    let account_balance = account.native_token.parse::<u64>().unwrap();

    assert!(account_balance > 1_000_000_000);
}

/// Transfer the native token.
#[tokio::test]
async fn transfer_token() {
    let config = Config::read_from_env();
    let amount_to_transfer = 123_456_789;
    let planck_to_one = config.planck_to_one;
    let result = transfer_native_token(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        &config.account_public,
        amount_to_transfer,
        planck_to_one,
    )
    .await
    .unwrap();

    println!("Transaction hash: {}", result);
}

/// Query the state of deployed contract.
#[tokio::test]
async fn check_contract_state() {
    let config = Config::read_from_env();
    let field = "auth"; // get_count
    let result = query_contract_state(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        &config.contract_address,
        Contract::SimpleCounter,
        field,
    )
    .await
    .unwrap();

    assert_eq!(
        result.output[0],
        "YtyhRxkUA5gAPsFXQzQKdexK4GUCaiDqk8RrQtU4FiwNYHY"
    );
}

/// Send a transaction to deployed contract.
#[tokio::test]
async fn execute_contract() {
    let config = Config::read_from_env();
    // No argument in fn increment().
    let argument = Vec::new();
    let method_name = "increment";
    let result = execute_contract_method(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        &config.contract_address,
        Contract::SimpleCounter,
        method_name,
        argument,
    )
    .await
    .unwrap();

    assert_eq!(result.message_name, method_name);
    assert_eq!(result.message_type, "tx");
}

/// Deploy contract from the contract name.
#[tokio::test]
async fn deploy_contract_with_name() {
    let config = Config::read_from_env();
    let mut argument = Vec::new();
    argument.push("5");
    let _result = deploy_contract(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        Contract::SimpleCounter,
        argument,
    )
    .await
    .unwrap();
}

/// Deploy contract from the contract hash.
/// Below test will be failed cause we already used the empty salt for testing.
#[ignore]
#[tokio::test]
async fn deploy_contract_with_hash() {
    let config = Config::read_from_env();
    let mut argument = Vec::new();
    argument.push("5");
    let salt = ""; // Empty string for Null in ts.
    let _result = deploy_contract_with_code_hash(
        &config.test_shibuya_node_url,
        &config.test_http_server_url,
        Contract::SimpleCounter,
        argument,
        salt,
    )
    .await
    .unwrap();
}
