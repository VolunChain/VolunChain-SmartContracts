#![allow(dead_code)]
use crate::types::{NFT, NFTError, NFTMetadata, NFTMintBatch};
use soroban_sdk::{Address, Env, String, Vec};

/// NFT Core interface - represents the core functionality of the NFT contract
#[allow(dead_code)]
pub trait NFTCoreInterface {
    // Initialize the contract with an admin
    fn initialize(env: Env, admin: Address) -> Result<(), NFTError>;
    
    // Mint a new NFT
    fn mint_nft(
        env: Env,
        minter: Address,
        recipient: Address,
        title: String,
        description: String,
        attributes: Vec<(String, String)>,
        transferable: bool
    ) -> Result<u128, NFTError>;
    
    // Batch mint multiple NFTs in a single transaction
    fn batch_mint_nfts(
        env: Env,
        minter: Address,
        batch: NFTMintBatch,
    ) -> Result<Vec<u128>, NFTError>;
    
    // Burn an NFT
    fn burn_nft(env: Env, owner: Address, token_id: u128) -> Result<(), NFTError>;
}

/// Admin interface - represents administrative operations for the NFT contract
#[allow(dead_code)]
pub trait NFTAdminInterface {
    // Add an authorized minter
    fn add_authorized_minter(env: Env, admin: Address, minter: Address) -> Result<(), NFTError>;
    
    // Remove an authorized minter
    fn remove_authorized_minter(env: Env, admin: Address, minter: Address) -> Result<(), NFTError>;
    
    // Set the base URI for external metadata
    fn set_uri_base(env: Env, admin: Address, base_uri: String, suffix: String) -> Result<(), NFTError>;
    
    // Pause contract operations (except admin functions)
    fn pause_contract(env: Env, admin: Address) -> Result<(), NFTError>;
    
    // Unpause contract operations
    fn unpause_contract(env: Env, admin: Address) -> Result<(), NFTError>;
    
    // Upgrade contract version
    fn upgrade_contract(env: Env, admin: Address, new_version: u32) -> Result<(), NFTError>;
}

/// Query interface - represents read-only operations for the NFT contract
#[allow(dead_code)]
pub trait NFTQueryInterface {
    // Get an NFT by token ID
    fn get_nft(env: Env, token_id: u128) -> Result<NFT, NFTError>;
    
    // Get all NFTs owned by an address
    fn get_nfts_by_owner(env: Env, owner: Address) -> Vec<NFT>;
    
    // Get paginated NFTs owned by an address
    fn get_nfts_by_owner_paginated(
        env: Env,
        owner: Address,
        start_pos: u32,
        limit: u32
    ) -> Vec<NFT>;
    
    // Check if an address is an authorized minter
    fn is_authorized_minter(env: Env, minter: Address) -> bool;
    
    // Get the metadata for a token
    fn get_token_metadata(env: Env, token_id: u128) -> Result<NFTMetadata, NFTError>;
    
    // Get the external URI for a token
    fn get_token_uri(env: Env, token_id: u128) -> Option<String>;
    
    // Get the contract version
    fn get_contract_version(env: Env) -> u32;
}