use nft_core::NFTError;
use crate::{
    datatype::RecognitionNFT,
    interfaces::DistributionOperations,
    RecognitionSystemContract,
};

use soroban_sdk::{Address, Env, Map, Symbol, Vec};

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
            .ok_or(NFTError::IDInvalid)?;

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

        // Emit burn event
        env.events().publish(
            (Symbol::new(&env, "badge_burned"), owner.clone()),
            token_id,
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
        // Make sure we're calling the right DataKey from reputation system
        let organizations = match env.storage().instance().get::<_, Vec<Address>>(&reputation_system::DataKey::Organizations) {
            Some(orgs) => orgs,
            None => return false,
        };
        
        if !organizations.contains(&org) {
            return false;
        }

        // Check volunteer endorsements from the organization
        let endorsement_key = &reputation_system::DataKey::Endorsements(volunteer.clone());
        match env.storage().instance().get::<_, Map<Address, u32>>(endorsement_key) {
            Some(endorsements) => endorsements.contains_key(org),
            None => false
        }
    }
}