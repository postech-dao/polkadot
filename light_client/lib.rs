#![cfg_attr(not(feature = "std"), no_std)]

pub use self::client::{Client, ClientRef};

use ink_lang as ink;

#[ink::contract]
mod client {
    use pdao_colony_contract_common::*;

    use ink_env::call::FromAccountId;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub struct LightClientUpdateMessage2 {
        pub header: Header,
        pub proof: BlockFinalizationProof,
    }
    //#[derive(Copy, Clone)]
    #[ink(storage)]
    pub struct Client {
        height: u64,
        last_header: Header,
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
            }
        }


        #[ink(message)]
        pub fn update(&mut self, message: LightClientUpdateMessage2) -> Result<()> {
            let header = message.header;
            let proof = message.proof;

            let mut state = LightClient{
                height : self.height,
                last_header : self.last_header.clone(),
            };

            if false == LightClient::update(&mut state, header, proof) {
                return Err(Error::UpdateError);
            };
            Ok(())
        }

        #[ink(message)]
        pub fn verify_commitment(
            &self,
            _message: Vec<u8>,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {

            let state = LightClient{
                height : self.height,
                last_header : self.last_header.clone(),
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