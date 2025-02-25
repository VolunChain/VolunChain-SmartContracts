use crate::event::Event;
use soroban_sdk::{Address, Env, String, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
// #[derive(Clone, Debug, Eq, PartialEq, TryFromVal, IntoVal)]
pub struct NFTMetadata {
    pub owner: Address,
    pub ev_title: String,
    pub ev_date: String,
    pub ev_org: String,
    pub ev_task: String,
}

impl NFTMetadata {
    pub fn new(env: &Env, event_id: u64, owner: Address, task: String) -> Self {
        let event: Event = env
            .storage()
            .persistent()
            .get(&event_id)
            .expect("Event ID invalid");

        Self {
            owner,
            ev_title: event.title.clone(),
            ev_date: event.date.clone(),
            ev_org: event.organization.clone(),
            ev_task: task,
        }
    }

    pub fn update_metadata(env: Env, admin: Address, event_id: u64, token_id: u32) {
        // Check that admin is authorized
        admin.require_auth();

        // Confirm event exists
        let event: Event = env
            .storage()
            .persistent()
            .get(&event_id)
            .expect("Event not found");

        // Get the existing NFT
        let mut nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT ID Invalid");

        // Assign updated event fields
        nft.ev_title = event.title.clone();
        nft.ev_date = event.date.clone();
        nft.ev_org = event.organization.clone();
        nft.ev_task = nft.ev_task;

        env.storage().persistent().set(&token_id, &nft);
    }

    pub fn get_metadata(env: Env, token_id: u32) -> NFTMetadata {
        let nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT ID Invalid");
        nft
    }
}
