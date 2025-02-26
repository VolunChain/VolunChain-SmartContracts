use crate::DataKey;
use crate::metadata::NFTMetadata;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, TryFromVal, Vec
};

pub trait RecognitionBadgeMinting {
    fn mint_recognition_badge(env: &Env, recipient: Address, organization: Address, title: String, date: String, task: String) -> u32;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecognitionBadge {
    pub owner: Address,
    pub metadata: NFTMetadata,
}

#[allow(dead_code)]
pub struct NFTMinting;

impl RecognitionBadgeMinting for NFTMinting {
    fn mint_recognition_badge(env: &Env, recipient: Address, organization: Address, title: String, date: String, task: String) -> u32 {
        // Authenticate caller == token minter
        recipient.require_auth();

        // Get curernt token ID and increment token counter 
        let mut current_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::TokenCounter)
            .unwrap();
        current_id += 1;
        env.storage()
            .instance()
            .set(&DataKey::TokenCounter, &current_id);

        let metadata = NFTMetadata::new(&env, organization, title, date, task);
        let nft = RecognitionBadge {
            owner: recipient.clone(),
            metadata,
        };

        // env.storage().persistent().set(&current_id, &nft);

        let mut badges: Vec<RecognitionBadge> = env
            .storage()
            .instance()
            .get(&DataKey::RecognitionBadges(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));

        badges.push_back(nft);

        env.storage()
            .instance()
            .set(&DataKey::RecognitionBadges(recipient.clone()), &badges);


        current_id
    }
}
