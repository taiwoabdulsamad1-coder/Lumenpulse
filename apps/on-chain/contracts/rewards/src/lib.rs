#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
enum DataKey {
    Admin,
    Reward(Address),
}

fn get_reward(env: &Env, addr: Address) -> i128 {
    env.storage().persistent().get::<DataKey, i128>(&DataKey::Reward(addr)).unwrap_or(0)
}

fn set_reward(env: &Env, addr: Address, amount: i128) {
    env.storage().persistent().set(&DataKey::Reward(addr), &amount);
}

fn require_admin(env: &Env, caller: &Address) {
    let admin = env.storage().persistent().get::<DataKey, Address>(&DataKey::Admin).unwrap();
    caller.require_auth();
    assert!(caller == &admin, "not authorized");
}

#[contract]
pub struct RewardsContract;

#[contractimpl]
impl RewardsContract {
    pub fn init(env: Env, admin: Address) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }

    pub fn set_user_reward(env: Env, caller: Address, user: Address, amount: i128) {
        require_admin(&env, &caller);
        set_reward(&env, user, amount);
    }

    pub fn get_user_reward(env: Env, user: Address) -> i128 {
        get_reward(&env, user)
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
    fn rewards_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, RewardsContract);
        let client = RewardsContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        client.init(&admin);

        client.set_user_reward(&admin, &user, &100);
        let reward = client.get_user_reward(&user);
        assert_eq!(reward, 100);

        let saved_admin = client.get_admin();
        assert_eq!(saved_admin, admin);
    }
}