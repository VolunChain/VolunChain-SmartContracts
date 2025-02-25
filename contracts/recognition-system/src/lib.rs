#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Map, Symbol, symbol_short, Vec,
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
    
    pub fn mint_nft_with_event(env: Env, volunteer: Address, event_id: u64, task: String) {
        let event = env
            .storage()
            .persistent()
            .get(&event_id)
            .expect("Event does not exist");

        crate::token::ReputationBadge::mint_nft(
            env,
            volunteer,
            event.title,
            event.date,
            event.organization,
            task,
        );
    }
}