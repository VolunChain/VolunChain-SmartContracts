#![no_std]
extern crate alloc;

use alloc::string::ToString;

use nft_core::NFTCore;
use nft_core::{NFT, NFTError};
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Map, String, Symbol, Vec
};


#[contracttype]
pub enum DataKey {
    Reputation(Address),
    Endorsements(Address),
    Organizations,
}

#[contract]
pub struct ReputationSystem;

#[contractimpl]
impl ReputationSystem {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        let organizations: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Organizations, &organizations);
    }

    pub fn add_organization(env: Env, admin: Address, org: Address) {
        admin.require_auth();

        let mut organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap_or_else(|| Vec::new(&env));
        organizations.push_back(org);
        env.storage().instance().set(&DataKey::Organizations, &organizations);
    }

    pub fn endorse_volunteer(
        env: Env,
        org: Address,
        volunteer: Address,
        score: u32,
        category: Symbol,
    ) -> Result<u128, NFTError> {
        org.require_auth();

        let organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap_or_else(|| Vec::new(&env));
        if !organizations.contains(&org) {
            panic!("Unauthorized organization");
        }

        let endorsement_key = DataKey::Endorsements(volunteer.clone());
        let mut endorsements: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&endorsement_key)
            .unwrap_or_else(|| Map::new(&env));

        endorsements.set(org.clone(), score);
        env.storage().instance().set(&endorsement_key, &endorsements);

        let attributes = Vec::from_array(
            &env,
            [
                (
                    String::from_str(&env, "category"),
                    String::from_str(&env, &category.to_string()),
                ),
                (
                    String::from_str(&env, "score"),
                    String::from_str(&env, &score.to_string()),
                ),
            ],
        );
        

        let token_id = NFTCore::mint_nft(
            env.clone(),
            org.clone(),
            volunteer.clone(),
            String::from_str(&env, "Reputation Badge"),
            String::from_str(&env, "Endorsement received"),
            attributes,
            false,
        )?;

        Self::update_reputation(&env, &volunteer);
        Ok(token_id)
    }

    fn update_reputation(env: &Env, volunteer: &Address) {
        let mut total_score = 0u32;

        if let Some(endorsements) = env
            .storage()
            .instance()
            .get(&DataKey::Endorsements(volunteer.clone()))
        {
            let endorsements: Map<Address, u32> = endorsements;
            for (_org, score) in endorsements.iter() {
                total_score += score;
            }
        }

        let badges = NFTCore::get_nfts_by_owner(env.clone(), volunteer.clone());
        total_score += badges.len() as u32 * 10;

        env.storage()
            .instance()
            .set(&DataKey::Reputation(volunteer.clone()), &total_score);
    }

    pub fn get_reputation(env: Env, volunteer: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Reputation(volunteer))
            .unwrap_or(0)
    }

    pub fn get_organizations(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_badges(env: Env, owner: Address) -> Vec<NFT> {
        NFTCore::get_nfts_by_owner(env, owner)
    }
}
