#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod simple_counter_with_hashmap {

    /// To use hashmap(Mapping)
    /// ink_storage::Mapping is more optimised than ink_prelude::collections::HashMap
    use ink_storage::{traits::SpreadAllocate, Mapping};

    /// Auth is a mapping(address => isMember)
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Counter {
        owner: AccountId,
        auth: Mapping<AccountId, bool>,
        count: u64,
        auth_count: u64,
    }

    /// Emitted when tx ocurrs
    #[ink(event)]
    pub struct Transaction {
        value: u64,
    }

    /// Define Error type
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        AlreadyRegistered,
        NotRegistered,
        NotFirstAuth,
        TxValueIsNotValid,
        AccountIdIsNotValid,
    }

    /// Define contract result type
    pub type Result<T> = core::result::Result<T, Error>;

    /// contract function
    impl Counter {
        #[ink(constructor)]
        pub fn new(count: u64, first_auth: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = contract.env().caller();
                contract.count = count;
                contract.auth.insert(first_auth, &true);
                contract.auth_count = 1;
            })
        }

        /// Check whether a caller is the owner or not
        fn _ensure_caller_is_owner(&self) {
            let caller = self.env().caller();
            assert!(caller == self.owner);
            if caller != self.owner {
                panic!("caller is not owner!")
            }
        }

        /// Check whether a caller is auth or not
        fn _ensure_caller_is_auth(&self) {
            let caller = self.env().caller();
            assert!(self.auth.contains(caller));
            if !self.auth.contains(caller) {
                panic!("caller is not auth!")
            }
        }

        /// Only contract owner can set the first auth.
        #[ink(message)]
        pub fn set_first_auth(&mut self, first_auth: AccountId) -> Result<()> {
            self._ensure_caller_is_owner();

            if self.auth_count != 0 {
                return Err(Error::NotFirstAuth);
            }

            self.auth.insert(&first_auth, &true);
            self.auth_count += 1;

            Ok(())
        }

        /// transfer owner to other account id
        #[ink(message)]
        pub fn transfer_ownership(&mut self, to: AccountId) -> Result<()> {
            self._ensure_caller_is_owner();

            if to == AccountId::from([0x00; 32]) || to == self.owner {
                return Err(Error::AccountIdIsNotValid);
            }

            self.owner = to;
            Ok(())
        }

        /// Auth can register new auth.
        #[ink(message)]
        pub fn register_new_auth(&mut self, new_auth: AccountId) -> Result<()> {
            self._ensure_caller_is_auth();

            if self.auth.contains(&new_auth) {
                return Err(Error::AlreadyRegistered);
            }

            self.auth.insert(&new_auth, &true);
            self.auth_count += 1;

            Ok(())
        }

        /// Only auth can remove auth
        #[ink(message)]
        pub fn remove_auth(&mut self, auth: AccountId) -> Result<()> {
            self._ensure_caller_is_auth();

            if !self.auth.contains(&auth) {
                return Err(Error::NotRegistered);
            }

            self.auth.remove(&auth);
            self.auth_count -= 1;

            Ok(())
        }

        /// execute a transaction
        #[ink(message)]
        pub fn execute_tx(&mut self, value: u64) -> Result<u64> {
            self._ensure_caller_is_auth();

            if value > 10 {
                return Err(Error::TxValueIsNotValid);
            }
            self.count += value;
            self.env().emit_event(Transaction { value });

            Ok(value)
        }

        /// increment
        #[ink(message)]
        pub fn increment(&mut self) {
            self._ensure_caller_is_auth();
            self.count += 1;
        }

        /// decrement
        #[ink(message)]
        pub fn decrement(&mut self) {
            self._ensure_caller_is_auth();
            self.count -= 1;
        }

        /// reset
        #[ink(message)]
        pub fn reset(&mut self) {
            self._ensure_caller_is_auth();
            self.count = 0;
        }

        /// return contract owner
        #[ink(message)]
        pub fn get_contract_onwer(&self) -> AccountId {
            self.owner
        }

        /// return auth count
        #[ink(message)]
        pub fn get_auth_count(&self) -> u64 {
            self.auth_count
        }

        /// return whether caller is auth(true) or not(false)
        #[ink(message)]
        pub fn is_auth(&self) -> bool {
            self.auth.contains(self.env().caller())
        }

        /// return whether the account id is auth(true) or not(false)
        #[ink(message)]
        pub fn is_auth_account_id(&self, account_id: AccountId) -> bool {
            self.auth.contains(&account_id)
        }

        /// return count
        #[ink(message)]
        pub fn get_count(&self) -> u64 {
            self.count
        }
    }

    // Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // module and test functions are marked with a `#[test]` attribute.
    // The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn init_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let counter = Counter::new(u64::MAX, account_id2);

            assert_eq!(counter.get_count(), u64::MAX);
            assert_eq!(counter.get_auth_count(), 1);

            assert_eq!(counter.get_contract_onwer(), account_id);
            assert!(counter.get_contract_onwer() != account_id2);

            assert!(!counter.is_auth_account_id(account_id));
            assert!(counter.is_auth_account_id(account_id2));
        }

        ///set first auth when there is no auth.
        #[ink::test]
        fn set_first_auth_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let account_id3 = AccountId::from([0x03; 32]);
            let mut counter = Counter::new(0, account_id);

            assert!(counter.set_first_auth(account_id) == Err(Error::NotFirstAuth));
            assert_eq!(counter.get_auth_count(), 1);
            assert_eq!(counter.remove_auth(account_id), Ok(()));
            assert_eq!(counter.get_auth_count(), 0);
            assert_eq!(counter.set_first_auth(account_id2), Ok(()));
            assert_eq!(counter.get_auth_count(), 1);
            assert_eq!(
                counter.set_first_auth(account_id3),
                Err(Error::NotFirstAuth)
            );
            assert_eq!(counter.get_auth_count(), 1);

            assert!(!counter.is_auth_account_id(account_id));
            assert!(counter.is_auth_account_id(account_id2));
        }

        #[ink::test]
        #[should_panic]
        fn only_auth_works() {
            // let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let account_id3 = AccountId::from([0x03; 32]);
            let mut counter = Counter::new(0, account_id2);

            assert!(!counter.is_auth());
            assert_eq!(counter.register_new_auth(account_id3), Ok(())); //panic; only auth can register new auth.
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let mut counter = Counter::new(0, account_id);

            assert_eq!(
                counter.transfer_ownership(AccountId::from([0x00; 32])),
                Err(Error::AccountIdIsNotValid)
            );
            assert_eq!(counter.get_contract_onwer(), account_id);
            assert_eq!(
                counter.transfer_ownership(account_id),
                Err(Error::AccountIdIsNotValid)
            );
            assert_eq!(counter.get_contract_onwer(), account_id);
            assert_eq!(counter.transfer_ownership(account_id2), Ok(()));
            assert_eq!(counter.get_contract_onwer(), account_id2); //change owner
        }

        #[ink::test]
        #[should_panic]
        fn only_owner_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let mut counter = Counter::new(0, account_id);

            assert_eq!(counter.remove_auth(account_id), Ok(()));
            assert_eq!(counter.get_auth_count(), 0);
            assert_eq!(counter.transfer_ownership(account_id2), Ok(()));
            assert_eq!(counter.get_contract_onwer(), account_id2);
            assert_eq!(counter.set_first_auth(account_id2), Ok(())); //panic; only owner can set the first auth.
        }

        #[ink::test]
        fn register_auth_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let account_id3 = AccountId::from([0x03; 32]);
            let mut counter = Counter::new(0, account_id);

            assert_eq!(counter.register_new_auth(account_id2), Ok(()));
            assert_eq!(counter.register_new_auth(account_id3), Ok(()));
            assert!(counter.is_auth_account_id(account_id));
            assert!(counter.is_auth_account_id(account_id2));
            assert!(counter.is_auth_account_id(account_id3));
            assert_eq!(counter.get_auth_count(), 3);

            assert_eq!(
                counter.register_new_auth(account_id3),
                Err(Error::AlreadyRegistered)
            );
            assert_eq!(counter.get_auth_count(), 3);
        }

        #[ink::test]
        fn remove_auth_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let account_id2 = AccountId::from([0x02; 32]);
            let account_id3 = AccountId::from([0x03; 32]);
            let mut counter = Counter::new(0, account_id);

            assert_eq!(counter.register_new_auth(account_id2), Ok(()));
            assert_eq!(counter.remove_auth(account_id3), Err(Error::NotRegistered));
            assert_eq!(counter.get_auth_count(), 2);
            assert_eq!(counter.register_new_auth(account_id3), Ok(()));
            assert_eq!(counter.get_auth_count(), 3);
            assert_eq!(counter.remove_auth(account_id3), Ok(()));
            assert_eq!(counter.get_auth_count(), 2);
        }

        #[ink::test]
        #[should_panic]
        fn underflow() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let mut counter = Counter::new(0, account_id);
            counter.decrement(); //panic
        }

        #[ink::test]
        #[should_panic]
        fn overflow() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let mut counter = Counter::new(u64::MAX, account_id);
            counter.increment(); //panic
        }

        #[ink::test]
        fn reset_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let mut counter = Counter::new(0, account_id);

            counter.increment();
            counter.increment();
            counter.increment();
            counter.increment();
            counter.increment();
            assert_eq!(counter.get_count(), 5);
            counter.decrement();
            counter.decrement();
            counter.decrement();
            assert_eq!(counter.get_count(), 2);
            counter.reset();
            assert_eq!(counter.get_count(), 0);
        }

        #[ink::test]
        fn tx_works() {
            let account_id = AccountId::from([0x01; 32]); //default owner
            let mut counter = Counter::new(100, account_id);

            assert_eq!(counter.execute_tx(0), Ok(0));
            assert_eq!(counter.get_count(), 100);
            assert_eq!(counter.execute_tx(5), Ok(5));
            assert_eq!(counter.get_count(), 105);
            assert_eq!(counter.execute_tx(10), Ok(10));
            assert_eq!(counter.get_count(), 115);
            assert_eq!(counter.execute_tx(11), Err(Error::TxValueIsNotValid));
            assert_eq!(counter.get_count(), 115);
            assert_eq!(counter.execute_tx(u64::MAX), Err(Error::TxValueIsNotValid));
            assert_eq!(counter.get_count(), 115);
        }
    }
}