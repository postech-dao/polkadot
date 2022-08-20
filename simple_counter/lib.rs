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
        auth: Vec<AccountId>,
        init: bool,
    }

    /// Event emitted when transaction occurs
    #[ink(event)]
    pub struct Transaction {
        value: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        AlreadyInitialized,
        NotInitialized,
        WillBeZeroAuth,
        NotAuthorized,
        AlreadyRegistered,
        AlreadyRemoved,
        ValueIsOver10,
        CallerNotAuth,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl State {
        /// Creates a new simple_counter contract.
        #[ink(constructor)]
        pub fn new(init_count: u64) -> Self {
            Self {
                count: init_count,
                auth: Vec::new(),
                init: false,
            }
        }

        /// Initialize the contract.
        #[ink(message)]
        pub fn init(&mut self, init_count: u64, _auth: AccountId) -> Result<()> {
            if self.init {
                return Err(Error::AlreadyInitialized);
            }

            self.count = init_count;
            self.auth.push(_auth);
            self.init = true;

            Ok(())
        }

        /// Checks the caller is in auth
        #[ink(message)]
        pub fn only_auth(&self) -> Result<()> {
            let from = Self::env().caller();

            if !self.init {
                return Err(Error::NotInitialized);
            }

            if !self.auth.contains(&from) {
                return Err(Error::NotAuthorized);
            }

            Ok(())
        }

        /// Executes tx
        #[ink(message)]
        pub fn execute(&mut self, input: u64) -> Result<u64> {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");

            if input > 10 {
                return Err(Error::ValueIsOver10);
            }

            self.count += input;

            Self::env().emit_event(Transaction { value: input });

            Ok(input)
        }

        /// Registers an auth
        #[ink(message)]
        pub fn add_auth(&mut self, new_auth: AccountId) -> Result<()> {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");

            if self.auth.contains(&new_auth) {
                return Err(Error::AlreadyRegistered);
            }

            self.auth.push(new_auth);

            Ok(())
        }

        /// Removes an auth from Vec<AccountId>
        #[ink(message)]
        pub fn remove_auth(&mut self, _auth: AccountId) -> Result<()> {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");

            if self.auth.len() == 1 {
                return Err(Error::WillBeZeroAuth);
            }

            if !self.auth.contains(&_auth) {
                return Err(Error::AlreadyRemoved);
            }

            self.auth.retain(|&x| x != _auth);

            Ok(())
        }

        /// Increases the count by one.
        #[ink(message)]
        pub fn increment(&mut self) {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");
            self.count += 1;
        }

        /// Decreases the count by one.
        #[ink(message)]
        pub fn decrement(&mut self) {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");
            self.count -= 1;
        }

        /// Resets the count.
        #[ink(message)]
        pub fn reset(&mut self) {
            self.only_auth()
                .expect("contract is not initialized or caller is not in auth");
            self.count = 0;
        }

        /// Returns the count.
        #[ink(message)]
        pub fn get_count(&self) -> u64 {
            self.count
        }

        /// Returns the list of auths.
        #[ink(message)]
        pub fn get_auth(&self) -> Vec<AccountId> {
            self.auth.clone()
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::counter::Error::*;
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let state = State::new(0);

            assert_eq!(state.count, 0);
            assert_eq!(state.auth.len(), 0);
        }

        #[ink::test]
        fn init_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32])).unwrap();

            assert_eq!(state.auth[0], AccountId::from([0x01; 32]));
        }

        #[ink::test]
        #[should_panic]
        fn prevent_re_init_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32])).unwrap();

            state.init(3, AccountId::from([0x01; 32])).unwrap(); //panic occurs
        }

        #[ink::test]
        fn get_count_works() {
            let state = State::new(0);

            assert_eq!(state.get_count(), 0);
        }

        #[ink::test]
        fn get_auth_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32])).unwrap();

            assert_eq!(state.auth[0], AccountId::from([0x01; 32]));
        }

        #[ink::test]
        fn add_auth_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice).unwrap();

            let result = state.add_auth(AccountId::from([0x02; 32]));

            assert_eq!(result, Ok(()));
        }

        #[ink::test]
        fn remove_auth_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice).unwrap();

            let _result = state.add_auth(AccountId::from([0x02; 32]));

            let result2 = state.remove_auth(AccountId::from([0x02; 32]));

            assert_eq!(result2, Ok(()));
        }

        // Testing increment, decrement, reset in one test
        #[ink::test]
        fn count_control_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice).unwrap();

            state.increment();
            assert_eq!(state.count, 6);

            state.decrement();
            assert_eq!(state.count, 5);

            state.increment();
            assert_eq!(state.count, 6);

            state.reset();
            assert_eq!(state.count, 0);
        }

        // Testing transaction
        #[ink::test]
        fn execute_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice).unwrap();

            let result = state.execute(8).unwrap();
            assert_eq!(result, 8);

            let result2 = state.execute(15);
            assert_eq!(result2, Err(ValueIsOver10));
        }
    }
}
