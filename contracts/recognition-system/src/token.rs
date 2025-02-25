use crate::metadata::NFTMetadata;
use crate::DataKey;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol, Vec};

pub trait RecognitionBadgeMintBurn {}

#[contract]
pub struct RecognitionBadge;

#[contractimpl]
impl RecognitionBadge {
    pub fn mint_recognition_badge(env: Env, to: Address, event_id: u32, task: String) -> u32 {
        to.require_auth();

        let mut current_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::TokenCounter)
            .unwrap();
        current_id += 1;
        env.storage()
            .instance()
            .set(&DataKey::TokenCounter, &current_id);

        let metadata = NFTMetadata::new(env, &event_id, to, task);
        env.storage().persistent().set(&current_id, &metadata);

        let mut badges = env
            .storage()
            .instance()
            .get(&DataKey::Badges(to.clone()))
            .unwrap_or_else(|| Vec::new(&env));

        badges.push_back(current_id);

        env.storage()
            .instance()
            .set(&DataKey::Badges(to.clone()), &badges);

        current_id
    }

    pub fn burn_nft(env: Env, owner: Address, token_id: u32) {
        owner.require_auth();

        let nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT not exist");
        if nft.owner != owner {
            panic!("Unauthorized sender");
        }

        env.storage().persistent().remove(&token_id);
    }

    pub fn get_volunteer_nfts(env: Env, volunteer: Address) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&crate::DataKey::Badges(volunteer))
            .unwrap_or_else(|| Vec::new(&env))
    }
}
