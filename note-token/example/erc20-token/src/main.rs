#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod detail;
mod error;
extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{constants::*, Address, ERC20};
use casper_types::{CLValue, Key, U256};

use crate::error::ErrorERC20;

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn deposit() {
    let owner: Address = detail::get_named_arg_with_user_errors::<Address>(
        OWNER_RUNTIME_ARG_NAME,
        ErrorERC20::MissingOwner,
        ErrorERC20::InvalidOwner,
    )
    .unwrap_or_revert();

    let amount: U256 = detail::get_named_arg_with_user_errors::<U256>(
        AMOUNT_RUNTIME_ARG_NAME,
        ErrorERC20::MissingMintAmount,
        ErrorERC20::InvalidMintAmount,
    )
    .unwrap_or_revert();

    let deposit_token: Key = detail::get_named_arg_with_user_errors::<Key>(
        DEPOSIT_TOKEN_RUNTIME_ARG_NAME,
        ErrorERC20::MissingDepositToken,
        ErrorERC20::InvalidDepositToken,
    )
    .unwrap_or_revert();

    ERC20::default()
        .deposit(owner,deposit_token, amount)
        .unwrap_or_revert_with(ErrorERC20::FailCallToMint);
}

#[no_mangle]
pub extern "C" fn redeem() {
    let owner: Address = detail::get_named_arg_with_user_errors::<Address>(
        OWNER_RUNTIME_ARG_NAME,
        ErrorERC20::MissingOwner,
        ErrorERC20::InvalidOwner,
    )
    .unwrap_or_revert();

    let amount: U256 = detail::get_named_arg_with_user_errors::<U256>(
        AMOUNT_RUNTIME_ARG_NAME,
        ErrorERC20::MissingMintAmount,
        ErrorERC20::InvalidMintAmount,
    )
    .unwrap_or_revert();
    let redeem_token: Key = detail::get_named_arg_with_user_errors::<Key>(
        REDEEM_TOKEN_RUNTIME_ARG_NAME,
        ErrorERC20::MissingRedeemToken,
        ErrorERC20::InvalidRedeemToken,
    )
    .unwrap_or_revert();

    ERC20::default()
        .redeem(owner, redeem_token, amount)
        .unwrap_or_revert_with(ErrorERC20::FailCallToBurn);
}


#[no_mangle]
pub extern "C" fn set_fee() {

    let fee: U256 = detail::get_named_arg_with_user_errors::<U256>(
        ARG_FEE,
        ErrorERC20::MissingFee,
        ErrorERC20::InvalidFee,
    )
    .unwrap_or_revert();

    ERC20::default()
        .set_fee(fee)
        .unwrap_or_revert_with(ErrorERC20::FailCallToBurn);
}


#[no_mangle]
pub extern "C" fn set_supported_token() {

    let enable: bool = detail::get_named_arg_with_user_errors::<bool>(
        ARG_ENABLED,
        ErrorERC20::MissingEnabled,
        ErrorERC20::InvalidEnabled,
    )
    .unwrap_or_revert();
    let supported_token: Key = detail::get_named_arg_with_user_errors::<Key>(
        ARG_SUPPORTED_TOKEN,
        ErrorERC20::MissingSupportedToken,
        ErrorERC20::InvalidSupportedToken,
    )
    .unwrap_or_revert();

    ERC20::default()
        .set_supported_token(supported_token, enable)
        .unwrap_or_revert_with(ErrorERC20::FailCallToBurn);
}

#[no_mangle]
pub extern "C" fn set_supported_token_decimals() {

    let decimals: u8 = detail::get_named_arg_with_user_errors::<u8>(
        ARG_DECIMALS,
        ErrorERC20::MissingDecimals,
        ErrorERC20::InvalidDecimals,
    )
    .unwrap_or_revert();
    let supported_token: Key = detail::get_named_arg_with_user_errors::<Key>(
        ARG_SUPPORTED_TOKEN,
        ErrorERC20::MissingSupportedToken,
        ErrorERC20::InvalidSupportedToken,
    )
    .unwrap_or_revert();

    ERC20::default()
        .set_supported_token_decimals(supported_token, decimals)
        .unwrap_or_revert_with(ErrorERC20::FailCallToBurn);
}


#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);
    let fee: U256 = runtime::get_named_arg(FEE);
    let fee_receiver: Key = runtime::get_named_arg(FEE_RECEIVER);
    let contract_owner: Key = runtime::get_named_arg(CONTRACT_OWNER);

    let _token = ERC20::install(
        name,
        symbol,
        decimals,
        total_supply,
        fee,
        fee_receiver,
        contract_owner,
    )
    .unwrap_or_revert();
}
