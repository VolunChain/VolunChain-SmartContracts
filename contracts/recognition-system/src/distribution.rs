use crate::{
    datatype::{NFTError, RecognitionNFT, DataKeys},
    interfaces::DistributionOperations,
    RecognitionSystemContract,
};
use soroban_sdk::{Address, Env, Symbol, Vec};

impl DistributionOperations for RecognitionSystemContract {
    /// @notice Allows an owner to burn their badge NFT
    /// @param env The contract environment
    /// @param owner Address of the NFT owner
    /// @param token_id ID of the NFT to burn
    fn burn_nft(env: Env, owner: Address, token_id: u128) -> Result<(), NFTError> {
        // Require authentication from badge owner
        owner.require_auth();

        // Retrieve the badge
        let nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::BadgeNotFound)?;

        // Verify ownership
        if nft.owner != owner {
            return Err(NFTError::UnauthorizedOwner);
        }

        // Remove the NFT from storage
        env.storage().persistent().remove(&token_id);
        
        // Update the volunteer's badge list by removing this token_id
        let badges_key = crate::datatype::DataKeys::VolunteerRecognition(owner.clone());
        if let Some(mut token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&badges_key) {
            let position = token_ids.first_index_of(&token_id);
            if let Some(pos) = position {
                token_ids.remove(pos);
                env.storage().persistent().set(&badges_key, &token_ids);
            }
        }

        // Emit detailed burn event
        env.events().publish(
            (Symbol::new(&env, "badge_burned"), owner.clone(), token_id),
            (nft.metadata.ev_org, nft.metadata.ev_title, nft.metadata.ev_date),
        );
        
        Ok(())
    }

    /// @notice Transfer functionality - should always fail since badges are soulbound
    /// @param env The contract environment
    /// @param from The address of the sender
    /// @param to The address of the recipient
    /// @param token_id The unique identifier of the badge
    /// @return Result indicating failure due to soulbound nature
    fn attempt_transfer(from: Address, _to: Address, _token_id: u128) -> Result<(), NFTError> {
        // Require auth
        from.require_auth();
        
        // Always fail for soulbound tokens
        Err(NFTError::TokenCannotBeTransferred)
    }

    /// @notice Function to verify a badge is authentic and exists
    /// @param env The contract environment
    /// @param token_id The unique identifier of the badge
    /// @return Result indicating whether the badge is authentic or not
    fn verify_badge_authenticity(env: Env, token_id: u128) -> Result<bool, NFTError> {
        // Check if badge exists
        match env.storage().persistent().get::<_, RecognitionNFT>(&token_id) {
            Some(_) => Ok(true),
            None => Err(NFTError::BadgeNotFound)
        }
    }

    /// @notice Verifies if a volunteer is endorsed by an organization
    /// @param env The contract environment
    /// @param volunteer The volunteer address to verify
    /// @param org The organization address to check endorsement from
    /// @return true if volunteer is endorsed by organization, false otherwise
    fn verify_confirmed_volunteer(env: &Env, volunteer: Address, org: Address) -> bool {
        // Validate addresses first
        let volunteer_str = volunteer.to_string();
        let org_str = org.to_string();
        if volunteer_str.len() == 0 || org_str.len() == 0 {
            return false;
        }
        
        // Try to get reputation system contract ID from storage
        let reputation_contract_id = match env.storage().instance().get::<_, Address>(&DataKeys::ReputationContractId) {
            Some(id) => id,
            None => return false, // If no contract ID stored, volunteer is not endorsed
        };

        // Create client and verify volunteer reputation
        let client = crate::interfaces::ReputationSystemClient::new(env, reputation_contract_id);
        match client.get_reputation(&volunteer) {
            Ok(reputation) => reputation > 0, // Volunteer has reputation > 0
            Err(_) => false, // If call fails, volunteer is not endorsed
        }
    }
}