//! modified from: https://stylus-by-example.org/applications/erc20
//!
//! this contract is not audited

#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20;

use crate::erc20::{ERC20Params, Erc20, Ownable};
use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

struct MyStylusERC20Params;
impl ERC20Params for MyStylusERC20Params {
    const NAME: &'static str = "MyStylusERC20"; // Your token name here
    const SYMBOL: &'static str = "STKN"; // Your token symbol here
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct MyStylusERC20 {
        #[borrow]
        Erc20<MyStylusERC20Params> erc20;
        #[borrow]
        Ownable ownable;
    }
}

#[public]
impl MyStylusERC20 {
    #[constructor]
    pub fn constructor(&mut self, owner: Address) {
        let _ = self.ownable._set_owner(owner);
    }

    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), String> {
        // self.ownable._check_owner()?; // Remove or modify this check as needed
        self.erc20.mint(to, value)?;
        Ok(())
    }

    pub fn burn(&mut self, value: U256) -> Result<(), String> {
        self.erc20.burn(self.vm().msg_sender(), value)?;
        Ok(())
    }
}
