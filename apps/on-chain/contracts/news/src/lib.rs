#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String, symbol, Symbol};

const ADMIN: Symbol = symbol!("ADMIN");

#[contract]
pub struct NewsContract;

#[contractimpl]
impl NewsContract {
    // Initialize the contract with an admin address
    pub fn init(env: Env, admin: Address) {
        if env.storage().persistent().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().persistent().set(&ADMIN, &admin);
    }

    // Post a news item (admin-only). Uses id as key prefix.
    pub fn post_news(env: Env, caller: Address, id: String, title: String, body: String) {
        Self::require_admin(&env, &caller);
        let title_key = Self::mk_key(&env, &id, "title");
        let body_key = Self::mk_key(&env, &id, "body");
        env.storage().persistent().set(&title_key, &title);
        env.storage().persistent().set(&body_key, &body);
    }

    // Retrieve a news item by id
    pub fn get_news(env: Env, id: String) -> Option<(String, String)> {
        let title_key = Self::mk_key(&env, &id, "title");
        let body_key = Self::mk_key(&env, &id, "body");
        let title: Option<String> = env.storage().persistent().get::<String>(&title_key);
        let body: Option<String> = env.storage().persistent().get::<String>(&body_key);
        match (title, body) {
            (Some(t), Some(b)) => Some((t, b)),
            _ => None,
        }
    }

    // Internal helper to build per-news keys
    fn mk_key(env: &Env, id: &String, field: &str) -> String {
        let composed = format!("NEWS:{}:{}", id.to_string(), field);
        String::from_str(env, &composed)
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
    fn post_and_get_news() {
        let env = Env::default();
        let admin = Address::generate(&env);
        NewsContract::init(env.clone(), admin.clone());

        let id = String::from_str(&env, "news-1");
        let title = String::from_str(&env, "Breaking: Soroban in Lumenpulse");
        let body = String::from_str(&env, "We migrated to Stellar's Soroban!");

        NewsContract::post_news(env.clone(), admin.clone(), id.clone(), title.clone(), body.clone());
        let fetched = NewsContract::get_news(env.clone(), id).unwrap();
        assert_eq!(fetched.0, title);
        assert_eq!(fetched.1, body);
    }
}