#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod treasury {
    use pdao_colony_contract_common::*;
    
    use client::ClientRef;
    use pdao_beacon_chain_common::*;
    use ink_env::call::FromAccountId;
    use ink_storage::traits::SpreadAllocate;
    use rust_decimal::prelude::*;

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
        pub fn verify_tx(&self, _message: message::DeliverableMessage, block_height: u64, proof: MerkleProof) -> Result<()> {
           
            let e = self.light_client.verify_commitment(_message, block_height, proof).unwrap();
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

        #[ink(message)]
        pub fn transfer_token(&mut self, _message: message::DeliverableMessage, block_height: u64, proof: MerkleProof) {

            let _message2=_message.clone();
            let _proof = proof.clone();
            self.verify_tx(_message2, block_height, _proof).expect("Verify failed");

            match _message {
                message::DeliverableMessage::FungibleTokenTransfer(_message) => {
                    self.transfer_treasury_FT(_message, block_height, proof);
                },
                message::DeliverableMessage::NonFungibleTokenTransfer(_message) => {
                    self.transfer_treasury_NFT(_message, block_height, proof);
                },
                message::DeliverableMessage::Custom(_message) => {
                    self.transfer_treasury_Custom(_message, block_height, proof);
                },
            }

        }

        //#[ink(payable)]
        pub fn transfer_treasury_FT(&mut self, _message: message::FungibleTokenTransfer, block_height: u64, proof: MerkleProof) -> Result<()> {
            
            let addr = _message.receiver_address.as_str();
            let value = _message.amount as u128;

            assert!(value >= self.env().balance(), "insufficient funds!");

            let account = addr.to_owned();
            //let AccountId: T::AccountId = account;
            //self.env().transfer(AccountId, value);            
            Ok(())

        }
        pub fn transfer_treasury_NFT(&mut self, _message: message::NonFungibleTokenTransfer, block_height: u64, proof: MerkleProof) -> Result<()> {
            unimplemented!();
        }
        pub fn transfer_treasury_Custom(&mut self, _message: message::Custom, block_height: u64, proof: MerkleProof) -> Result<()> {
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
            let mut treasury = create_contract(contract_balance);
            assert_eq!(treasury.get_treasury_fungible_token_balance(), 100);
            /*
            let mut s = (accounts.eve).clone();
            AsRef::<AccountId>::as_ref(&s);
            */
            let FTMsg=FungibleTokenTransfer{
                token_id : String::from("12"),
                amount : 150,
                receiver_address : String::from(s),
                contract_sequence : 2,
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

        fn default_accounts(
        ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }
        pub struct FungibleTokenTransfer {
            pub token_id: String,
            pub amount: u128,
            pub receiver_address: String,
            pub contract_sequence: u64,
        }


    }
}