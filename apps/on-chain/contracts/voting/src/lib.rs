#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address};

#[contracttype]
enum DataKey {
    Admin,
    Up(Symbol),
    Down(Symbol),
}

fn get_count(env: &Env, key: DataKey) -> i64 {
    env.storage().persistent().get::<DataKey, i64>(&key).unwrap_or(0)
}

fn set_count(env: &Env, key: DataKey, value: i64) {
    env.storage().persistent().set(&key, &value);
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn init(env: Env, admin: Address) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }

    pub fn vote_up(env: Env, caller: Address, item: Symbol) {
        caller.require_auth();
        let current = get_count(&env, DataKey::Up(item.clone()));
        set_count(&env, DataKey::Up(item), current + 1);
    }

    pub fn vote_down(env: Env, caller: Address, item: Symbol) {
        caller.require_auth();
        let current = get_count(&env, DataKey::Down(item.clone()));
        set_count(&env, DataKey::Down(item), current + 1);
    }

    pub fn get_score(env: Env, item: Symbol) -> i64 {
        let up = get_count(&env, DataKey::Up(item.clone()));
        let down = get_count(&env, DataKey::Down(item));
        up - down
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage().persistent().get::<DataKey, Address>(&DataKey::Admin).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn voting_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        client.init(&admin);

        let item = Symbol::new(&env, "news1");
        client.vote_up(&user, &item);
        client.vote_up(&user, &item);
        client.vote_down(&user, &item);

        let score = client.get_score(&item);
        assert_eq!(score, 1);

        let saved_admin = client.get_admin();
        assert_eq!(saved_admin, admin);
    }
}