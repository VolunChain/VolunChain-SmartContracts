use crate::storage::{get_nft, increment_token_count, is_authorized_minter, save_nft};
use crate::types::{NFT, NFTError, NFTMetadata, DataKey};
use soroban_sdk::{Address, Env, Map, String, Symbol, Vec};

pub fn mint_nft(
    env: &Env,
    minter: Address,
    recipient: Address,
    title: String,
    description: String,
    attributes: Vec<(String, String)>,
    transferable: bool
) -> Result<u128, NFTError> {
    // Require auth from minter
    minter.require_auth();
    
    // Check if minter is authorized
    if !is_authorized_minter(env, &minter) {
        return Err(NFTError::UnauthorizedMinter);
    }
    
    // Validate metadata
    if title.len() == 0 || description.len() == 0 {
        return Err(NFTError::InvalidMetadata);
    }
    
    // Convert attributes to Map
    let mut attr_map = Map::new(env);
    for (key, value) in attributes.iter() {
        attr_map.set(key.clone(), value.clone());
    }
    
    // Create metadata
    let metadata = NFTMetadata {
        issuer: minter.clone(),
        title,
        description,
        creation_date: env.ledger().timestamp(),
        attributes: attr_map,
    };
    
    // Generate new token ID
    let token_id = increment_token_count(env);
    
    // Create NFT
    let nft = NFT {
        id: token_id,
        owner: recipient.clone(),
        metadata,
        transferable,
        minted_at: env.ledger().timestamp(),
    };
    
    // Store NFT
    save_nft(env, &nft);
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "nft_minted"), recipient.clone()),
        token_id,
    );
    
    Ok(token_id)
}

pub fn burn_nft(
    env: &Env,
    owner: Address,
    token_id: u128
) -> Result<(), NFTError> {
    // Require auth from owner
    owner.require_auth();
    
    // Get NFT
    let nft = get_nft(env, token_id)?;
    
    // Verify ownership
    if nft.owner != owner {
        return Err(NFTError::NotTokenOwner);
    }
    
    // Remove NFT
    env.storage().persistent().remove(&DataKey::NFT(token_id));
    
    // Update owner's token list by removing this token
    if let Some(mut token_ids) = env.storage().persistent().get::<_, Vec<u128>>(&DataKey::OwnerTokens(owner.clone())) {
        let position = token_ids.first_index_of(&token_id);
        if let Some(pos) = position {
            token_ids.remove(pos);
            env.storage().persistent().set(&DataKey::OwnerTokens(owner.clone()), &token_ids);
        }
    }
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "nft_burned"), owner.clone()),
        token_id,
    );
    
    Ok(())
}