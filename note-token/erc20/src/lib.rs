//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc20 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod address;
mod allowances;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
mod helpers;
mod total_supply;

use alloc::string::{String, ToString};

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ContractHash, EntryPoints, HashAddr,RuntimeArgs, Key, URef, U256,
};

pub use address::Address;
use constants::*;
pub use error::Error;
use helpers::*;

/// Implementation of ERC20 standard functionality.
#[derive(Default)]
pub struct ERC20 {
    balances_uref: OnceCell<URef>,
    allowances_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
}

impl ERC20 {
    fn new(
        balances_uref: URef,
        allowances_uref: URef,
        total_supply_uref: URef,
    ) -> Self {
        Self {
            balances_uref: balances_uref.into(),
            allowances_uref: allowances_uref.into(),
            total_supply_uref: total_supply_uref.into(),
        }
    }

    fn total_supply_uref(&self) -> URef {
        *self
            .total_supply_uref
            .get_or_init(total_supply::total_supply_uref)
    }

    fn read_total_supply(&self) -> U256 {
        total_supply::read_total_supply_from(self.total_supply_uref())
    }

    fn write_total_supply(&self, total_supply: U256) {
        total_supply::write_total_supply_to(self.total_supply_uref(), total_supply)
    }

    fn balances_uref(&self) -> URef {
        *self.balances_uref.get_or_init(balances::get_balances_uref)
    }

    fn read_balance(&self, owner: Address) -> U256 {
        balances::read_balance_from(self.balances_uref(), owner)
    }

    fn write_balance(&mut self, owner: Address, amount: U256) {
        balances::write_balance_to(self.balances_uref(), owner, amount)
    }

    fn allowances_uref(&self) -> URef {
        *self
            .allowances_uref
            .get_or_init(allowances::allowances_uref)
    }


    fn read_allowance(&self, owner: Address, spender: Address) -> U256 {
        allowances::read_allowance_from(self.allowances_uref(), owner, spender)
    }

    fn write_allowance(&mut self, owner: Address, spender: Address, amount: U256) {
        allowances::write_allowance_to(self.allowances_uref(), owner, spender, amount)
    }

    fn transfer_balance(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        balances::transfer_balance(self.balances_uref(), sender, recipient, amount)
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        fee: U256,
        fee_receiver: Key,
        contract_owner: Key,
    ) -> Result<ERC20, Error> {
        let default_entry_points = entry_points::default();
        ERC20::install_custom(
            name,
            symbol,
            decimals,
            initial_supply,
            ERC20_TOKEN_CONTRACT_KEY_NAME,
            default_entry_points,
            fee,
            fee_receiver,
            contract_owner,
        )
    }


    /// init function
    #[no_mangle]
    pub extern "C" fn init() {
        if helpers::named_uref_exists(CONTRACT_NAME) {
            runtime::revert(Error::ContractAlreadyInitialized);
        }    
        storage::new_dictionary("supported_token")
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
        storage::new_dictionary("supported_token_decimals")
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
        // Start collecting the runtime arguments.
        let contract_name: String = helpers::get_named_arg_with_user_errors(
            CONTRACT_NAME,
            Error::MissingContractName,
            Error::InvalidContractName,
        )
        .unwrap_or_revert();


    runtime::put_key(CONTRACT_NAME, storage::new_uref(contract_name).into());

    }

    /// Returns the name of the token.
    pub fn name(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
    }

    /// Returns the symbol of the token.
    pub fn symbol(&self) -> String {
        detail::read_from(SYMBOL_KEY_NAME)
    }

    /// Returns the decimals of the token.
    pub fn decimals(&self) -> u8 {
        detail::read_from(DECIMALS_KEY_NAME)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self) -> U256 {
        self.read_total_supply()
    }

    /// Returns the balance of `owner`.
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.read_balance(owner)
    }

    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        self.transfer_balance(sender, recipient, amount)
    }

    /// Transfers `amount` of tokens from `owner` to `recipient` if the direct caller has been
    /// previously approved to spend the specified amount on behalf of the owner.
    pub fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let spender = detail::get_immediate_caller_address()?;
        if amount.is_zero() {
            return Ok(());
        }
        let spender_allowance = self.read_allowance(owner, spender);
        let new_spender_allowance = spender_allowance
            .checked_sub(amount)
            .ok_or(Error::InsufficientAllowance)?;
        self.transfer_balance(owner, recipient, amount)?;
        self.write_allowance(owner, spender, new_spender_allowance);
        Ok(())
    }

    /// Allows `spender` to transfer up to `amount` of the direct caller's tokens.
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<(), Error> {
        let owner = detail::get_immediate_caller_address()?;
        self.write_allowance(owner, spender, amount);
        Ok(())
    }

    /// Returns the amount of `owner`'s tokens allowed to be spent by `spender`.
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.read_allowance(owner, spender)
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn deposit(
        &mut self,
        owner: Address,
        deposit_token: Key,
        amount: U256,
    ) -> Result<(), Error> {
        // Check as if deposit token was supported

        let deposit_token_dictionary_key = make_dictionary_item_key_for_contract(deposit_token);
        let enabled_value =
            get_dictionary_value_from_key::<bool>(SUPPORTED_TOKEN, &deposit_token_dictionary_key)
                .unwrap_or_revert();
        if enabled_value != true {
            runtime::revert(Error::InvalidSupportedToken);
        }

        /// TODO: Calculate NOTE to mint to users
        let deposit_token_decimals_value = get_dictionary_value_from_key::<u8>(
            SUPPORTED_TOKEN_DECIMALS,
            &deposit_token_dictionary_key,
        )
        .unwrap_or_revert_with(Error::InvalidSupportedToken);

        let this_decimals: u8 = self.decimals();

        // Transfer amount to the contract

        let caller = get_immediate_caller_key();
        let this_self_key: Key = get_self_key();
        let deposit_token_contract_hash_addr: HashAddr =
            deposit_token.into_hash().unwrap_or_revert();
        let deposit_token_contract_hash: ContractHash =
            ContractHash::new(deposit_token_contract_hash_addr);

        let _: () = runtime::call_contract(
            deposit_token_contract_hash, // deposit_token contract
            TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => caller,
                "recipient" => this_self_key,
                "amount" => amount,
            },
        );

        let maybe_output_amount =
            amount / U256::from(deposit_token_decimals_value) * U256::from(this_decimals);

        // Mint NOTE to owner
        let new_balance = {
            let balance = self.read_balance(owner);
            balance
                .checked_add(maybe_output_amount)
                .ok_or(Error::Overflow)?
        };
        let new_total_supply = {
            let total_supply: U256 = self.read_total_supply();
            total_supply
                .checked_add(maybe_output_amount)
                .ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Burn `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn redeem(&mut self, owner: Address, redeem_token: Key, amount: U256) -> Result<(), Error> {
        let this_self_key: Key = get_self_key();
        let caller = get_immediate_caller_address().unwrap_or_revert();
        if caller != owner {
            runtime::revert(Error::InvalidCaller);
        }

        // Check if overFlow
        let new_balance = {
            let balance = self.read_balance(owner);
            balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };
        let new_total_supply = {
            let total_supply = self.read_total_supply();
            total_supply.checked_sub(amount).ok_or(Error::Overflow)?
        };

        // Check as if deposit token was supported

        let redeem_token_dictionary_key = make_dictionary_item_key_for_contract(redeem_token);
        let enabled_value =
            get_dictionary_value_from_key::<bool>(SUPPORTED_TOKEN, &redeem_token_dictionary_key)
                .unwrap_or_revert();
        if enabled_value != true {
            runtime::revert(Error::InvalidSupportedToken);
        }

        // TODO: Calculate NOTE to burn to users
        let redeem_token_decimals_value = get_dictionary_value_from_key::<u8>(
            SUPPORTED_TOKEN_DECIMALS,
            &redeem_token_dictionary_key,
        )
        .unwrap_or_revert_with(Error::InvalidSupportedToken);

        let this_decimals: u8 = self.decimals();
        let maybe_output_amount =
            amount / U256::from(this_decimals) * U256::from(redeem_token_decimals_value);

        let current_fee = helpers::get_stored_value_with_user_errors::<U256>(
            "fee",
            Error::MissingFee,
            Error::InvalidFee,
        );
        let current_fee_receiver = helpers::get_stored_value_with_user_errors::<Key>(
            "fee_receiver",
            Error::MissingFeeReceiver,
            Error::InvalidFeeReceiver,
        );

        let needed_fee = maybe_output_amount * current_fee/ U256::from("1000");
        let maybe_output_amount_after_fee: U256 = maybe_output_amount - needed_fee;

        // transfer fee to receiver

        let redeem_token_contract_hash_addr: HashAddr = redeem_token.into_hash().unwrap_or_revert();
        let redeem_token_contract_hash: ContractHash =
            ContractHash::new(redeem_token_contract_hash_addr);
        let _: () = runtime::call_contract(
            redeem_token_contract_hash, // wcspr contract
            TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => this_self_key,
                "recipient" => current_fee_receiver,
                "amount" => needed_fee,
            },
        );

        // Transfer maybe_output_amount_after_fee to the contract

        let _: () = runtime::call_contract(
            redeem_token_contract_hash, // wcspr contract
            TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => this_self_key,
                "recipient" => owner,
                "amount" => maybe_output_amount_after_fee,
            },
        );

        // Burn NOTE from owner
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total
    /// supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    // pub fn burn(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
    //     let new_balance = {
    //         let balance = self.read_balance(owner);
    //         balance
    //             .checked_sub(amount)
    //             .ok_or(Error::InsufficientBalance)?
    //     };
    //     let new_total_supply = {
    //         let total_supply = self.read_total_supply();
    //         total_supply.checked_sub(amount).ok_or(Error::Overflow)?
    //     };
    //     self.write_balance(owner, new_balance);
    //     self.write_total_supply(new_total_supply);
    //     Ok(())
    // }

    /// Set supported tokens that were enabled for users to deposit into contract to mint NOTE ERC20
    /// The dictionary values
    pub fn set_supported_token(&mut self, supported_token: Key, enabled: bool) -> Result<(), Error> {
        // Check caller must be DEV account
        let caller = helpers::get_immediate_caller_key();
        let current_owner = helpers::get_stored_value_with_user_errors::<Key>(
            "contract_owner",
            Error::MissingContractOwner,
            Error::InvalidContractOwner,
        );

        if caller != current_owner {
            runtime::revert(Error::InvalidContractOwner);
        }

        // Take valid new_addresses from runtime args
        //let new_addresses_whitelist: Key = runtime::get_named_arg(ARG_NEW_ADDRESSES_WHITELIST);
        // let supported_token = helpers::get_named_arg_with_user_errors::<Key>(
        //     ARG_SUPPORTED_TOKEN,
        //     Error::MissingSupportedToken,
        //     Error::InvalidSupportedToken,
        // )
        // .unwrap_or_revert_with(Error::CannotGetWhitelistAddrressArg);

        // let enabled = helpers::get_named_arg_with_user_errors::<bool>(
        //     ARG_ENABLED,
        //     Error::MissingEnabled,
        //     Error::InvalidEnabled,
        // )
        // .unwrap_or_revert_with(Error::CannotGetEnabled);

        // Get new address if valid.
        let token_dictionary_key = helpers::make_dictionary_item_key_for_contract(supported_token);

        // Check if new_address already in dictionary with same enabled key
        if get_dictionary_value_from_key::<bool>(SUPPORTED_TOKEN, &token_dictionary_key).is_some() {
            let old_enabled: bool =
                get_dictionary_value_from_key::<bool>(SUPPORTED_TOKEN, &token_dictionary_key)
                    .unwrap_or_revert();
            if old_enabled == enabled {
                runtime::revert(Error::SameEnabledValue);
            }
        }
        // Add new_addresses into dictionary

        write_dictionary_value_from_key(SUPPORTED_TOKEN, &token_dictionary_key, enabled);
        Ok(())
    }

    pub fn set_supported_token_decimals(&mut self, supported_token: Key, decimals: u8) -> Result<(), Error> {
        // Check caller must be DEV account
        let caller = helpers::get_immediate_caller_key();
        let current_owner = helpers::get_stored_value_with_user_errors::<Key>(
            "contract_owner",
            Error::MissingContractOwner,
            Error::InvalidContractOwner,
        );

        if caller != current_owner {
            runtime::revert(Error::InvalidContractOwner);
        }

        // Take valid new_addresses from runtime args
        //let new_addresses_whitelist: Key = runtime::get_named_arg(ARG_NEW_ADDRESSES_WHITELIST);
        // let supported_token = helpers::get_named_arg_with_user_errors::<Key>(
        //     ARG_SUPPORTED_TOKEN,
        //     Error::MissingSupportedToken,
        //     Error::InvalidSupportedToken,
        // )
        // .unwrap_or_revert_with(Error::CannotGetWhitelistAddrressArg);

        // let decimals = helpers::get_named_arg_with_user_errors::<u8>(
        //     ARG_DECIMALS,
        //     Error::MissingDecimals,
        //     Error::InvalidDecimals,
        // )
        // .unwrap_or_revert_with(Error::CannotGetEnabled);

        // Get new address if valid.
        let token_dictionary_key = helpers::make_dictionary_item_key_for_contract(supported_token);

        // Check if new_address already in dictionary with same enabled key
        if get_dictionary_value_from_key::<u8>(SUPPORTED_TOKEN_DECIMALS, &token_dictionary_key)
            .is_some()
        {
            let old_decimal: u8 = get_dictionary_value_from_key::<u8>(
                SUPPORTED_TOKEN_DECIMALS,
                &token_dictionary_key,
            )
            .unwrap_or_revert();
            if old_decimal == decimals {
                runtime::revert(Error::SameDecimalsValue);
            }
        }
        // Add new_addresses into dictionary

        write_dictionary_value_from_key(SUPPORTED_TOKEN_DECIMALS, &token_dictionary_key, decimals);
        Ok(())
    }
    /// Installs the ERC20 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        contract_key_name: &str,
        entry_points: EntryPoints,
        fee: U256,
        fee_receiver: Key,
        contract_owner: Key,
    ) -> Result<ERC20, Error> {
        let balances_uref = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
        let allowances_uref = storage::new_dictionary(ALLOWANCES_KEY_NAME).unwrap_or_revert();
        // We need to hold on a RW access rights because tokens can be minted or burned.
        let total_supply_uref = storage::new_uref(initial_supply).into_read_write();

        // Add custom dictionary for NOTE ERC20
        let mut named_keys = NamedKeys::new();

        let name_key = {
            let name_uref = storage::new_uref(name).into_read();
            Key::from(name_uref)
        };

        let symbol_key = {
            let symbol_uref = storage::new_uref(symbol).into_read();
            Key::from(symbol_uref)
        };

        let decimals_key = {
            let decimals_uref = storage::new_uref(decimals).into_read();
            Key::from(decimals_uref)
        };

        let total_supply_key = Key::from(total_supply_uref);

        let balances_dictionary_key = {
            // Sets up initial balance for the caller - either an account, or a contract.
            let caller = detail::get_caller_address()?;
            balances::write_balance_to(balances_uref, caller, initial_supply);

            runtime::remove_key(BALANCES_KEY_NAME);

            Key::from(balances_uref)
        };

        let allowances_dictionary_key = {
            runtime::remove_key(ALLOWANCES_KEY_NAME);

            Key::from(allowances_uref)
        };


        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY_NAME.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY_NAME.to_string(), decimals_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY_NAME.to_string(), allowances_dictionary_key);
        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);
        // Add custom named_keys for NOTE ERC20
        named_keys.insert("fee".to_string(), storage::new_uref(fee).into());
        named_keys.insert(
            "fee_receiver".to_string(),
            Key::from(storage::new_uref(fee_receiver)),
        );
        named_keys.insert(
            "contract_owner".to_string(),
            Key::from(storage::new_uref(contract_owner)),
        );

        let (contract_hash, _version) =
            storage::new_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));
         // Call contract to initialize it
        runtime::call_contract::<()>(
            contract_hash,
            ENTRY_POINT_INIT,
            runtime_args! {
                CONTRACT_NAME => "this_is_note",
            },
        );

        Ok(ERC20::new(
            balances_uref,
            allowances_uref,
            total_supply_uref,
        ))
    }
}
