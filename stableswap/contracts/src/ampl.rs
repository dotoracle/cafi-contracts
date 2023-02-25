use crate::events::CAFIDexEvent;
use crate::events;
pub const A_PRECISION: u128 = 100;
pub const MAX_A: u128 = 1000000;    //10^6
pub const MAX_A_CHANGE: u128 = 2;
pub const MIN_RAMP_TIME: u64 = 86400 * 14;

use crate::structs::Swap;
use crate::helpers::{current_block_timestamp, require};
use crate::error::Error;
pub fn get_a(swap: &mut Swap) -> u128 {
    _get_a_precise(swap) / A_PRECISION
}

pub fn get_a_precise(swap: &mut Swap) -> u128 {
    _get_a_precise(swap)
}

fn _get_a_precise(swap: &mut Swap) -> u128 {
    let t1 = swap.future_a_time;
    let a1 = swap.future_a;
    let timestamp = current_block_timestamp();

    if timestamp < t1 {
        let t0 = swap.initial_a_time;
        let a0 = swap.initial_a;
        if a1 > a0 {
            return a0 + (a1 - a0) * ((timestamp - t0) as u128) / ((t1 - t0) as u128)
        } else {
            return 
            a0 - (a0 - a1) * ((timestamp - t0) as u128) / ((t1 - t0) as u128)
        }
    }
    a1
}

pub fn ramp_a(swap: &mut Swap, future_a_: u128, future_time_: u64) {
    let timestamp = current_block_timestamp();
    require(timestamp >= swap.initial_a_time + 86400, Error::WaitBeforeRamp);

    require(future_time_ >= timestamp + MIN_RAMP_TIME, Error::InsufficientRampTime);

    require(future_a_ > 0 && future_a_ < MAX_A, Error::FutureAMustBeGood);

    let initial_a_precise = _get_a_precise(swap);
    let future_a_precise = future_a_ * A_PRECISION;

    if future_a_precise < initial_a_precise {
        require(future_a_precise * MAX_A_CHANGE >= initial_a_precise, Error::FutureATooSmall);
    } else {
        require(future_a_precise <= initial_a_precise * MAX_A_CHANGE, Error::FutureATooLarge);
    }

    swap.initial_a = initial_a_precise;
    swap.future_a = future_a_precise;
    swap.initial_a_time = timestamp;
    swap.future_a_time = future_time_;

    events::emit(&CAFIDexEvent::RampA {
        old_a: initial_a_precise,
        new_a: future_a_precise,
        initial_time: timestamp,

        future_time: future_time_
    });
}

pub fn stop_ramp_a(swap: &mut Swap) {
    let timestamp = current_block_timestamp();
    require(swap.future_a_time >= timestamp, Error::RampAlreadyStopped);

    let curent_a = _get_a_precise(swap);
    swap.initial_a = curent_a;
    swap.future_a = curent_a;
    swap.initial_a_time = timestamp;
    swap.future_a_time = timestamp;

    events::emit(&CAFIDexEvent::StopRampA {
        current_a: curent_a,
        time: timestamp
    });
}