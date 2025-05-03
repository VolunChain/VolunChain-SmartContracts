use crate::types::{DataKey, NFT, NFTError, ExternalURIMetadata};
use soroban_sdk::{Address, Env, Vec, String, log};

// Ledger constants (approx. 30 days in ledgers)
const INSTANCE_LIFETIME_THRESHOLD: u32 = 26280;
const NFT_LIFETIME_THRESHOLD: u32 = 26280;

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
    
    // Bump instance storage lifetime
    bump_instance(env);
    
    count
}

pub fn save_nft(env: &Env, nft: &NFT) {
    // Store the NFT
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
        
        // Extend lifetime
        env.storage().persistent().extend_ttl(
            &DataKey::OwnerTokens(nft.owner.clone()),
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_LIFETIME_THRESHOLD
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
            // Extend NFT storage lifetime
            bump_nft(env, id);
        }
    }
    
    nfts
}

pub fn get_nfts_by_owner_paginated(
    env: &Env, 
    owner: Address, 
    start_pos: u32, 
    limit: u32
) -> Vec<NFT> {
    let token_ids: Vec<u128> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerTokens(owner.clone()))
        .unwrap_or_else(|| Vec::new(env));
    
    let mut nfts = Vec::new(env);
    let mut count = 0;
    let total = token_ids.len();
    
    if start_pos >= total {
        return nfts;
    }
    
    for i in start_pos..total {
        if count >= limit {
            break;
        }
        
        let id = token_ids.get(i).unwrap();
        if let Some(nft) = env.storage().persistent().get(&DataKey::NFT(id)) {
            nfts.push_back(nft);
            // Extend NFT storage lifetime
            bump_nft(env, id);
            count += 1;
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
        authorized_minters.push_back(minter.clone());
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedMinters, &authorized_minters);
        
        // Bump instance storage
        bump_instance(env);
        
        log!(env, "Added authorized minter: {}", minter);
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
        
        // Bump instance storage
        bump_instance(env);
        
        log!(env, "Removed authorized minter: {}", minter);
    }
}

pub fn get_admin(env: &Env) -> Result<Address, NFTError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(NFTError::AdminRequired)
}

pub fn set_uri_base(env: &Env, base_uri: String, suffix: String) {
    let uri_metadata = ExternalURIMetadata {
        base_uri,
        token_uri_suffix: suffix,
    };
    
    env.storage()
        .instance()
        .set(&DataKey::URIBase, &uri_metadata);
    
    // Bump instance storage
    bump_instance(env);
    
    log!(env, "Set URI base");
}

pub fn get_uri_base(env: &Env) -> Option<ExternalURIMetadata> {
    env.storage()
        .instance()
        .get(&DataKey::URIBase)
}

pub fn build_token_uri(env: &Env, _token_id: u128) -> Option<String> {
    let uri_data = get_uri_base(env);
    
    if uri_data.is_none() {
        return None;
    }
    
    let uri_data = uri_data.unwrap();
    let base_uri = uri_data.base_uri.clone();
    
    Some(base_uri)
}

// Storage lifetime management functions
pub fn bump_instance(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_LIFETIME_THRESHOLD);
}

pub fn bump_nft(env: &Env, token_id: u128) {
    env.storage().persistent().extend_ttl(
        &DataKey::NFT(token_id),
        NFT_LIFETIME_THRESHOLD,
        NFT_LIFETIME_THRESHOLD
    );
}

pub fn set_contract_version(env: &Env, version: u32) {
    env.storage()
        .instance()
        .set(&DataKey::ContractVersion, &version);
    
    bump_instance(env);
}