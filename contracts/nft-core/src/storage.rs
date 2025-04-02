use crate::types::{DataKey, NFT, NFTError};
use soroban_sdk::{Address, Env, Vec};

pub fn get_token_count(env: &Env) -> u128 {
    env.storage()
        .instance()
        .get(&DataKey::TokenCount)
        .unwrap_or(0)
}

pub fn increment_token_count(env: &Env) -> u128 {
    let count = get_token_count(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::TokenCount, &count);
    count
}

pub fn save_nft(env: &Env, nft: &NFT) {
    env.storage().persistent().set(&DataKey::NFT(nft.id), nft);
    
    // Update owner's token list
    let mut owner_tokens: Vec<u128> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerTokens(nft.owner.clone()))
        .unwrap_or_else(|| Vec::new(env));
    
    if !owner_tokens.contains(&nft.id) {
        owner_tokens.push_back(nft.id);
        env.storage().persistent().set(
            &DataKey::OwnerTokens(nft.owner.clone()),
            &owner_tokens
        );
    }
}

pub fn get_nft(env: &Env, token_id: u128) -> Result<NFT, NFTError> {
    env.storage()
        .persistent()
        .get(&DataKey::NFT(token_id))
        .ok_or(NFTError::TokenNotFound)
}

pub fn get_nfts_by_owner(env: &Env, owner: Address) -> Vec<NFT> {
    let token_ids: Vec<u128> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerTokens(owner.clone()))
        .unwrap_or_else(|| Vec::new(env));
    
    let mut nfts = Vec::new(env);
    for id in token_ids.iter() {
        if let Some(nft) = env.storage().persistent().get(&DataKey::NFT(id)) {
            nfts.push_back(nft);
        }
    }
    
    nfts
}

pub fn is_authorized_minter(env: &Env, minter: &Address) -> bool {
    let authorized_minters: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorizedMinters)
        .unwrap_or_else(|| Vec::new(env));
    
    authorized_minters.contains(minter)
}

pub fn add_authorized_minter(env: &Env, minter: Address) {
    let mut authorized_minters: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorizedMinters)
        .unwrap_or_else(|| Vec::new(env));
    
    if !authorized_minters.contains(&minter) {
        authorized_minters.push_back(minter);
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedMinters, &authorized_minters);
    }
}

pub fn remove_authorized_minter(env: &Env, minter: Address) {
    let mut authorized_minters: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorizedMinters)
        .unwrap_or_else(|| Vec::new(env));
    
    let index = authorized_minters.first_index_of(&minter);
    if let Some(idx) = index {
        authorized_minters.remove(idx);
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedMinters, &authorized_minters);
    }
}

pub fn get_admin(env: &Env) -> Result<Address, NFTError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(NFTError::AdminRequired)
}