//! Implementation of total supply.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, Key};

use crate::{constants::MINTER_KEY_NAME, detail};

#[inline]
pub(crate) fn minter_uref() -> URef {
    detail::get_uref(MINTER_KEY_NAME)
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_minter_from(uref: URef) -> Key {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_minter_to(uref: URef, value: Key) {
    storage::write(uref, value);
}
