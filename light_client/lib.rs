#![cfg_attr(not(feature = "std"), no_std)]

pub use self::client::{Client, ClientRef};

use ink_lang as ink;

#[ink::contract]
mod client {
    use pdao_beacon_chain_common::*;
    use pdao_colony_contract_common::*;

    #[ink(storage)]
    pub struct Client {
        height: u64,
        last_header: Header,
        chain_name: String,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        UpdateError,
        VerifyError,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Client {
        #[ink(constructor)]
        pub fn new(initial_header: Header) -> Self {
            Self {
                height: 0,
                last_header: initial_header,
                chain_name: String::from("Astar"),
            }
        }

        //update it's own header
        #[ink(message)]
        pub fn update_light_client(
            &mut self,
            header: light_client::Header,
            proof: light_client::BlockFinalizationProof,
        ) -> Result<()> {
            let mut state = LightClient {
                height: self.height,
                last_header: self.last_header.clone(),
                chain_name: self.chain_name.clone(),
            };

            if false == LightClient::update(&mut state, header.clone(), proof) {
                return Err(Error::UpdateError);
            };
            self.height += 1;
            self.last_header = header;

            Ok(())
        }
        //invoked by treasury contract, verify txs
        #[ink(message)]
        pub fn verify_commitment(
            &self,
            _message: message::DeliverableMessage,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            let state = LightClient {
                height: self.height,
                last_header: self.last_header.clone(),
                chain_name: self.chain_name.clone(),
            };

            if false == LightClient::verify_commitment(&state, _message, block_height, proof) {
                return Err(Error::VerifyError);
            };
            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn update() {
            let mut state = Client::new(String::from("0x1"));
            let header = String::from("0x2");
            let proof = String::from("valid");
            state.update_light_client(header, proof);
            assert_eq!(state.last_header, String::from("0x2"));
        }

        #[ink::test]
        fn verify() {
            let mut state = Client::new(String::from("0x1"));

            let msg = message::FungibleTokenTransfer {
                token_id: String::from("0x1"),
                amount: 300,
                receiver_address: String::from("0x3"),
                contract_sequence: 21,
            };
            let proof = String::from("valid");
            state.verify_commitment(
                message::DeliverableMessage::FungibleTokenTransfer(msg),
                1,
                proof,
            );
        }
    }
}
