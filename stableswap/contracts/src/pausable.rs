use crate::helpers::{self, *};
use crate::constants::*;
use crate::owner::*;
use crate::error::Error;

pub(crate) fn get_paused() -> bool {
    let paused: bool = helpers::get_key(PAUSED).unwrap();
    paused

}

pub(crate) fn set_paused(paused: bool) {
    only_owner();
    helpers::set_key(PAUSED, paused);
}

pub(crate) fn when_not_paused() {
    require(!get_paused(), Error::ContractPaused);
}