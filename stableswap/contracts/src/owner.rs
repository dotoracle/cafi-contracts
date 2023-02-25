use crate::helpers::{self, *};
use crate::constants::*;
use casper_types::Key;
use crate::error::Error;

pub(crate) fn only_owner() {
    require(owner() == helpers::get_immediate_caller_key(), Error::OnlyOwnerCanRevoke);
}

pub(crate)

fn owner() -> Key {
    let owner_key: Key = helpers::get_key(CONTRACT_OWNER_KEY_NAME).unwrap();
    owner_key
}