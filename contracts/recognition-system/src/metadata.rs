use crate::minting::RecognitionBadge;
use soroban_sdk::{Address, Env, String, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub ev_org: Address,
    pub ev_title: String,
    pub ev_date: String,
    pub task: String,
}

#[allow(dead_code)]
impl NFTMetadata {
    pub fn new(env: &Env, organization: Address, title: String, date: String, task: String) -> Self {
        Self {
            ev_org: organization,
            ev_title: title,
            ev_date: date,
            task,
        }
    }

    pub fn update_metadata(env: &Env, admin: Address, token_id: u32, organization: Address, title: String, date: String, task: String) {
        // Check that admin is authorized
        admin.require_auth();

        // Get the existing NFT
        let mut nft: RecognitionBadge = env
            .storage() 
            .persistent()
            .get(&token_id)
            .expect("NFT Token ID Invalid");

        // Assign updated event fields
        nft.metadata.ev_title = title;
        nft.metadata.ev_date = date;
        nft.metadata.ev_org = organization;
        nft.metadata.task = task;

        env.storage().persistent().set(&token_id, &nft);
    }

    pub fn get_metadata(env: &Env, token_id: u32) -> NFTMetadata {
        let nft: RecognitionBadge = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT Token ID Invalid");
        nft.metadata
    }
}
