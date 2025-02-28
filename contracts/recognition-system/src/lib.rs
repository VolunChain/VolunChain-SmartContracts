#![no_std]
use datatype::{AdminError, DataKeys, NFTError, NFTMetadata, RecognitionNFT};
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Symbol, Vec,
};

mod datatype;
mod distribution;
mod interfaces;
mod metadata;
mod minting;

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
    /// @param env The contract environment
    /// @param admin The address of the contract administrator
    /// @return Result indicating success or initialization error
    pub fn initialize(env: Env, admin: Address) -> Result<(), AdminError> {
        // Check if already initialized
        if env.storage().instance().has(&DataKeys::Admin) {
            return Err(AdminError::AlreadyInitialized);
        }

        // Require authentication from admin
        admin.require_auth();
        
        // Set admin and initialize token counter
        env.storage().instance().set(&DataKeys::Admin, &admin);
        env.storage().instance().set(&DataKeys::TokenCounter, &0);

        // Emit initialization event
        env.events().publish(
            (Symbol::new(&env, "Contract Initialized"), admin.clone()),
            env.ledger().timestamp(),
        );

        Ok(())
    }

    /// @notice Retrieves the admin address for this contract
    /// @param env The contract environment
    /// @return The admin address or an error if not found
    pub fn get_admin(env: Env) -> Result<Address, AdminError> {
        env.storage()
            .instance()
            .get(&DataKeys::Admin)
            .ok_or(AdminError::UnauthorizedSender)
    }

    /// @notice Retrieves a specific NFT badge by its ID
    /// @param env The contract environment
    /// @param token_id The unique identifier of the badge
    /// @return The badge details or error if not found
    pub fn get_volunteer_badge(env: Env, token_id: u128) -> Result<RecognitionNFT, NFTError> {
        // Try to retrieve the NFT from persistent storage
        if let Some(nft) = env
            .storage()
            .persistent()
            .get(&token_id) {
            Ok(nft)
        } else {
            Err(NFTError::IDInvalid)
        }
    }

    /// @notice Retrieves all badges owned by a specific volunteer
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @return A vector of badges owned by the volunteer
    pub fn get_volunteer_badges(
        env: Env,
        volunteer: Address,
    ) -> Result<Vec<RecognitionNFT>, NFTError> {
        // Get the list of token IDs owned by this volunteer
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids: Vec<u128> = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));

        // Load each NFT by its ID and collect in a vector
        let mut nfts: Vec<RecognitionNFT> = Vec::new(&env);
        for id in token_ids.iter() {
            if let Some(nft) = env.storage().persistent().get(&id) {
                nfts.push_back(nft);
            }
        }

        Ok(nfts)
    }

    /// @notice Gets the metadata for a specific badge
    /// @param env The contract environment
    /// @param token_id The unique identifier of the badge
    /// @return The badge's metadata or error if not found
    pub fn get_metadata(env: &Env, token_id: u128) -> Result<NFTMetadata, NFTError> {
        let nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::BadgeNotFound)?;

        Ok(nft.metadata)
    }

    /// @notice Checks if a volunteer owns a specific badge
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @param token_id The unique identifier of the badge
    /// @return true if volunteer owns the badge, false otherwise
    pub fn has_badge(env: Env, volunteer: Address, token_id: u128) -> bool {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        if let Some(token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&badges_key) {
            token_ids.contains(&token_id)
        } else {
            false
        }
    }
    
    /// @notice Checks if a volunteer has any badges from a specific organization
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @param org The address of the organization
    /// @return true if volunteer has any badges from the org, false otherwise
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

    /// @notice Gets the total number of badges owned by a volunteer
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @return The number of badges owned
    pub fn get_badge_count(env: Env, volunteer: Address) -> u32 {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        if let Some(token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&badges_key) {
            token_ids.len() as u32
        } else {
            0
        }
    }
    
    /// @notice Gets the IDs of all badges owned by a volunteer
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @return A vector of badge IDs owned by the volunteer
    pub fn get_badge_ids(env: Env, volunteer: Address) -> Result<Vec<u128>, NFTError> {
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));
            
        Ok(token_ids)
    }
    
    /// @notice Exports badge data in a simplified format for external applications
    /// @param env The contract environment
    /// @param token_id The unique identifier of the badge
    /// @return Tuple of (org_address, title, date, task) or error if not found
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
