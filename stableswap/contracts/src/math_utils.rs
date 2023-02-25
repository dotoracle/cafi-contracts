pub fn within1(a: u128, b: u128) -> bool {
    difference(a, b) <= 1
}

pub fn difference(a: u128, b: u128) -> u128 {
    if a > b {
       return a - b
    }
    b - a
}