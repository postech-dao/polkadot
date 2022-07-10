#![cfg_attr(not(feature = "std"), no_std)]

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
    }
    // Emitted when transaction executed successfully.
    #[ink(event)]
    pub struct Transaction {
        value: u64,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the name already exists upon registration.
        AuthAlreadyExists,
        /// Returned if caller is not owner while required to.
        NotRegistered,
        /// Returned if value is over than 10.
        ValueisOverthanTen,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl State {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                count: 0,
                auth: Vec::new(),
            }
        }
        //Init value and clear all auth.
        #[ink(message)]
        pub fn init(&mut self, _init_value: u64) {
            self.count = _init_value;
            self.auth.clear();
        }
        //Only register Id.
        #[ink(message)]
        pub fn register(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if self.auth.contains(&caller) {
                return Err(Error::AuthAlreadyExists);
            }

            self.auth.push(caller);

            Ok(())
        }
        //Check itself auth or not.
        #[ink(message)]
        pub fn auth_check(&self) -> Result<()> {
            let caller = Self::env().caller();

            if !self.auth.contains(&caller) {
                return Err(Error::NotRegistered);
            } else {
                return Ok(());
            }
        }
        //Check auth with Id.
        #[ink(message)]
        pub fn is_auth(&self, account_id: AccountId) -> bool {
            self.auth.contains(&account_id)
        }

        #[ink(message)]
        pub fn execute(&mut self, value: u64) -> Result<u64> {
            if Err(Error::NotRegistered) == self.auth_check() {
                return Err(Error::NotRegistered);
            }

            if value > 10 {
                return Err(Error::ValueisOverthanTen);
            }

            self.count += value;
            Self::env().emit_event(Transaction { value });

            Ok(value)
        }
        //Reset count to zero.
        #[ink(message)]
        pub fn reset(&mut self) {
            self.count = 0;
        }

        #[ink(message)]
        pub fn get_count(&self) -> u64 {
            self.count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let state = State::new();
            assert_eq!(state.get_count(), 0);
        }
        #[ink::test]
        fn init_works() {
            let account_id = AccountId::from([0x01; 32]); 

            let mut state = State::new();
            assert_eq!(state.get_count(), 0);
            set_caller(account_id);
            state.register();

            state.init(5);
            assert_eq!(state.get_count(), 5);
            assert!(!state.is_auth(account_id));
        }
        #[ink::test]
        fn register_works() {
            let account_id = AccountId::from([0x01; 32]); 
            let account_id2 = AccountId::from([0x02; 32]);
            let account_id3 = AccountId::from([0x03; 32]);

            let mut state = State::new();
            set_caller(account_id);
            assert_eq!(state.register(), Ok(()));
            set_caller(account_id2);
            assert_eq!(state.register(), Ok(()));

            assert_eq!(state.is_auth(account_id), true);
            assert_eq!(state.is_auth(account_id2), true);
            assert_eq!(state.is_auth(account_id3), false);
        }
        #[ink::test]
        fn auth_check_works() {
            let account_id = AccountId::from([0x01; 32]); 

            let mut state = State::new();
            state.init(4);
            assert_eq!(state.get_count(), 4);
            set_caller(account_id);
            state.register();
            assert_eq!(state.auth_check(), Ok(()));
        }
        #[ink::test]
        fn execute_works() {
            let account_id = AccountId::from([0x01; 32]); 
            let account_id2 = AccountId::from([0x02; 32]);

            let mut state = State::new();
            set_caller(account_id);
            state.register();
            assert_eq!(state.execute(5), Ok(5));
            set_caller(account_id2);
            assert_eq!(state.execute(3), Err(Error::NotRegistered));
            state.register();
            assert_eq!(state.execute(15), Err(Error::ValueisOverthanTen));
            assert_eq!(state.execute(8), Ok(8));
            assert_eq!(state.get_count(), 13);
        }
        #[ink::test]
        fn reset_works() {
            let account_id = AccountId::from([0x01; 32]);
            let account_id2 = AccountId::from([0x02; 32]);

            let mut state = State::new();
            set_caller(account_id);
            state.register();
            assert_eq!(state.execute(6), Ok(6));
            set_caller(account_id2);
            state.reset();
            assert_eq!(state.get_count(), 0);
        }

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }
    }
}

