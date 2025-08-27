use crate::{
    datatype::{DataKeys, NFTError, RecognitionNFT, MAX_TITLE_LEN, MAX_DATE_LEN, MAX_TASK_LEN},
    interfaces::{MetadataOperations, MintingOperations, ReputationSystemClient},
    RecognitionSystemContract,
};
use soroban_sdk::{
    Address, Env, String, Symbol, Vec,
};

impl MintingOperations for RecognitionSystemContract {
    /// Mints a recognition badge for a recipient.
    ///
    /// # Parameters
    /// - `env`: The environment context.
    /// - `recipient`: The address of the recipient.
    /// - `organization`: The address of the organization minting the badge.
    /// - `title`: The title of the badge.
    /// - `date`: The date the badge is minted.
    /// - `task`: The task associated with the badge.
    ///
    /// # Returns
    /// Returns the token ID of the minted badge or an error if the minting fails.
    fn mint_recognition_badge(
        env: &Env,
        recipient: Address,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<u128, NFTError> {
        // Require auth from recipient
        recipient.require_auth();
        
        // Validate addresses
        let recipient_str = recipient.to_string();
        let org_str = organization.to_string();
        if recipient_str.len() == 0 || org_str.len() == 0 {
            return Err(NFTError::InvalidAddress);
        }
        
        // Validate inputs with length limits
        if title.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }
        if title.len() as u32 > MAX_TITLE_LEN {
            return Err(NFTError::TitleTooLong);
        }
        
        if date.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }
        if date.len() as u32 > MAX_DATE_LEN {
            return Err(NFTError::DateTooLong);
        }
        
        if task.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }
        if task.len() as u32 > MAX_TASK_LEN {
            return Err(NFTError::TaskTooLong);
        }

        // Check if organization is authorized
        if !Self::verify_authorized_organization(env, organization.clone()) {
            return Err(NFTError::OrganizationNotAuthorized);
        }
        
        // Check badge limit for volunteer
        let volunteer_tokens: Vec<u128> = env
            .storage()
            .persistent()
            .get(&DataKeys::VolunteerRecognition(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));
        if volunteer_tokens.len() as u32 >= 100 { // MAX_BADGES_PER_VOLUNTEER was removed, using a placeholder value
            return Err(NFTError::BadgeLimitExceeded);
        }
        
        // Get or initialize token counter with overflow protection
        let current_id: u128 = env
            .storage()
            .instance()
            .get(&DataKeys::TokenCounter)
            .unwrap_or(0_u128);
        
        // Check for overflow
        if current_id == u128::MAX {
            return Err(NFTError::TokenCounterOverflow);
        }
        
        let new_id = current_id.checked_add(1)
            .ok_or(NFTError::TokenCounterOverflow)?;
            
        env.storage()
            .instance()
            .set(&DataKeys::TokenCounter, &new_id);

        // Create metadata and NFT
        let metadata = Self::create_nft_metadata(organization.clone(), title.clone(), date.clone(), task.clone())?;
        let nft = RecognitionNFT {
            owner: recipient.clone(),
            metadata,
        };

        // Store NFT by token ID
        env.storage().persistent().set(&new_id, &nft);

        // Update volunteer's badge list
        let mut volunteer_tokens: Vec<u128> = env
            .storage()
            .persistent()
            .get(&DataKeys::VolunteerRecognition(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));
        volunteer_tokens.push_back(new_id);
        env.storage().persistent().set(
            &DataKeys::VolunteerRecognition(recipient.clone()),
            &volunteer_tokens,
        );

        // Emit detailed event
        env.events().publish(
            (Symbol::new(env, "badge_minted"), recipient.clone(), organization.clone()),
            (new_id, title.clone(), date.clone(), task.clone()),
        );

        Ok(new_id)
    }
    
    // Helper function to verify if an organization is authorized
    /// Verifies if an organization is authorized to mint badges.
    ///
    /// # Parameters
    /// - `env`: The environment context.
    /// - `org`: The address of the organization to verify.
    ///
    /// # Returns
    /// Returns true if the organization is authorized, false otherwise.
    fn verify_authorized_organization(env: &Env, org: Address) -> bool {
        // Try to get reputation system contract ID from storage
        let reputation_contract_id = match env.storage().instance().get::<_, Address>(&DataKeys::ReputationContractId) {
            Some(id) => id,
            None => return false, // If no contract ID stored, organization is not authorized
        };

        // Create client and verify organization
        let client = ReputationSystemClient::new(env, reputation_contract_id);
        match client.get_organizations() {
            Ok(organizations) => organizations.contains(&org),
            Err(_) => false, // If call fails, organization is not authorized
        }
    }
}
