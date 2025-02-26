#![no_std]
use reputation_system;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, String, Symbol, Vec,
};

mod distribution;
mod metadata;
mod minting;

#[cfg(test)]
mod test;

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    RecognitionBadges(Address),
    Recognition(Address),
    TokenCounter,
}

#[contract]
pub struct RecognitionSystem;

#[contractimpl]
impl RecognitionSystem {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        // TODO: Initialize token counter in Datakey
    }

    pub fn verify_confirmed_volunteer(env: &Env, volunteer: Address, org: Address) -> bool {
        // Check org is authorized
        let organizations = reputation_system::ReputationSystem::get_organizations(env.clone());
        if !organizations.contains(&org) {
            return false;
        }

         // Check volunteer endorsements from the org
        let endorsement_key = &reputation_system::DataKey::Endorsements(volunteer.clone());
        let endorsements: Map<Address, u32> = env
            .storage()
            .instance()
            .get(endorsement_key)
            .unwrap_or_else(|| Map::new(&env));
        
        endorsements.contains_key(org)
    }
}
