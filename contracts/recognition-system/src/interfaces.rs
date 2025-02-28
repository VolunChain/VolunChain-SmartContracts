use crate::datatype::{NFTError, NFTMetadata};
use soroban_sdk::{Address, Env, String};

#[allow(dead_code)]
pub trait MetadataOperations {
    fn create_nft_metadata(
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<NFTMetadata, NFTError>;

    fn update_metadata(
        env: &Env,
        admin: Address,
        token_id: u128,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<(), NFTError>;
}

#[allow(dead_code)]
pub trait MintingOperations {
    fn mint_recognition_badge(
        env: &Env,
        recipient: Address,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<u128, NFTError>;

    fn verify_authorized_organization(
        env: &Env,
        org: Address
    ) -> bool;
}

#[allow(dead_code)]
pub trait DistributionOperations {
    fn burn_nft(
        env: Env,
        owner: Address,
        token_id: u128
    ) -> Result<(), NFTError>;

    fn attempt_transfer(
        from: Address,
        to: Address,
        token_id: u128
    ) -> Result<(), NFTError>;

    fn verify_badge_authenticity(
        env: Env,
        token_id: u128
    ) -> Result<bool, NFTError>;

    fn verify_confirmed_volunteer(env: &Env,
        volunteer:
        Address, org: Address
    ) -> bool;
}
