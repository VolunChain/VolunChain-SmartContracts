#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

mod types;
mod storage;
mod minting;
mod admin;
mod interfaces;
mod test;

pub use types::{NFT, NFTError, NFTMetadata, DataKey};

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
    
    // Query operations
    pub fn get_nft(env: Env, token_id: u128) -> Result<types::NFT, types::NFTError> {
        storage::get_nft(&env, token_id)
    }
    
    pub fn get_nfts_by_owner(env: Env, owner: Address) -> Vec<types::NFT> {
        storage::get_nfts_by_owner(&env, owner)
    }
    
    pub fn is_authorized_minter(env: Env, minter: Address) -> bool {
        storage::is_authorized_minter(&env, &minter)
    }
    
    pub fn get_token_metadata(env: Env, token_id: u128) -> Result<types::NFTMetadata, types::NFTError> {
        let nft = storage::get_nft(&env, token_id)?;
        Ok(nft.metadata)
    }
}