use crate::{
    datatype::{DataKeys, NFTError, RecognitionNFT},
    interfaces::{MetadataOperations, MintingOperations},
    RecognitionSystemContract,
};
use soroban_sdk::{
    Address, Env, String, Symbol, Vec,
};

impl MintingOperations for RecognitionSystemContract {
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
        
        // Validate inputs
        if title.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }
        
        if date.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }
        
        if task.len() == 0 {
            return Err(NFTError::MetadataInvalid);
        }

        // Check if organization is authorized
        if !Self::verify_authorized_organization(env, organization.clone()) {
            return Err(NFTError::OrganizationNotAuthorized);
        }
        
        // Get or initialize token counter
        let mut current_id: u128 = env
            .storage()
            .instance()
            .get(&DataKeys::TokenCounter)
            .unwrap_or(0_u128);
        current_id += 1;
        env.storage()
            .instance()
            .set(&DataKeys::TokenCounter, &current_id);

        // Create metadata and NFT
        let metadata = Self::create_nft_metadata(organization, title, date, task)?;
        let nft = RecognitionNFT {
            owner: recipient.clone(),
            metadata,
        };

        // Store NFT by token ID
        env.storage().persistent().set(&current_id, &nft);

        // Update volunteer's badge list
        let mut volunteer_tokens: Vec<u128> = env
            .storage()
            .persistent()
            .get(&DataKeys::VolunteerRecognition(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));
        volunteer_tokens.push_back(current_id);
        env.storage().persistent().set(
            &DataKeys::VolunteerRecognition(recipient.clone()),
            &volunteer_tokens,
        );

        // Emit event
        env.events().publish(
            (Symbol::new(env, "badge_minted"), recipient.clone()),
            current_id,
        );

        Ok(current_id)
    }
    
    // Helper function to verify if an organization is authorized
    fn verify_authorized_organization(env: &Env, org: Address) -> bool {
        // Check if this organization exists in the reputation system
        match env.storage().instance().get::<_, Vec<Address>>(&reputation_system::DataKey::Organizations) {
            Some(organizations) => organizations.contains(&org),
            None => false
        }
    }
}
