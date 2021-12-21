#![cfg_attr(not(feature = "std"), no_std)]
/// This is an example of how an ink! contract may call the Web3Games

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;

pub type MultiTokenId = u32;
pub type TokenId = u32;
pub type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;



/// Error Config
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TokenMultiErr {
    FailCreateToken,
}


impl ink_env::chain_extension::FromStatusCode for TokenMultiErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailCreateToken),
            _ => panic!("encountered unknown status code"),
        }
    }
}

/// Environment Config
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;

    type ChainExtension = TokenMulti;
}


/// Functions Trait Config
#[ink::chain_extension]
pub trait TokenMulti {
    // important !
    type ErrorCode = TokenMultiErr;

    #[ink(extension = 65665, returns_result = false)]
    fn create_token(uri:Vec<u8>) -> MultiTokenId;

    #[ink(extension = 65666, returns_result = false)]
    fn set_approval_for_all(id:MultiTokenId,operator:AccountId,approved:bool) -> MultiTokenId;

    #[ink(extension = 65667, returns_result = false)]
    fn transfer_from(id:MultiTokenId,from:AccountId,to:AccountId,token_id:TokenId,amount:Balance);

    #[ink(extension = 65568, returns_result = false)]
    fn batch_transfer_from(id:MultiTokenId,from:AccountId,to:AccountId,token_ids:Vec<TokenId>,amounts:Vec<Balance>);

    #[ink(extension = 65569, returns_result = false)]
    fn mint(id:MultiTokenId,to:AccountId,token_id:TokenId,amount:Balance);

    #[ink(extension = 65570, returns_result = false)]
    fn mint_batch(id:MultiTokenId,to:AccountId,token_ids:Vec<TokenId>,amounts:Vec<Balance>);

    #[ink(extension = 65571, returns_result = false)]
    fn burn(id:MultiTokenId,token_id:TokenId,amount:Balance);

    #[ink(extension = 65572, returns_result = false)]
    fn burn_batch(id:MultiTokenId,token_ids:Vec<TokenId>,amounts:Vec<Balance>);
}

/// Contract Config
#[ink::contract(env = crate::CustomEnvironment)]
pub mod rand_extension {
    use super::*;
    use ink_prelude::vec::Vec;


    #[ink(storage)]
    pub struct MultiTokenExtension {
        /// Stores a single `bool` value on the storage.
        value: MultiTokenId,
    }

    #[ink(event)]
    pub struct MultiTokenCreate {
        #[ink(topic)]
        multi_token_id: MultiTokenId,
    }


    impl MultiTokenExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{value:Default::default()}
        }


        #[ink(message)]
        pub fn create_multi_token(&mut self, uri:Vec<u8>) -> Result<(), TokenMultiErr> {
            let multi_token_id = self.env().extension().create_token(uri)?;
            self.value = multi_token_id;
            Self::env().emit_event(MultiTokenCreate { multi_token_id });
            Ok(())
        }

        #[ink(message)]
        pub fn approve_multi_token(&mut self, id:MultiTokenId,operator:AccountId,approved:bool) -> Result<(), TokenMultiErr> {
            self.env().extension().set_approval_for_all(id,operator,approved)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from_multi_token(&mut self, id:MultiTokenId,from:AccountId,to:AccountId,token_id:TokenId,amount:Balance) -> Result<(), TokenMultiErr> {
            self.env().extension().transfer_from(id,from,to,token_id,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn batch_transfer_from_multi_token(&mut self, id:MultiTokenId,from:AccountId,to:AccountId,token_ids:Vec<TokenId>,amounts:Vec<Balance>) -> Result<(), TokenMultiErr> {
            self.env().extension().batch_transfer_from(id,from,to,token_ids,amounts)?;
            Ok(())
        }


        #[ink(message)]
        pub fn mint_multi_token(&mut self, id:MultiTokenId,to:AccountId,token_id:TokenId,amount:Balance) -> Result<(), TokenMultiErr> {
            self.env().extension().mint(id,to,token_id,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint_batch_multi_token(&mut self, id:MultiTokenId,to:AccountId,token_ids:Vec<TokenId>,amounts:Vec<Balance>) -> Result<(), TokenMultiErr> {
            self.env().extension().mint_batch(id,to,token_ids,amounts)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn_multi_token(&mut self, id:MultiTokenId,token_id:TokenId,amount:Balance) -> Result<(), TokenMultiErr> {
            self.env().extension().burn(id,token_id,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn_batch_multi_token(&mut self, id:MultiTokenId,token_ids:Vec<TokenId>,amounts:Vec<Balance>) -> Result<(), TokenMultiErr> {
            self.env().extension().burn_batch(id,token_ids,amounts)?;
            Ok(())
        }


        #[ink(message)]
        pub fn get(&self) -> MultiTokenId {
            self.value
        }
    }


    /// Test Config
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let Multi_token_extension = MultiTokenExtension::default();
            assert_eq!(Multi_token_extension.get(), [0; 32]);
        }
    }
}