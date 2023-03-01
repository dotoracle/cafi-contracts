use casper_types::U256;
pub fn within1(a: u128, b: u128) -> bool {
    difference(a, b) <= 1
}

pub fn difference(a: u128, b: u128) -> u128 {
    if a > b {
       return a - b
    }
    b - a
}

pub fn mul_div(a: u128, b: u128, c: u128) -> u128 {
    (U256::from(a) * U256::from(b) / U256::from(c)).as_u128()
}