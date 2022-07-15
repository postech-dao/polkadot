#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_lang as ink;

#[ink::contract]
mod counter {

    use ink_prelude::vec::Vec;

    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct State {
        count: u64,
        auth: Vec<AccountId>, //We could use Mapping<,> of course.
        init: bool,
    }

    #[ink(event)] //Event emitted when transaction occurs
    pub struct Transaction {
        value: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        AlreadyRegistered,
        AlreadyRemoved,
        ValueIsOver10,
        CallerNotAuth,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl State {
        #[ink(constructor)]
        pub fn new(_initCount: u64) -> Self {
            Self {
                count: _initCount,
                auth: Vec::new(),
                init: false,
            }
        }

        #[ink(message)]
        pub fn init(&mut self, _initCount: u64, _auth: AccountId) {
            if self.init {
                panic!("Already initalized");
            }

            self.count = _initCount;
            self.auth.push(_auth);
            self.init = true;
        }

        #[ink(message)] //Check the caller is in auth
        pub fn only_auth(&self) {
            let from = Self::env().caller();

            if !self.init {
                panic!("Not initalized yet");
            }

            if !self.auth.contains(&from) {
                panic!("Caller is not authorized");
            }
        }

        #[ink(message)] //Execute our transaction
        pub fn execute(&mut self, input: u64) -> Result<u64> {
            self.only_auth();

            if input > 10 {
                return Err(Error::ValueIsOver10);
            }

            self.count += input;

            Self::env().emit_event(Transaction { value: input });

            Ok(input)
        }

        #[ink(message)] //Add auth
        pub fn add_auth(&mut self, new_auth: AccountId) -> Result<()> {
            self.only_auth();

            if self.auth.contains(&new_auth) {
                return Err(Error::AlreadyRegistered);
            }

            self.auth.push(new_auth);

            Ok(())
        }

        #[ink(message)] //Remove auth from Vec<AccountId>
        pub fn remove_auth(&mut self, _auth: AccountId) -> Result<()> {
            self.only_auth();

            if self.auth.len() == 1 {
                panic!("Auth will be empty");
            }

            if !self.auth.contains(&_auth) {
                return Err(Error::AlreadyRemoved);
            }

            self.auth.retain(|&x| x != _auth);

            Ok(())
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.only_auth();
            self.count += 1;
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            self.only_auth();
            self.count -= 1;
        }

        #[ink(message)]
        pub fn reset(&mut self) {
            self.only_auth();
            self.count = 0;
        }

        #[ink(message)] //Since below 2 functions are "view" function, we don't need to check whether caller is in auth or not.
        pub fn get_count(&self) -> u64 {
            self.count
        }

        #[ink(message)]
        pub fn get_auth(&self) -> Vec<AccountId> {
            self.auth.clone()
        }
    }

    //Below test cases are based on above functions.
    //We can check that our functions work well.

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::counter::Error::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let state = State::new(0);

            assert_eq!(state.count, 0); //count should be 0
            assert_eq!(state.auth.len(), 0); //We don't push any auth yet
        }
        // write down other tests for simple_counter.
    }
}