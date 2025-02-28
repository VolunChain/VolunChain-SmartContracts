
use crate::datatype::{
    NFTError, NFTMetadata
};
use soroban_sdk::{Address, Env, String, Symbol, U256, Vec};

pub trait MetadataOperations {
    fn new(
        env: &Env,
        organization: Address,
        title: String,
        date: String,
        task: String
    ) -> Result<NFTMetadata, NFTError>;

    fn update_metadata(
        env: &Env,
        admin: Address,
        token_id: U256,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<(), NFTError>;
}

pub trait MintingOperations {
    fn mint_recognition_badge(
        env: &Env,
        recipient: Address,
        organization: Address,
        title: String,
        date: String,
        task: String
    ) -> Result<U256, NFTError>;
}

pub trait DistributionOperations {
    fn burn_nft(
        env: Env,
        owner: Address,
        token_id: U256
    );

    fn verify_confirmed_volunteer(
        env: &Env,
        volunteer: Address,
        org: Address
    ) -> bool;
}
