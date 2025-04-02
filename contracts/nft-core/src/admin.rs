use crate::storage::{add_authorized_minter as add_minter, get_admin, remove_authorized_minter as remove_minter};
use crate::types::{DataKey, NFTError};
use soroban_sdk::{Address, Env, Symbol, Vec};

pub fn initialize(env: &Env, admin: Address) -> Result<(), NFTError> {
    // Check if already initialized
    if env.storage().instance().has(&DataKey::Admin) {
        return Err(NFTError::ContractAlreadyInitialized);
    }
    
    // Require auth from admin
    admin.require_auth();
    
    // Set admin
    env.storage().instance().set(&DataKey::Admin, &admin);
    
    // Initialize empty list of authorized minters
    let minters: Vec<Address> = Vec::new(env);
    env.storage().instance().set(&DataKey::AuthorizedMinters, &minters);
    
    // Initialize token counter
    env.storage().instance().set(&DataKey::TokenCount, &0u128);
    
    // Emit initialization event
    env.events().publish(
        (Symbol::new(env, "contract_initialized"), admin.clone()),
        env.ledger().timestamp(),
    );
    
    Ok(())
}

pub fn add_authorized_minter(env: &Env, admin: Address, minter: Address) -> Result<(), NFTError> {
    // Require auth from admin
    admin.require_auth();
    
    // Check if caller is admin
    let stored_admin = get_admin(env)?;
    if stored_admin != admin {
        return Err(NFTError::AdminRequired);
    }
    
    // Add minter to authorized list
    add_minter(env, minter.clone());
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "minter_added"), admin.clone()),
        minter,
    );
    
    Ok(())
}

pub fn remove_authorized_minter(env: &Env, admin: Address, minter: Address) -> Result<(), NFTError> {
    // Require auth from admin
    admin.require_auth();
    
    // Check if caller is admin
    let stored_admin = get_admin(env)?;
    if stored_admin != admin {
        return Err(NFTError::AdminRequired);
    }
    
    // Remove minter from authorized list
    remove_minter(env, minter.clone());
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "minter_removed"), admin.clone()),
        minter,
    );
    
    Ok(())
}