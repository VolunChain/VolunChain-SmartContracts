#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec};

mod distribution;
mod metadata;
mod nft_minting;

#[contracttype]
pub enum DataKey {
    Reputation(Address),   // Stores reputation score for each volunteer
    Endorsements(Address), // Stores endorsements received by volunteer
    Organizations,         // Stores authorized organizations
    Badges(Address),       // Stores NFT badges owned by volunteer
}

#[contract]
pub struct ReputationSystem;

#[contractimpl]
impl ReputationSystem {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        // Initialize authorized organizations list
        let organizations: Vec<Address> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::Organizations, &organizations);
    }

    // Add an organization to authorized list
    pub fn add_organization(env: Env, admin: Address, org: Address) {
        admin.require_auth();

        let mut organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap();
        organizations.push_back(org);
        env.storage()
            .instance()
            .set(&DataKey::Organizations, &organizations);
    }

    // Core function to endorse a volunteer (only authorized organizations)
    pub fn endorse_volunteer(
        env: Env,
        org: Address,
        volunteer: Address,
        score: u32,
        _category: Symbol,
    ) {
        org.require_auth();

        // Verify organization is authorized
        let organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap();
        if !organizations.contains(&org) {
            panic!("Unauthorized organization");
        }

        // Update endorsements
        let endorsement_key = DataKey::Endorsements(volunteer.clone());
        let mut endorsements: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&endorsement_key)
            .unwrap_or_else(|| Map::new(&env));

        endorsements.set(org, score);
        env.storage()
            .instance()
            .set(&endorsement_key, &endorsements);

        // Recalculate reputation
        Self::update_reputation(&env, &volunteer);
    }

    // Calculate and update reputation score
    fn update_reputation(env: &Env, volunteer: &Address) {
        let mut total_score = 0u32;

        // Sum endorsements
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

        // Add badge multipliers
        if let Some(badges) = env
            .storage()
            .instance()
            .get(&DataKey::Badges(volunteer.clone()))
        {
            let badges: Vec<Symbol> = badges;
            total_score += badges.len() as u32 * 10; // Each badge adds 10 points
        }

        // Store updated reputation
        env.storage()
            .instance()
            .set(&DataKey::Reputation(volunteer.clone()), &total_score);
    }

    // Query volunteer's reputation score
    pub fn get_reputation(env: Env, volunteer: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Reputation(volunteer))
            .unwrap_or(0)
    }

    // Query all organizations
    pub fn get_organizations(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap()
    }
}

#[cfg(test)]
mod test;
