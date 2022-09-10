#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod treasury {
    use pdao_colony_contract_common::*;

    use client::ClientRef;
    use ink_env::call::FromAccountId;
    use ink_env::*;
    use pdao_beacon_chain_common::*;
    use parity_codec::{Decode, Encode};
    use sp_runtime::AccountId32;
    use rustc_hex::FromHex;
    use bs58;

    #[ink(storage)]
    pub struct Treasury {
        light_client: ClientRef,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NoMatching,
        FailToTransfer,
        FailToVerify,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Treasury {
        #[ink(constructor)]
        pub fn new(lightclient_code: AccountId) -> Self {
            let light_client = ClientRef::from_account_id(lightclient_code);
            Self { light_client }
        }

        //invoked by transfer_token(). It calls light-client contract's methods(cross-contract call)
        pub fn verify_tx(
            &self,
            _message: message::DeliverableMessage,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            //if error, tx be reverted
            let e = self
                .light_client
                .verify_commitment(_message, block_height, proof)
                .unwrap();
            //if no-error tx, committed
            Ok(())
        }

        //get balance for fungible token
        #[ink(message)]
        pub fn get_treasury_fungible_token_balance(&self) -> Balance {
            ink_env::debug_println!("received payment: {}", self.env().balance());
            let balance = self.env().balance().into();
            balance
        }
        //verify tx and call withdraw function for each type
        #[ink(message)]
        pub fn transfer_token(
            &mut self,
            _message: message::DeliverableMessage,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            let _message2 = _message.clone();
            let _proof = proof.clone();

            //
            self.verify_tx(_message2, block_height, _proof)
                .expect("Verify failed");

            match _message {
                message::DeliverableMessage::FungibleTokenTransfer(_message) => {
                    self.transfer_treasury_FT(_message, block_height, proof);
                }
                message::DeliverableMessage::NonFungibleTokenTransfer(_message) => {
                    self.transfer_treasury_NFT(_message, block_height, proof);
                }
                message::DeliverableMessage::Custom(_message) => {
                    self.transfer_treasury_Custom(_message, block_height, proof);
                }
            }
            Err(Error::NoMatching)
        }

        //withdraw balance to valid account. trnasfer_token() calls this method
        pub fn transfer_treasury_FT(
            &mut self,
            _message: message::FungibleTokenTransfer,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            //let addr = "_message.receiver_address".as_str();
            let value = _message.amount as u128;
            assert!(value >= self.env().balance(), "insufficient balance!");

            let address_str = _message.receiver_address;
            let mut output:Vec<u8>;
            match bs58::decode(address_str).into(&mut output){
                Ok(_res)=> (),
                Err(_e)=> (),
            };
            let cut_address_vec:Vec<u8> = output.drain(1..33).collect();
            let mut array = [0; 32];
            let bytes = &cut_address_vec[..array.len()]; 
            array.copy_from_slice(bytes); 
            let account32: AccountId32 = array.into();
            let mut to32 = AccountId32::as_ref(&account32);
            let to_address : AccountId = AccountId::decode(&mut to32).unwrap_or_default();
            if self.env().transfer(to_address, value).is_err() {
                return Err(Error::FailToTransfer);
            };
            Ok(())
        }
        pub fn transfer_treasury_NFT(
            &mut self,
            _message: message::NonFungibleTokenTransfer,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            unimplemented!();
        }
        pub fn transfer_treasury_Custom(
            &mut self,
            _message: message::Custom,
            block_height: u64,
            proof: MerkleProof,
        ) -> Result<()> {
            unimplemented!();
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::treasury::Error::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let contract_balance = 100;
            let accounts = default_accounts();
            let treasury = create_contract(contract_balance);
            assert_eq!(treasury.get_treasury_fungible_token_balance(), 100);

            let FTMsg = message::FungibleTokenTransfer {
                token_id: String::from("12"),
                amount: 150,
                receiver_address: String::from("0x01"),
                contract_sequence: 2,
            };
            set_sender(accounts.eve);
            set_balance(accounts.eve, 0);
        }

        fn create_contract(initial_balance: Balance) -> Treasury {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);

            Treasury::new(accounts.eve)
        }
        fn contract_id() -> AccountId {
            ink_env::test::callee::<ink_env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }
    }
}
//
