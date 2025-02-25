use crate::datatype::{Event};
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
    pub fn new(env: &Env, event_id: u64, task: String) -> Self{
        // TODO: Confirm event;

        let event: Event = env.storage().persistent().get(&event_id).expect("Event ID invalid");

        Self {
            owner: ,
            ev_title: event.title,
            ev_date: event.date,
            ev_org: event.organization,
            ev_task: task,
        }
    }

    pub fn update_metadata(
        env: Env,
        admin: Address,
        token_id: u32,
        ev_title: String,
        ev_date: String,
        ev_org: String,
        ev_task: String,
    ) {
        Self::check_admin(&env, &admin);

        let mut nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT ID Invalid");

        nft.ev_title = ev_title;
        nft.ev_date = ev_date;
        nft.ev_org = ev_org;
        nft.ev_task = ev_task;

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