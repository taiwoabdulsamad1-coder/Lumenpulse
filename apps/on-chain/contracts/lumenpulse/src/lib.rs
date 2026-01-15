#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, String, symbol};

const ADMIN: Symbol = symbol!("ADMIN");
const GREETING: Symbol = symbol!("GREETING");

#[contract]
pub struct LumenpulseContract;

#[contractimpl]
impl LumenpulseContract {
    // Initialize the contract with an admin address
    pub fn init(env: Env, admin: Address) {
        if env.storage().persistent().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().persistent().set(&ADMIN, &admin);
    }

    // Set a greeting (admin-only)
    pub fn set_greeting(env: Env, caller: Address, greeting: String) {
        Self::require_admin(&env, &caller);
        env.storage().persistent().set(&GREETING, &greeting);
    }

    // Get the current greeting
    pub fn get_greeting(env: Env) -> Option<String> {
        env.storage().persistent().get::<String>(&GREETING)
    }

    // Store a value by key (admin-only)
    pub fn set_value(env: Env, caller: Address, key: Symbol, value: i128) {
        Self::require_admin(&env, &caller);
        env.storage().persistent().set(&key, &value);
    }

    // Read a value by key
    pub fn get_value(env: Env, key: Symbol) -> Option<i128> {
        env.storage().persistent().get::<i128>(&key)
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .persistent()
            .get::<Address>(&ADMIN)
            .expect("not initialized");
        env.require_auth(caller);
        if &admin != caller {
            panic!("not authorized");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn basic_flow() {
        let env = Env::default();
        let admin = Address::generate(&env);
        LumenpulseContract::init(env.clone(), admin.clone());

        // set greeting
        LumenpulseContract::set_greeting(env.clone(), admin.clone(), String::from_str(&env, "Hello, Stellar!"));
        let g = LumenpulseContract::get_greeting(env.clone()).unwrap();
        assert_eq!(g, String::from_str(&env, "Hello, Stellar!"));

        // set/get value
        let key = symbol!("EXAMPLE");
        LumenpulseContract::set_value(env.clone(), admin.clone(), key, 42);
        assert_eq!(LumenpulseContract::get_value(env.clone(), symbol!("EXAMPLE")), Some(42));
    }
}