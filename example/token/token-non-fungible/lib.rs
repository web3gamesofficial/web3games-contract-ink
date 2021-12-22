#![cfg_attr(not(feature = "std"), no_std)]
/// This is an example of how an ink! contract may call the Web3Games

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;

pub type NonFungibleTokenId = u32;
pub type TokenId = u32;
pub type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
pub type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;



/// Error Config
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NonFungibleErr {
    FailCreateToken,
}


impl ink_env::chain_extension::FromStatusCode for NonFungibleErr {
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
    type ErrorCode = NonFungibleErr;

    #[ink(extension = 65601, returns_result = false)]
    fn create_token(name:Vec<u8>,symbol:Vec<u8>,base_uri:Vec<u8>) -> NonFungibleTokenId;

    #[ink(extension = 65602, returns_result = false)]
    fn approve(id:NonFungibleTokenId,to:AccountId,token_id:TokenId);

    #[ink(extension = 65603, returns_result = false)]
    fn set_approve_for_all(id:NonFungibleTokenId,operator:AccountId,approved:bool);

    #[ink(extension = 65604, returns_result = false)]
    fn transfer_from(id:NonFungibleTokenId,from:AccountId,to:AccountId,token_id:TokenId);

    #[ink(extension = 65605, returns_result = false)]
    fn mint(id:NonFungibleTokenId,to:AccountId,token_id:TokenId) ;

    #[ink(extension = 65606, returns_result = false)]
    fn burn(id:NonFungibleTokenId,token_id:TokenId);
}

/// Contract Config
#[ink::contract(env = crate::CustomEnvironment)]
pub mod rand_extension {
    use super::*;
    use ink_prelude::vec::Vec;



    #[ink(storage)]
    pub struct NonFungibleTokenExtension {
        /// Stores a single `bool` value on the storage.
        value: NonFungibleTokenId,
    }

    #[ink(event)]
    pub struct NonFungibleTokenCreate {
        #[ink(topic)]
        non_fungible_token_id: NonFungibleTokenId,
    }

    #[ink(event)]
    pub struct NonFungibleTokenMint {
        #[ink(topic)]
        non_fungible_token_id: NonFungibleTokenId,
    }

    impl NonFungibleTokenExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{value:Default::default()}
        }


        #[ink(message)]
        pub fn create_non_fungible_token(&mut self, name:Vec<u8>,symbol:Vec<u8>,base_uri:Vec<u8>) -> Result<(), NonFungibleErr> {
            // Get the on-chain random seed
            let non_fungible_token_id = self.env().extension().create_token(name,symbol,base_uri)?;
            self.value = non_fungible_token_id;
            Self::env().emit_event(NonFungibleTokenCreate { non_fungible_token_id });
            Ok(())
        }

        #[ink(message)]
        pub fn approve_non_fungible_token(&mut self, id:NonFungibleTokenId,to:AccountId,token_id:TokenId) -> Result<(), NonFungibleErr> {
            self.env().extension().approve(id,to,token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn approve_all_non_fungible_token(&mut self, id:NonFungibleTokenId,operator:AccountId,approved:bool) -> Result<(), NonFungibleErr> {
            self.env().extension().set_approve_for_all(id,operator,approved)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from_non_fungible_token(&mut self, id:NonFungibleTokenId,from:AccountId,to:AccountId,token_id:TokenId) -> Result<(), NonFungibleErr> {
            self.env().extension().transfer_from(id,from,to,token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint_non_fungible_token(&mut self, id:NonFungibleTokenId,to:AccountId,token_id:TokenId) -> Result<(), NonFungibleErr> {
            self.env().extension().mint(id,to,token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn_non_fungible_token(&mut self, id:NonFungibleTokenId,token_id:TokenId) -> Result<(), NonFungibleErr> {
            self.env().extension().burn(id,token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> NonFungibleTokenId {
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