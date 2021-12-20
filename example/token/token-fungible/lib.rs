#![cfg_attr(not(feature = "std"), no_std)]
/// This is an example of how an ink! contract may call the Web3Games

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;

pub type FungibleTokenId = u32;
pub type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;



/// Error Config
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TokenFungibleErr {
    FailCreateToken,
}


impl ink_env::chain_extension::FromStatusCode for TokenFungibleErr {
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

    type ChainExtension = TokenFungible;
}


/// Functions Trait Config
#[ink::chain_extension]
pub trait TokenFungible {
    // important !
    type ErrorCode = TokenFungibleErr;

    #[ink(extension = 65537, returns_result = false)]
    fn create_token(name:Vec<u8>,symbol:Vec<u8>,decimals:u8) -> FungibleTokenId;

    #[ink(extension = 65538, returns_result = false)]
    fn approve(id:FungibleTokenId,spender:AccountId,amount:Balance);

    #[ink(extension = 65539, returns_result = false)]
    fn transfer(id:FungibleTokenId,recipient:AccountId,amount:Balance);

    #[ink(extension = 65540, returns_result = false)]
    fn transfer_from(id:FungibleTokenId,sender:AccountId,recipient:AccountId,amount:Balance);

    #[ink(extension = 65541, returns_result = false)]
    fn mint(id:FungibleTokenId,account:AccountId,amount:Balance) ;

    #[ink(extension = 65542, returns_result = false)]
    fn burn(id:FungibleTokenId,account:AccountId,amount:Balance);
}

/// Contract Config
#[ink::contract(env = crate::CustomEnvironment)]
pub mod rand_extension {
    use super::TokenFungibleErr;
    use ink_prelude::vec::Vec;
    use crate::{AccountId as Other_Account, Balance as Other_Balance, FungibleTokenId};


    #[ink(storage)]
    pub struct FungibleTokenExtension {
        /// Stores a single `bool` value on the storage.
        value: FungibleTokenId,
    }

    #[ink(event)]
    pub struct FungibleTokenCreate {
        #[ink(topic)]
        fungible_token_id: FungibleTokenId,
    }

    #[ink(event)]
    pub struct FungibleTokenMint {
        #[ink(topic)]
        fungible_token_id: FungibleTokenId,
    }

    impl FungibleTokenExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{value:Default::default()}
        }


        #[ink(message)]
        pub fn create_fungible_token(&mut self, name:Vec<u8>,symbol:Vec<u8>,decimals:u8) -> Result<(), TokenFungibleErr> {
            // Get the on-chain random seed
            let fungible_token_id = self.env().extension().create_token(name,symbol,decimals)?;
            self.value = fungible_token_id;
            Self::env().emit_event(FungibleTokenCreate { fungible_token_id });
            Ok(())
        }

        #[ink(message)]
        pub fn approve_fungible_token(&mut self, id:FungibleTokenId,spender:Other_Account,amount:Other_Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().approve(id,spender,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_fungible_token(&mut self, id:FungibleTokenId,recipient:Other_Account,amount:Other_Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().transfer(id,recipient,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from_fungible_token(&mut self, id:FungibleTokenId,sender:Other_Account,recipient:Other_Account,amount:Other_Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().transfer_from(id,sender,recipient,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint_fungible_token(&mut self, id:FungibleTokenId,account:Other_Account,amount:Other_Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().mint(id,account,amount)?;
            self.env().emit_event(FungibleTokenMint { fungible_token_id: self.value });
            Ok(())
        }

        #[ink(message)]
        pub fn burn_fungible_token(&mut self, id:FungibleTokenId,account:Other_Account,amount:Other_Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().burn(id,account,amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> FungibleTokenId {
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
            let fungible_token_extension = FungibleTokenExtension::default();
            assert_eq!(fungible_token_extension.get(), [0; 32]);
        }
    }
}