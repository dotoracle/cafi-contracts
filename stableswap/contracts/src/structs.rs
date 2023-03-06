use alloc::{
    vec::*,
};

use casper_types::{
    Key, CLType, CLTyped, bytesrepr::{ self, ToBytes, FromBytes}, U128
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Swap {
    pub initial_a: u128,
    pub future_a: u128,
    pub initial_a_time: u64,
    pub future_a_time: u64,
    pub swap_fee: u64,
    pub admin_fee: u64,
    pub lp_token: Key,
    pub pooled_tokens: Vec<Key>,
    pub token_precision_multipliers: Vec<u128>,
    pub balances: Vec<u128>
}

impl ToBytes for Swap {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = bytesrepr::allocate_buffer(self)?;
        result.extend(U128::from(self.initial_a).to_bytes()?);
        result.extend(U128::from(self.future_a).to_bytes()?);
        result.extend(self.initial_a_time.to_bytes()?);
        result.extend(self.future_a_time.to_bytes()?);
        result.extend(self.swap_fee.to_bytes()?);
        result.extend(self.admin_fee.to_bytes()?);
        result.extend(self.lp_token.to_bytes()?);
        result.extend(self.pooled_tokens.to_bytes()?);
        result.extend(self.token_precision_multipliers.clone().into_iter().map(|x| U128::from(x)).collect::<Vec<U128>>().to_bytes()?);
        result.extend(self.balances.clone().into_iter().map(|x| U128::from(x)).collect::<Vec<U128>>().to_bytes()?);
        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        U128::from(self.initial_a).serialized_length()
            + U128::from(self.future_a).serialized_length()
            + self.initial_a_time.serialized_length()
            + self.future_a_time.serialized_length()
            + self.swap_fee.serialized_length()
            + self.admin_fee.serialized_length()
            + self.lp_token.serialized_length()
            + self.pooled_tokens.serialized_length()
            + self.token_precision_multipliers.clone().into_iter().map(|x| U128::from(x)).collect::<Vec<U128>>().serialized_length()
            + self.balances.clone().into_iter().map(|x| U128::from(x)).collect::<Vec<U128>>().serialized_length()
    }
}

impl FromBytes for Swap {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (initial_a, remainder) = U128::from_bytes(bytes)?;
        let initial_a = initial_a.as_u128();
        let (future_a, remainder) = U128::from_bytes(remainder)?;
        let future_a = future_a.as_u128();
        let (initial_a_time, remainder) = u64::from_bytes(remainder)?;
        let (future_a_time, remainder) = u64::from_bytes(remainder)?;
        let (swap_fee, remainder) = u64::from_bytes(remainder)?;
        let (admin_fee, remainder) = u64::from_bytes(remainder)?;
        let (lp_token, remainder) = Key::from_bytes(remainder)?;
        let (pooled_tokens, remainder) = Vec::<Key>::from_bytes(remainder)?;
        let (token_precision_multipliers, remainder) = Vec::<U128>::from_bytes(remainder)?;
        let token_precision_multipliers = token_precision_multipliers.into_iter().map(|x| x.as_u128()).collect::<Vec<u128>>();
        let (balances, remainder) = Vec::<U128>::from_bytes(remainder)?;
        let balances = balances.into_iter().map(|x| x.as_u128()).collect::<Vec<u128>>();

        let ret = Swap {
            initial_a,
            future_a,
            initial_a_time,
            future_a_time, //token seller
            swap_fee,  // min price in WCSPR
            admin_fee,
            lp_token,
            pooled_tokens,
            token_precision_multipliers,
            balances
        };
        Ok((ret, remainder))
    }
}

impl CLTyped for Swap {
    fn cl_type() -> CLType {
        CLType::Any
    }
}


pub(crate) struct CalculateWithdrawOneTokenDYInfo {
    pub d0: u128,
    pub d1: u128,
    pub new_y: u128,
    pub fee_per_token: u128,
    pub precise_a: u128
}

pub(crate) struct ManageLiquidityInfo {
    pub d0: u128,
    pub d1: u128,
    pub d2: u128,
    pub precise_a: u128,
    pub lp_token: Key,
    pub total_supply: u128,
    pub balances: Vec<u128>,
    pub multipliers: Vec<u128>
}