#![no_std]
use datatype::{AdminError, DataKeys, NFTError, NFTMetadata, RecognitionNFT};
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol, Vec,
    U256,
};

// use reputation_system;

mod datatype;
mod distribution;
mod interfaces;
mod metadata;
mod minting;

#[cfg(test)]
mod test;

#[contract]
pub struct RecognitionSystemContract;

#[contractimpl]
impl RecognitionSystemContract {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) -> Result<(), AdminError> {
        if env.storage().instance().has(&DataKeys::Admin) {
            return Err(AdminError::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKeys::Admin, &admin);
        env.storage().instance().set(&DataKeys::TokenCounter, &0);

        env.events().publish(
            (Symbol::new(&env, "Contract Initialized"), admin.clone()),
            env.ledger().timestamp(),
        );

        Ok(())
    }

    pub fn get_admin(env: Env) -> Result<Address, AdminError> {
        env.storage()
            .instance()
            .get(&DataKeys::Admin)
            .ok_or(AdminError::UnauthorizedSender)
    }

    pub fn get_volunteer_badge(env: Env, token_id: U256) -> Result<RecognitionNFT, NFTError> {
        if let Some(nft) = env
            .storage()
            .persistent()
            .get(&token_id) {
            Ok(nft)
        } else {
            Err(NFTError::IDInvalid)
        }
    }

    pub fn get_volunteer_badges(
        env: Env,
        volunteer: Address,
    ) -> Result<Vec<RecognitionNFT>, NFTError> {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids: Vec<U256> = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));

        let mut nfts = Vec::new(&env);
        for id in token_ids.iter() {
            if let Some(nft) = env.storage().persistent().get(&id) {
                nfts.push_back(nft);
            }
        }

        Ok(nfts)
    }

    pub fn get_metadata(env: &Env, token_id: U256) -> Result<NFTMetadata, NFTError> {
        let nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT not found");

        Ok(nft.metadata)
    }
}
