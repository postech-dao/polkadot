#![cfg_attr(not(feature = "std"), no_std)]

pub use self::client::{Client, ClientRef};

use ink_lang as ink;

#[ink::contract]
mod client {
    use pdao_colony_contract_common::*;
    use pdao_beacon_chain_common::*;
    use ink_env::call::FromAccountId;
    use ink_storage::traits::SpreadAllocate;
    
    #[ink(storage)]
    pub struct Client {
        height: u64,
        last_header: Header,
        chain_name : String,
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
                chain_name : String::from("Astar"),
            }
        }


        #[ink(message)]
        pub fn update_light_client(&mut self, header: light_client::Header, proof: light_client::BlockFinalizationProof) -> Result<()> {

            let mut state = LightClient{
                height : self.height,
                last_header : self.last_header.clone(),
                chain_name : self.chain_name.clone(),
            };

            if false == LightClient::update(&mut state, header, proof) {
                return Err(Error::UpdateError);
            };
            Ok(())
        }

        #[ink(message)]
        pub fn verify_commitment(
            &self,
            _message: message::DeliverableMessage,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {

            let state = LightClient{
                height : self.height,
                last_header : self.last_header.clone(),
                chain_name : self.chain_name.clone(),
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


    }
}