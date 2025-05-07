#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

pub mod types;
mod storage;
mod minting;
mod admin;
mod interfaces;
mod test;

pub use types::{NFT, NFTError, NFTMetadata, DataKey, NFTMintBatch, ExternalURIMetadata};

#[contract]
pub struct NFTCore;

#[contractimpl]
impl NFTCore {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) -> Result<(), types::NFTError> {
        admin::initialize(&env, admin)
    }
    
    // Minting operations
    pub fn mint_nft(
        env: Env,
        minter: Address,
        recipient: Address,
        title: String,
        description: String,
        attributes: Vec<(String, String)>,
        transferable: bool
    ) -> Result<u128, types::NFTError> {
        minting::mint_nft(&env, minter, recipient, title, description, attributes, transferable)
    }
    
    pub fn batch_mint_nfts(
        env: Env,
        minter: Address,
        batch: NFTMintBatch,
    ) -> Result<Vec<u128>, types::NFTError> {
        minting::batch_mint_nfts(&env, minter, batch)
    }
    
    pub fn burn_nft(env: Env, owner: Address, token_id: u128) -> Result<(), types::NFTError> {
        minting::burn_nft(&env, owner, token_id)
    }
    
    // Admin operations
    pub fn add_authorized_minter(env: Env, admin: Address, minter: Address) -> Result<(), types::NFTError> {
        admin::add_authorized_minter(&env, admin, minter)
    }
    
    pub fn remove_authorized_minter(env: Env, admin: Address, minter: Address) -> Result<(), types::NFTError> {
        admin::remove_authorized_minter(&env, admin, minter)
    }
    
    pub fn set_uri_base(env: Env, admin: Address, base_uri: String, suffix: String) -> Result<(), types::NFTError> {
        admin::set_uri_base_for_tokens(&env, admin, base_uri, suffix)
    }
    
    pub fn pause_contract(env: Env, admin: Address) -> Result<(), types::NFTError> {
        admin::pause_contract(&env, admin)
    }
    
    pub fn unpause_contract(env: Env, admin: Address) -> Result<(), types::NFTError> {
        admin::unpause_contract(&env, admin)
    }
    
    pub fn upgrade_contract(env: Env, admin: Address, new_version: u32) -> Result<(), types::NFTError> {
        admin::upgrade_contract(&env, admin, new_version)
    }
    
    // Query operations
    pub fn get_nft(env: Env, token_id: u128) -> Result<types::NFT, types::NFTError> {
        storage::get_nft(&env, token_id)
    }
    
    pub fn get_nfts_by_owner(env: Env, owner: Address) -> Vec<types::NFT> {
        storage::get_nfts_by_owner(&env, owner)
    }
    
    pub fn get_nfts_by_owner_paginated(
        env: Env,
        owner: Address,
        start_pos: u32,
        limit: u32
    ) -> Vec<types::NFT> {
        storage::get_nfts_by_owner_paginated(&env, owner, start_pos, limit)
    }
    
    pub fn is_authorized_minter(env: Env, minter: Address) -> bool {
        storage::is_authorized_minter(&env, &minter)
    }
    
    pub fn get_token_metadata(env: Env, token_id: u128) -> Result<types::NFTMetadata, types::NFTError> {
        let nft = storage::get_nft(&env, token_id)?;
        Ok(nft.metadata)
    }
    
    pub fn get_token_uri(env: Env, token_id: u128) -> Option<String> {
        storage::build_token_uri(&env, token_id)
    }
    
    pub fn get_contract_version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContractVersion)
            .unwrap_or(0)
    }
}