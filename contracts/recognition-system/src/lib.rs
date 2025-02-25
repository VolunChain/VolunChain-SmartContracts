#![no_std]
use crate::reputation_system;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol, Vec,
};

mod event;
mod metadata;
mod token;

#[cfg(test)]
mod test;

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Badges(Address),
    Recognition(Address),
    Events,
    EventCounter,
    TokenCounter,
}

#[contract]
pub struct RecognitionSystem;

#[contractimpl]
impl RecognitionSystem {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        // Initialize authorized events list
        let events: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Events, &events);
    }

    pub fn verify_confirmed_volunteer(env: Env, volunteer: Address, org: Address) -> bool {
        // Check org is authorized
        let organizations = reputation_system::ReputationSystem::get_organizations(&env);
        if !organizations.contains(&org) {
            return false;
        }

        // Check volunteer endorsements from the org
        if let Some(endorsements) = env
            .storage()
            .instance()
            .get(&reputation_system::DataKey::Endorsements(volunteer.clone()))
        {
            let endorsements: soroban_sdk::Map<Address, u32> = endorsements;
            endorsements.contains_key(org)
        } else {
            false
        }
    }
}
