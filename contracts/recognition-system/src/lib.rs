#![no_std]

use datatype::{AdminError, DataKeys, NFTMetadata, RecognitionNFT};
use nft_core::NFTError;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};
use soroban_sdk::vec;
use nft_core::{NFTCore};
mod datatype;
mod distribution;
mod interfaces;
mod metadata;

#[cfg(test)]
mod test;

/// @title RecognitionSystemContract
/// @notice A contract for minting and managing non-transferable NFT badges
/// awarded to volunteers for their contributions.
#[contract]
pub struct RecognitionSystemContract;

#[contractimpl]
impl RecognitionSystemContract {
    /// @notice Initializes the contract with an admin who can perform privileged operations
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

    /// @notice Retrieves the contract administrator
    pub fn get_admin(env: Env) -> Result<Address, AdminError> {
        env.storage()
            .instance()
            .get(&DataKeys::Admin)
            .ok_or(AdminError::UnauthorizedSender)
    }

    /// @notice Delegates the minting of a recognition badge to the centralized NFTCore contract
    pub fn mint_recognition_badge(
        env: Env,
        recipient: Address,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<u128, NFTError> {
        // Require the recipient to authorize the action
        recipient.require_auth();

        // Validate input fields
        if title.len() == 0 || date.len() == 0 || task.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }

        // Only allow authorized organizations to mint
        if !Self::verify_authorized_organization(&env, organization.clone()) {
            return Err(NFTError::OrganizationNotAuthorized);
        }

        // Convert inputs into NFT attributes
        let attributes = Vec::from_array(
            &env,
            [
                (String::from_str(&env, "task"), task),
                (String::from_str(&env, "date"), date),
            ],
        );
        

        let badge_metadata = String::from_str(&env, "https://example.com/metadata.json");

        // Mint the NFT via the shared NFTCore module
        NFTCore::mint_nft(
            env.clone(),
            organization,
            recipient,
            String::from_str(&env, "Recognition Badge"),
            badge_metadata,
            attributes,
            false,
        )
        
    }

    /// @notice Verifies if an organization is authorized to mint badges
    fn verify_authorized_organization(env: &Env, org: Address) -> bool {
        use reputation_system::DataKey;
        match env
            .storage()
            .instance()
            .get::<_, Vec<Address>>(&DataKey::Organizations)
        {
            Some(orgs) => orgs.contains(&org),
            None => false,
        }
    }

    /// @notice Fetches a single badge by ID
    pub fn get_volunteer_badge(env: Env, token_id: u128) -> Result<RecognitionNFT, NFTError> {
        env.storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::IDInvalid)
    }

    /// @notice Fetches all badges owned by a given volunteer
    pub fn get_volunteer_badges(env: Env, volunteer: Address) -> Result<Vec<RecognitionNFT>, NFTError> {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids: Vec<u128> = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));

        let mut nfts: Vec<RecognitionNFT> = Vec::new(&env);
        for id in token_ids.iter() {
            if let Some(nft) = env.storage().persistent().get(&id) {
                nfts.push_back(nft);
            }
        }

        Ok(nfts)
    }

    /// @notice Fetches the metadata for a badge by token ID
    pub fn get_metadata(env: &Env, token_id: u128) -> Result<NFTMetadata, NFTError> {
        let nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::BadgeNotFound)?;
        Ok(nft.metadata)
    }

    /// @notice Checks if a volunteer owns a specific badge
    pub fn has_badge(env: Env, volunteer: Address, token_id: u128) -> bool {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        if let Some(token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&badges_key) {
            token_ids.contains(&token_id)
        } else {
            false
        }
    }

    /// @notice Checks if a volunteer has received a badge from a given organization
    pub fn has_org_badge(env: Env, volunteer: Address, org: Address) -> bool {
        if let Ok(badges) = Self::get_volunteer_badges(env.clone(), volunteer) {
            for badge in badges.iter() {
                if badge.metadata.ev_org == org {
                    return true;
                }
            }
        }
        false
    }

    /// @notice Returns the number of badges owned by a volunteer
    pub fn get_badge_count(env: Env, volunteer: Address) -> u32 {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        if let Some(token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&badges_key) {
            token_ids.len() as u32
        } else {
            0
        }
    }

    /// @notice Returns a list of token IDs for all badges owned by a volunteer
    pub fn get_badge_ids(env: Env, volunteer: Address) -> Result<Vec<u128>, NFTError> {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));

        Ok(token_ids)
    }

    /// @notice Returns exported badge data in a simplified format
    pub fn export_badge_data(env: Env, token_id: u128) -> Result<(Address, String, String, String), NFTError> {
        let nft = Self::get_volunteer_badge(env.clone(), token_id)?;
        Ok((
            nft.metadata.ev_org,
            nft.metadata.ev_title,
            nft.metadata.ev_date,
            nft.metadata.task,
        ))
    }
}
