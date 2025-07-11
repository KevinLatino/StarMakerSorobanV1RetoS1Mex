#![no_std]

use soroban_sdk::{contract, contractimpl, Env, symbol_short, Symbol};

const RESULT: Symbol = symbol_short!("RESULT");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    
    pub fn add(env: Env, a: i128, b: i128) -> i128 {
        let sum = a + b;
        env.storage().instance().set(&RESULT, &sum);
        return sum;
    }

    pub fn previous_result(env: Env) -> i128 {
        let stored_result = env.storage().instance().get(&RESULT).unwrap_or(0);
        return stored_result;
    }
}

mod test;
