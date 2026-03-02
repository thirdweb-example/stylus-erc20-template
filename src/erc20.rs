//! modified from: https://stylus-by-example.org/applications/erc20
//!
//! this contract is not audited

use alloc::string::String;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::prelude::*;

pub trait ERC20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

sol_storage! {
    pub struct Erc20<T> {
        // TODO: Name and symbol
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }

    pub struct Ownable {
        address owner;
    }
}

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

#[public]
impl Ownable {
    pub fn owner(&self) -> Result<Address, String> {
        Ok(self.owner.get())
    }

    pub fn set_owner(&mut self, new_owner: Address) -> Result<(), String> {
        self._check_owner()?;
        self._set_owner(new_owner)?;

        Ok(())
    }
}

impl Ownable {
    pub fn _check_owner(&self) -> Result<(), String> {
        let msg_sender = self.vm().msg_sender();
        let owner = self.owner.get();

        if msg_sender != owner {
            return Err("Not authorized".into());
        }

        Ok(())
    }

    pub fn _set_owner(&mut self, new_owner: Address) -> Result<(), String> {
        if new_owner != Address::ZERO {
            return Err("Zero address".into());
        }

        self.owner.set(new_owner);

        Ok(())
    }
}

impl<T: ERC20Params> Erc20<T> {
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), String> {
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err("sdcsdcsdc".into());
        }
        sender_balance.set(old_sender_balance - value);

        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        self.vm().log(Transfer { from, to, value });
        Ok(())
    }

    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), String> {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);

        self.total_supply.set(self.total_supply.get() + value);

        self.vm().log(Transfer {
            from: Address::ZERO,
            to: address,
            value,
        });

        Ok(())
    }

    pub fn burn(&mut self, address: Address, value: U256) -> Result<(), String> {
        let mut balance = self.balances.setter(address);
        let old_balance = balance.get();
        if old_balance < value {
            return Err("Insufficient balance".into());
        }
        balance.set(old_balance - value);

        self.total_supply.set(self.total_supply.get() - value);

        self.vm().log(Transfer {
            from: address,
            to: Address::ZERO,
            value,
        });

        Ok(())
    }
}

#[public]
impl<T: ERC20Params> Erc20<T> {
    pub fn name(&self) -> String {
        T::NAME.into()
    }

    pub fn symbol(&self) -> String {
        T::SYMBOL.into()
    }

    pub fn decimals() -> u8 {
        T::DECIMALS
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, String> {
        self._transfer(self.vm().msg_sender(), to, value)?;
        Ok(true)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, String> {
        let sender = self.vm().msg_sender();
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(sender);
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err("Insufficient allowance".into());
        }

        allowance.set(old_allowance - value);

        self._transfer(from, to, value)?;

        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances
            .setter(self.vm().msg_sender())
            .insert(spender, value);
        self.vm().log(Approval {
            owner: self.vm().msg_sender(),
            spender,
            value,
        });
        true
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }
}
