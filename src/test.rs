#![cfg(test)]

use soroban_sdk::{Env, U256};

extern crate std;

#[test]
fn test() {
    let env: Env = Env::default();
    let og_256: U256 = U256::from_u128(&env, 200000000000000000000000000000);
    // `200000000000000000000000000000` power of 3 should be equal to `8e+60` which
    // should be able to fit inside U256
    // the code however panics
    let _power_of_3 = og_256.pow(3);
}
