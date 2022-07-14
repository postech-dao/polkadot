
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

        //Do init
        #[ink::test]
        fn init_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32]));

            assert_eq!(state.auth[0], AccountId::from([0x01; 32]));
        }

        //Below case should panic
        #[ink::test]
        #[should_panic]
        fn prevent_re_init_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32]));

            state.init(3, AccountId::from([0x01; 32])); //panic occurs
        }

        //Get count
        #[ink::test]
        fn get_count_works() {
            let state = State::new(0);

            assert_eq!(state.get_count(), 0);
        }

        //Get Auth
        #[ink::test]
        fn get_auth_works() {
            let mut state = State::new(0);

            state.init(5, AccountId::from([0x01; 32]));

            assert_eq!(state.auth[0], AccountId::from([0x01; 32]));
        }

        #[ink::test]
        fn add_auth_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice);

            let result = state.add_auth(AccountId::from([0x02; 32]));

            assert_eq!(result, Ok(()));
        }

        #[ink::test]
        fn remove_auth_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice);

            let _result = state.add_auth(AccountId::from([0x02; 32]));

            let result2 = state.remove_auth(AccountId::from([0x02; 32]));

            assert_eq!(result2, Ok(()));
        }

        //Testing increment, decrement, reset in one test
        #[ink::test]
        fn count_control_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice);

            state.increment();
            assert_eq!(state.count, 6);

            state.decrement();
            assert_eq!(state.count, 5);

            state.increment();
            assert_eq!(state.count, 6);

            state.reset();
            assert_eq!(state.count, 0);
        }

        //Testing transaction
        #[ink::test]
        fn execute_works() {
            let mut state = State::new(0);

            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            state.init(5, accounts.alice);

            let result = state.execute(8).unwrap();
            assert_eq!(result, 8);

            let result2 = state.execute(15);
            assert_eq!(result2, Err(ValueIsOver10));
        }
    }
}
