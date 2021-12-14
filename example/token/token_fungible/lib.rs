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
}