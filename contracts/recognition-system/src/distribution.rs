use crate::{minting::RecognitionBadge, DataKey};
use crate::metadata::NFTMetadata;
use soroban_sdk::{
    contract, contracterror, contracttype,
    Address, Env, String, Vec
};

#[allow(dead_code)]
pub trait RecognitionDistribution {
    fn burn_nft(env: Env, owner: Address, token_id: u32);
    fn get_volunteer_nfts(env: Env, volunteer: Address) -> Vec<u32>;
}

#[allow(dead_code)]
pub struct NFTDistribution;

impl RecognitionDistribution for NFTDistribution {
     fn burn_nft(env: Env, owner: Address, token_id: u32) {
        owner.require_auth();

        let nft: RecognitionBadge = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT non-existent");

        if nft.owner != owner {
            panic!("Unauthorized sender");
        }

        env.storage().persistent().remove(&token_id);
    }

    fn get_volunteer_nfts(env: Env, volunteer: Address) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&crate::DataKey::RecognitionBadges(volunteer))
            .unwrap_or_else(|| Vec::new(&env))
    }
}
