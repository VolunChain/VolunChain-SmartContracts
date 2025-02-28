use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn initialize(env: Env, admin: Address) {
    admin.require_auth();

    // Initialize authorized organizations list
    let organizations: Vec<Address> = Vec::new(&env);
    env.storage()
        .instance()
        .set(&DataKey::Organizations, &organizations);

    // Initialize transaction ledger
    let transactions: Vec<Transaction> = Vec::new(&env);
    env.storage()
        .instance()
        .set(&DataKey::Transactions, &transactions);

    // Initialize project counter
    env.storage().instance().set(&DataKey::NextProjectId, &1u32);
}

pub fn add_organization(env: Env, admin: Address, org: Address) {
    admin.require_auth();

    let mut organizations: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::Organizations)
        .unwrap_or_else(|| Vec::new(&env));

    organizations.push_back(org);
    env.storage()
        .instance()
        .set(&DataKey::Organizations, &organizations);
}
