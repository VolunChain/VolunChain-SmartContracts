use crate::storage::{get_nft, increment_token_count, is_authorized_minter, save_nft, bump_instance, bump_nft};
use crate::types::{NFT, NFTError, NFTMetadata, DataKey, NFTMintBatch};
use soroban_sdk::{Address, Env, Map, String, Vec, log};

// Mint a new NFT
// # Security considerations
// - Only authorized minters can create NFTs
// - Validates all inputs before processing
// - Ensures proper event emission for auditability

pub fn mint_nft(
    env: &Env, 
    minter: Address, 
    recipient: Address, 
    title: String, 
    description: String, 
    attributes: Vec<(String, String)>,
    transferable: bool
) -> Result<u128, NFTError> {
    // Check if contract is paused
    if env.storage().instance().has(&DataKey::Paused) {
        return Err(NFTError::ContractPaused);
    }
    
    // Authorization validation
    if !is_authorized_minter(env, &minter) {
        return Err(NFTError::Unauthorized);
    }
    
    // Require minter authorization
    minter.require_auth();
    
    // Input validation
    if title.len() == 0 {
        return Err(NFTError::InvalidInput);
    }
    
    if description.len() == 0 {
        return Err(NFTError::InvalidInput);
    }
    
    // Recipient validation
    if recipient.to_string().len() == 0 {
        return Err(NFTError::InvalidRecipient);
    }
    
    // Increment token counter
    let token_id = increment_token_count(env);
    
    // Create NFT metadata
    let metadata = NFTMetadata {
        issuer: minter.clone(),
        title: title.clone(),
        description: description.clone(),
        creation_date: env.ledger().timestamp(),
        attributes: build_attribute_map(env, attributes.clone()),
    };
    
    // Create the NFT
    let nft = NFT {
        id: token_id,
        owner: recipient.clone(),
        metadata,
        transferable,
        minted_at: env.ledger().timestamp(),
    };
    
    // Save NFT to storage
    save_nft(env, &nft);
    
    // Extend storage lifetime
    bump_instance(env);
    bump_nft(env, token_id);
    
    // Emit mint event
    env.events().publish(
        ("mint", token_id),
        (minter.clone(), recipient.clone(), title.clone())
    );
    
    log!(env, "NFT minted: {} to {}", token_id, recipient);
    
    Ok(token_id)
}

// Mint multiple NFTs in a batch
// - Only authorized minters can create NFTs
// - Validates all batch data matches in length
// - Validates individual inputs
// - Efficiently processes multiple NFTs in a single transaction

pub fn batch_mint_nfts(
    env: &Env,
    minter: Address,
    batch: NFTMintBatch,
) -> Result<Vec<u128>, NFTError> {
    // Check if contract is paused
    if env.storage().instance().has(&DataKey::Paused) {
        return Err(NFTError::ContractPaused);
    }
    
    // Authorization validation
    if !is_authorized_minter(env, &minter) {
        return Err(NFTError::Unauthorized);
    }
    
    // Require minter authorization
    minter.require_auth();
    
    // Validate consistent lengths of batch data
    let recipients_len = batch.recipients.len();
    if batch.titles.len() != recipients_len ||
       batch.descriptions.len() != recipients_len ||
       batch.attributes.len() != recipients_len ||
       batch.transferable.len() != recipients_len {
        return Err(NFTError::InvalidBatchData);
    }
    
    // Validate batch is not empty
    if recipients_len == 0 {
        return Err(NFTError::InvalidBatchData);
    }
    
    // Validate maximum batch size to prevent gas attacks
    if recipients_len > 50 {
        return Err(NFTError::BatchTooLarge);
    }
    
    let mut token_ids = Vec::new(env);
    let current_timestamp = env.ledger().timestamp();
    
    for i in 0..recipients_len {
        let recipient = batch.recipients.get(i).unwrap();
        let title = batch.titles.get(i).unwrap();
        let description = batch.descriptions.get(i).unwrap();
        let attributes = batch.attributes.get(i).unwrap();
        let transferable = batch.transferable.get(i).unwrap();
        
        // Individual validations
        if title.len() == 0 || description.len() == 0 {
            return Err(NFTError::InvalidInput);
        }
        
        // Recipient validation
        if recipient.to_string().len() == 0 {
            return Err(NFTError::InvalidRecipient);
        }
        
        // Increment token counter
        let token_id = increment_token_count(env);
        
        // Create NFT metadata
        let metadata = NFTMetadata {
            issuer: minter.clone(),
            title: title.clone(),
            description: description.clone(),
            creation_date: current_timestamp,
            attributes: build_attribute_map(env, attributes.clone()),
        };
        
        // Create the NFT
        let nft = NFT {
            id: token_id,
            owner: recipient.clone(),
            metadata,
            transferable,
            minted_at: current_timestamp,
        };
        
        // Save NFT to storage
        save_nft(env, &nft);
        
        // Extend storage lifetime
        bump_nft(env, token_id);
        
        // Emit mint event
        env.events().publish(
            ("batch_mint", token_id),
            (minter.clone(), recipient.clone(), title.clone())
        );
        
        token_ids.push_back(token_id);
        
        log!(env, "Batch NFT minted: {} to {}", token_id, recipient);
    }
    
    bump_instance(env);
    
    Ok(token_ids)
}

// Burn (destroy) an NFT
// # Security considerations
// - Only the owner can burn their NFT
// - Requires authorization from the owner
// - Emits events for auditability

pub fn burn_nft(env: &Env, owner: Address, token_id: u128) -> Result<(), NFTError> {
    // Check if contract is paused
    if env.storage().instance().has(&DataKey::Paused) {
        return Err(NFTError::ContractPaused);
    }
    
    // Get the NFT
    let nft = get_nft(env, token_id)?;
    
    // Verify ownership
    if nft.owner != owner {
        return Err(NFTError::Unauthorized);
    }
    
    // Require owner authorization
    owner.require_auth();
    
    // Remove NFT from storage
    env.storage().persistent().remove(&DataKey::NFT(token_id));
    
    // Update owner's token list
    let mut owner_tokens: Vec<u128> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerTokens(owner.clone()))
        .unwrap_or_else(|| Vec::new(env));
    
    let index = owner_tokens.first_index_of(&token_id);
    if let Some(idx) = index {
        owner_tokens.remove(idx);
        
        if owner_tokens.is_empty() {
            env.storage().persistent().remove(&DataKey::OwnerTokens(owner.clone()));
        } else {
            env.storage().persistent().set(&DataKey::OwnerTokens(owner.clone()), &owner_tokens);
            // Extend TTL for the updated owner token list
            env.storage().persistent().extend_ttl(
                &DataKey::OwnerTokens(owner.clone()),
                26280,
                26280
            );
        }
    }
    
    // Emit burn event
    env.events().publish(
        ("burn", token_id),
        owner.clone()
    );
    
    log!(env, "NFT burned: {} by {}", token_id, owner);
    
    Ok(())
}

// Helper function to validate metadata
#[allow(dead_code)]
fn validate_metadata(
    env: &Env,
    title: &String,
    description: &String
) -> Result<(), NFTError> {
    if title.len() == 0 {
        log!(env, "Invalid metadata: empty title");
        return Err(NFTError::InvalidMetadata);
    }
    
    if description.len() == 0 {
        log!(env, "Invalid metadata: empty description");
        return Err(NFTError::InvalidMetadata);
    }
    
    if title.len() > 100 {
        log!(env, "Invalid metadata: title too long ({} > 100)", title.len());
        return Err(NFTError::MetadataTooLarge);
    }
    
    if description.len() > 1000 {
        log!(env, "Invalid metadata: description too long ({} > 1000)", description.len());
        return Err(NFTError::MetadataTooLarge);
    }
    
    Ok(())
}

// Helper function to build attribute map
fn build_attribute_map(
    env: &Env,
    attributes: Vec<(String, String)>
) -> Map<String, String> {
    let mut attr_map = Map::new(env);
    for (key, value) in attributes.iter() {
        attr_map.set(key.clone(), value.clone());
    }
    attr_map
}