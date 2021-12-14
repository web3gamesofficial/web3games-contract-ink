#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;
pub use token_fungbile::*;


/// Contract Config
#[ink::contract(env = crate::CustomEnvironment)]
mod rand_extension {
    use super::TokenFungibleErr;
    use ink_prelude::vec::Vec;
    use crate::{AccountId, Balance, FungibleTokenId};

    #[ink(storage)]
    pub struct FungibleTokenExtension {
        /// Stores a single `bool` value on the storage.
        value: FungibleTokenId,
    }

    #[ink(event)]
    pub struct FungibleToken {
        #[ink(topic)]
        create_fungible_token: FungibleTokenId,
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
            self.env().emit_event(FungibleToken { create_fungible_token: fungible_token_id });
            Ok(())
        }

        #[ink(message)]
        pub fn approve_fungible_token(&mut self, id:FungibleTokenId,spender:AccountId,amount:Balance) -> Result<(), TokenFungibleErr> {
            self.env().extension().approve(id,spender,amount)?;
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