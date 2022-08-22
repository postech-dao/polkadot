extern crate dotenv;
use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, fmt::Debug};

const HTTP_SERVER: &str = "http://localhost:8080/";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Block {
    pub block_hash: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Account {
    pub native_token: String,
    pub meme_token: String,
    pub non_fungible_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContractQuery {
    pub contract_name: String,
    pub message_name: String,
    pub message_type: String,
    pub output: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContractTx {
    pub contract_name: String,
    pub message_name: String,
    pub message_type: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContractDeploy {
    pub contract_name: String,
    pub contract_addr: String,
    pub tx_hash: String,
}

/// Return JSON response from path and data.
pub async fn get_response(path: &str, data: Value) -> Result<Value> {
    let client = reqwest::Client::new();
    let response = client
        .post(HTTP_SERVER.to_owned() + path)
        .json(&data)
        .send()
        .await?;

    Ok(match response.status() {
        reqwest::StatusCode::OK => response.json().await?,
        other => panic!("fail to get reponse properly: {:?}", other),
    })
}

/// Return the current block height.
pub async fn get_current_height(full_node_uri: &str) -> Result<Option<u64>> {
    let path = "current-height";
    let data = json!({
        "fullNodeUri": full_node_uri,
    });
    let result = get_response(path, data).await?;

    Ok(result["data"]["height"].as_u64())
}

/// Return the current block hash and timestamp.
pub async fn get_block(full_node_uri: &str, height: u64) -> Result<Block> {
    let path = "block-info";
    let data = json!({
        "fullNodeUri": full_node_uri,
        "height": height,
    });
    let result = get_response(path, data).await?;
    let block: Block = serde_json::from_value(result["data"].clone())?;

    Ok(block)
}

/// Return the native token, meme token(TBD), nft(TBD) balance of the given account.
pub async fn query_account(full_node_uri: &str, addr: &str) -> Result<Account> {
    let path = "account-info";
    let data = json!({
        "fullNodeUri": full_node_uri,
        "addr": addr,
    });
    let result = get_response(path, data).await?;
    let account: Account = serde_json::from_value(result["data"].clone())?;

    Ok(account)
}

/// Transfer the native token to receiver account.
pub async fn transfer_native_token(
    full_node_uri: &str,
    receiver_public_key: &str,
    amount: u64,
    planck_to_one: u8,
) -> Result<String> {
    let path = "native-token/transfer";
    dotenv().expect("failed to read .env file");
    let data = json!({
        "fullNodeUri": full_node_uri,
        "mnemonic": env::var("SIGNER_MNEMONIC").expect("fail to load signer mnemonic").as_str(),
        "toAddr": receiver_public_key,
        "amount": amount,
        "planckToOneNT": planck_to_one,
    });
    let result: Value = get_response(path, data).await?;

    Ok(result["data"]["txHash"].to_string())
}

/// Query the state of the deployed contract.
pub async fn query_contract_state(
    full_node_uri: &str,
    contract_addr: &str,
    contract_name: &str,
    field: &str,
) -> Result<ContractQuery> {
    let path = "contract-state";
    let data = json!({
        "fullNodeUri": full_node_uri,
        "contractAddr": contract_addr,
        "contractName": contract_name,
        "field": field,
    });
    let result = get_response(path, data).await?;
    let contract_tx: ContractQuery = serde_json::from_value(result["data"].clone())?;

    Ok(contract_tx)
}

/// Execute the method(Send a transaction) of the deployed contract.
pub async fn execute_contract_method(
    full_node_uri: &str,
    contract_addr: &str,
    contract_name: &str,
    method_name: &str,
    arguments: Vec<&str>,
) -> Result<ContractTx> {
    let path = "contract-method/execute";
    dotenv().expect("failed to read .env file");
    let data = json!({
        "fullNodeUri": full_node_uri,
        "mnemonic": env::var("SIGNER_MNEMONIC").expect("fail to load signer mnemonic").as_str(),
        "contractAddr": contract_addr,
        "contractName": contract_name,
        "methodName": method_name,
        "arguments": arguments,
    });
    let result: Value = get_response(path, data).await?;
    let contract_tx: ContractTx = serde_json::from_value(result["data"].clone())?;

    Ok(contract_tx)
}

/// Deploy the contract with its name.
/// e.g. simple_counter, light_client, treasury.
pub async fn deploy_contract(
    full_node_uri: &str,
    contract_name: &str,
    arguments: Vec<&str>,
) -> Result<ContractDeploy> {
    let path = "contract/deploy";
    dotenv().expect("failed to read .env file");
    let data = json!({
        "fullNodeUri": full_node_uri,
        "mnemonic": env::var("SIGNER_MNEMONIC").expect("fail to load signer mnemonic").as_str(),
        "contractName": contract_name,
        "arguments": arguments,
    });
    let result = get_response(path, data).await?;
    let contract_deploy: ContractDeploy = serde_json::from_value(result["data"].clone())?;

    Ok(contract_deploy)
}

/// Deploy the contract with code hash.
pub async fn deploy_contract_with_code_hash(
    full_node_uri: &str,
    contract_name: &str,
    arguments: Vec<&str>,
    salt: &str,
) -> Result<ContractDeploy> {
    let path = "contract-from-code-hash/deploy";
    dotenv().expect("failed to read .env file");
    let data = json!({
        "fullNodeUri": full_node_uri,
        "mnemonic": std::env::var("SIGNER_MNEMONIC").expect("fail to load signer mnemonic").as_str(),
        "contractName": contract_name,
        "arguments": arguments,
        "salt": salt,
    });
    let result = get_response(path, data).await?;
    let contract_deploy: ContractDeploy = serde_json::from_value(result["data"].clone())?;

    Ok(contract_deploy)
}
