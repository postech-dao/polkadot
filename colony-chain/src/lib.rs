use async_trait::async_trait;
use pdao_beacon_chain_common::message as pbc_message;
use pdao_colony_common::*;
use pdao_colony_contract_common::*;
use pdao_polkadot_interact::{self, get_block, get_current_height};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;

pub struct Shiden {
    pub full_node_uri: String,
    pub http_server_url: String,
    pub treasury_address: String,
    pub light_client_address: String,
}

#[async_trait]
impl ColonyChain for Shiden {
    async fn get_chain_name(&self) -> String {
        "shiden".to_owned()
    }

    async fn get_last_block(&self) -> Result<Block, Error> {
        let height = get_current_height(&self.full_node_uri, &self.http_server_url)
            .await
            .unwrap()
            .unwrap();
        let timestamp = get_block(&self.full_node_uri, &self.http_server_url, height)
            .await
            .unwrap()
            .timestamp;
        Ok(Block { height, timestamp })
    }

    async fn check_connection(&self) -> Result<(), Error> {
        let height = get_current_height(&self.full_node_uri, &self.http_server_url).await;
        match height {
            Ok(_height) => Ok(()),
            Err(_error) => Err(Error::ConnectionError(
                "Unable to get current height from full node".to_owned(),
            )),
        }
    }

    async fn get_contract_list(&self) -> Result<Vec<ContractInfo>, Error> {
        Ok(vec![
            ContractInfo {
                address: self.treasury_address.clone(),
                contract_type: ContractType::LightClient,
                sequence: 0,
            },
            ContractInfo {
                address: self.light_client_address.clone(),
                contract_type: ContractType::Treasury,
                sequence: 0,
            },
        ])
    }

    async fn get_relayer_account_info(&self) -> Result<(String, Decimal), Error> {
        Ok(("0x12341234".to_owned(), dec!(12.34)))
    }

    async fn get_light_client_header(&self) -> Result<Header, Error> {
        Ok("Hmm".to_owned())
    }

    async fn get_treasury_fungible_token_balance(&self) -> Result<HashMap<String, Decimal>, Error> {
        Ok(vec![
            ("Bitcoin".to_owned(), dec!(123.45)),
            ("Ether".to_owned(), dec!(444.44)),
        ]
        .into_iter()
        .collect())
    }

    async fn get_treasury_non_fungible_token_balance(
        &self,
    ) -> Result<Vec<(String, String)>, Error> {
        Ok(vec![
            ("BAYC".to_owned(), "1".to_owned()),
            ("Sandbox Land".to_owned(), "2".to_owned()),
        ])
    }

    async fn update_light_client(
        &self,
        _header: light_client::Header,
        _proof: light_client::BlockFinalizationProof,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn transfer_treasury_fungible_token(
        &self,
        _message: pbc_message::FungibleTokenTransfer,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn transfer_treasury_non_fungible_token(
        &self,
        _message: pbc_message::NonFungibleTokenTransfer,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn deliver_custom_order(
        &self,
        _contract_name: &str,
        _message: pbc_message::Custom,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }
}
