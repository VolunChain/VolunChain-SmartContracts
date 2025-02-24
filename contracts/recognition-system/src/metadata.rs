use crate::datatype::Event;
use crate::datatype::NFTMetadata;
use soroban_sdk::{Address, Env, String, Vec};

impl NFTMetadata {
    pub fn new(env: &Env, event_id: u64, task: String) -> Self{
        // TODO: Confirm event;

        let event: Event = env.storage().persistent().get(&event_id).expect("Event ID invalid");

        Self {
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
        name: String,
        description: String,
        attributes: Vec<String>,
    ) {
        Self::check_admin(&env, &admin);

        let mut nft: = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT ID Invalid");

        nft.metadata = NFTMetadata {
            name,
            description,
            attributes,
        };

        env.storage().persistent().set(&token_id, &nft);
    }

    pub fn get_metadata(env: Env, token_id: u32) -> NFTMetadata {
        let nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT ID Invalid");
        nft.metadata
    }
}