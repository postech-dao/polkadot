#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod treasury {
    //use pdao_colony_contract_common::*;
    
    use client::ClientRef;

    use ink_env::call::FromAccountId;
    use ink_storage::traits::SpreadAllocate;
    use rust_decimal::prelude::*;
    pub type MerkleProof = String;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, scale_info:: TypeInfo)]
    pub struct FungibleTokenTransferMessage {
        pub token_id: String,
        pub amount: u128,
        pub receiver_address: String,
        pub contract_sequence: u64,
    }

    #[ink(storage)]
    pub struct Treasury {
        light_client: ClientRef,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        FailToVerify,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Treasury {
        #[ink(constructor)]
        pub fn new(lightclient_code: AccountId) -> Self {
            let light_client = ClientRef::from_account_id(lightclient_code);
            Self {
                light_client,
            }
        }

        #[ink(message)]
        pub fn verify_fungible_token(&self, message: FungibleTokenTransferMessage, block_height: u64, proof: MerkleProof) -> Result<()> {
            let v = Vec::new();
            let e = self.light_client.verify_commitment(v, block_height, proof).unwrap();
            Ok(())
        }

        #[ink(message)]
        pub fn get_treasury_fungible_token_balance(&self) -> Balance {
            ink_env::debug_println!(
                "received payment: {}",
                self.env().balance()
            );
            let balance = self.env().balance().into();
            balance
        }

        /*
        pub fn get_treasury_non_fungible_token_balance() {

        }
        */

        #[ink(message, payable)]
        pub fn transfer_treasury_fungible_token(&mut self, message: FungibleTokenTransferMessage, block_height: u64, proof: MerkleProof) -> Result<()> {

            let addr = message.receiver_address.as_bytes();
            let value = message.amount as u128;

            assert!(value >= self.env().balance(), "insufficient funds!");

            self.verify_fungible_token(message, block_height, proof).expect("Verify failed");

            //let account = AccountId::from(addr);
            let dummy = AccountId::from([0x01; 32]);
            self.env().transfer(dummy, value);            
            Ok(())

        }
        /*

        #[ink(message, payable)]
        pub fn transfer_treasury_non_fungible_token(&mut self, message: NonFungibleTokenTransferMessage, block_height: u64, proof: MerkleProof) {
            //ink_env::debug_println!("requested value: {}", value);
            ink_env::debug_println!("contract balance: {}", self.env().balance());

            assert!(value <= self.env().balance(), "insufficient funds!");

            self.verify(message, block_height, proof).expect("Verify failed");
            
            let addr = message.receiver_address;
            let value = message.amount;

            self.env().transfer(addr, value);
            

        }
        */
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::treasury::Error::*;
        use ink_lang as ink;

        pub struct FungibleTokenTransferMessage {
            pub token_id: String,
            pub amount: Decimal,
            pub receiver_address: String,
            pub contract_sequence: u64,
        }
    }
}