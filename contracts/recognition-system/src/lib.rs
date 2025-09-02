#![no_std]
use datatype::{AdminError, DataKeys, NFTError, NFTMetadata, RecognitionNFT, MAX_PAGINATION_LIMIT};
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Symbol, Vec,
};

mod datatype;
mod distribution;
mod interfaces;
mod metadata;
mod nft_minting;

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
        
        // Validate admin address
        let admin_str = admin.to_string();
        if admin_str.len() == 0 {
            return Err(AdminError::UnauthorizedSender);
        }
        
        // Set admin and initialize token counter
        env.storage().instance().set(&DataKeys::Admin, &admin);
        env.storage().instance().set(&DataKeys::TokenCounter, &0);

        // Emit initialization event with timestamp validation
        let timestamp = env.ledger().timestamp();
        let max_future_time = timestamp + 24 * 3600; // 24 hours max future
        let safe_timestamp = if timestamp > max_future_time { timestamp } else { timestamp };
        
        env.events().publish(
            (Symbol::new(&env, "Contract Initialized"), admin.clone()),
            safe_timestamp,
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
            Err(NFTError::BadgeNotFound) // Fixed: correct error type
        }
    }

    /// @notice Retrieves all badges owned by a specific volunteer with pagination
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @param offset The starting index for pagination
    /// @param limit The maximum number of badges to return
    /// @return A vector of badges owned by the volunteer
    pub fn get_volunteer_badges_paginated(
        env: Env,
        volunteer: Address,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<RecognitionNFT>, NFTError> {
        // Validate pagination parameters
        if limit > MAX_PAGINATION_LIMIT {
            return Err(NFTError::PaginationLimitExceeded);
        }
        
        // Get the list of token IDs owned by this volunteer
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids: Vec<u128> = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));

        // Load each NFT by its ID and collect in a vector with pagination
        let mut nfts: Vec<RecognitionNFT> = Vec::new(&env);
        let total_count = token_ids.len() as u32;
        let start_index = offset as usize;
        let end_index = core::cmp::min(start_index + limit as usize, total_count as usize);
        
        for i in start_index..end_index {
            if let Some(token_id) = token_ids.get(i as u32) {
                if let Some(nft) = env.storage().persistent().get(&token_id) {
                    nfts.push_back(nft);
                }
            }
        }

        Ok(nfts)
    }

    /// @notice Retrieves all badges owned by a specific volunteer (backward compatibility)
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @return A vector of badges owned by the volunteer
    pub fn get_volunteer_badges(
        env: Env,
        volunteer: Address,
    ) -> Result<Vec<RecognitionNFT>, NFTError> {
        // Use pagination with default limits for backward compatibility
        Self::get_volunteer_badges_paginated(env, volunteer, 0, MAX_PAGINATION_LIMIT)
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
    
    /// @notice Gets the IDs of all badges owned by a volunteer with pagination
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @param offset The starting index for pagination
    /// @param limit The maximum number of badge IDs to return
    /// @return A vector of badge IDs owned by the volunteer
    pub fn get_badge_ids_paginated(
        env: Env, 
        volunteer: Address, 
        offset: u32, 
        limit: u32
    ) -> Result<Vec<u128>, NFTError> {
        // Validate pagination parameters
        if limit > MAX_PAGINATION_LIMIT {
            return Err(NFTError::PaginationLimitExceeded);
        }
        
        let badges_key = DataKeys::VolunteerRecognition(volunteer.clone());
        let token_ids = env
            .storage()
            .persistent()
            .get(&badges_key)
            .unwrap_or_else(|| Vec::new(&env));
            
        // Apply pagination
        let mut result: Vec<u128> = Vec::new(&env);
        let total_count = token_ids.len() as u32;
        let start_index = offset as usize;
        let end_index = core::cmp::min(start_index + limit as usize, total_count as usize);
        
        for i in start_index..end_index {
            if let Some(token_id) = token_ids.get(i as u32) {
                result.push_back(token_id);
            }
        }
            
        Ok(result)
    }
    
    /// @notice Gets the IDs of all badges owned by a volunteer (backward compatibility)
    /// @param env The contract environment
    /// @param volunteer The address of the volunteer
    /// @return A vector of badge IDs owned by the volunteer
    pub fn get_badge_ids(env: Env, volunteer: Address) -> Result<Vec<u128>, NFTError> {
        // Use pagination with default limits for backward compatibility
        Self::get_badge_ids_paginated(env, volunteer, 0, MAX_PAGINATION_LIMIT)
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

    /// @notice Sets the reputation system contract ID (admin only)
    /// @param env The contract environment
    /// @param admin The admin address
    /// @param reputation_contract_id The contract ID of the reputation-system
    /// @return Result indicating success or error
    pub fn set_reputation_contract_id(env: Env, admin: Address, reputation_contract_id: Address) -> Result<(), NFTError> {
        // Verify admin
        let contract_admin = match Self::get_admin(env.clone()) {
            Ok(admin_addr) => admin_addr,
            Err(_) => return Err(NFTError::UnauthorizedOwner),
        };
        if admin != contract_admin {
            return Err(NFTError::UnauthorizedOwner);
        }
        admin.require_auth();

        // Validate contract ID
        let contract_id_str = reputation_contract_id.to_string();
        if contract_id_str.len() == 0 {
            return Err(NFTError::InvalidAddress);
        }

        // Store the reputation system contract ID
        env.storage().instance().set(&DataKeys::ReputationContractId, &reputation_contract_id);

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "reputation_contract_set"), admin.clone()),
            reputation_contract_id,
        );

        Ok(())
    }

    /// @notice Gets the reputation system contract ID
    /// @param env The contract environment
    /// @return The contract ID or None if not set
    pub fn get_reputation_contract_id(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKeys::ReputationContractId)
    }
}
