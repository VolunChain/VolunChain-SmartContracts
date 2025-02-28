use crate::{
    datatype::{NFTMetadata, RecognitionNFT},
    interfaces::DistributionOperations,
    RecognitionSystemContract, RecognitionSystemContractArgs, RecognitionSystemContractClient
};
use soroban_sdk::{contract, contracterror, contracttype, Address, Env, Map, String, Vec};

impl DistributionOperations for RecognitionSystemContract {
    fn burn_nft(env: Env, owner: Address, token_id: u32) {
        owner.require_auth();

        let nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT non-existent");

        if nft.owner != owner {
            panic!("Unauthorized sender");
        }

        env.storage().persistent().remove(&token_id);
    }

    fn verify_confirmed_volunteer(env: &Env, volunteer: Address, org: Address) -> bool {
        // Check org is authorized
        let organizations = reputation_system::ReputationSystem::get_organizations(env.clone());
        if !organizations.contains(&org) {
            return false;
        }

        // Check volunteer endorsements from the org
        let endorsement_key = &reputation_system::DataKey::Endorsements(volunteer.clone());
        let endorsements: Map<Address, u32> = env
            .storage()
            .instance()
            .get(endorsement_key)
            .unwrap_or_else(|| Map::new(&env));

        endorsements.contains_key(org)
    }
}
