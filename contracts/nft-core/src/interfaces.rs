use crate::types::{NFTError, NFTMetadata, NFT};
use soroban_sdk::{Address, Env, String, Vec};

pub trait MintingOperations {
    fn mint_nft(
        env: &Env,
        minter: Address,
        recipient: Address,
        metadata: NFTMetadata,
        transferable: bool
    ) -> Result<u128, NFTError>;
    
    fn burn_nft(
        env: &Env,
        owner: Address,
        token_id: u128
    ) -> Result<(), NFTError>;
}

pub trait QueryOperations {
    fn get_nft(
        env: &Env, 
        token_id: u128
    ) -> Result<NFT, NFTError>;
    
    fn get_nfts_by_owner(
        env: &Env,
        owner: Address
    ) -> Vec<NFT>;
    
    fn get_nfts_by_issuer(
        env: &Env,
        issuer: Address
    ) -> Vec<NFT>;
}

pub trait AdminOperations {
    fn add_authorized_minter(
        env: &Env,
        admin: Address,
        minter: Address
    ) -> Result<(), NFTError>;
    
    fn remove_authorized_minter(
        env: &Env,
        admin: Address,
        minter: Address
    ) -> Result<(), NFTError>;
}