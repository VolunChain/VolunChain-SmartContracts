use crate::storage::{get_admin, bump_instance, set_contract_version, set_uri_base};
use crate::storage as storage_mod;
use crate::types::NFTError;
use soroban_sdk::{Address, Env, String, log};

// Constants
const CURRENT_CONTRACT_VERSION: u32 = 1;
const PAUSED_FLAG: u32 = 1;
const UNPAUSED_FLAG: u32 = 0;

// Initialize contract
pub fn initialize(env: &Env, admin: Address) -> Result<(), NFTError> {
    // Check if admin is already set
    if env.storage().instance().has(&crate::types::DataKey::Admin) {
        return Err(NFTError::ContractAlreadyInitialized);
    }
    
    // Store admin address
    env.storage().instance().set(&crate::types::DataKey::Admin, &admin);
    
    // Initialize authorized minters with admin
    let minters = soroban_sdk::vec![env, admin.clone()];
    env.storage().instance().set(&crate::types::DataKey::AuthorizedMinters, &minters);
    
    // Initialize token count
    env.storage().instance().set(&crate::types::DataKey::TokenCount, &0u128);
    
    // Set initial contract version
    set_contract_version(env, CURRENT_CONTRACT_VERSION);
    
    // Initialize paused state to unpaused
    env.storage().instance().set(&crate::types::DataKey::ContractVersion, &UNPAUSED_FLAG);
    
    // Bump instance storage
    bump_instance(env);
    
    log!(env, "Contract initialized with admin: {}", admin);
    
    Ok(())
}

// Add a new authorized minter
pub fn add_authorized_minter(env: &Env, admin: Address, minter: Address) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Check contract is not paused
    verify_not_paused(env)?;
    
    // Add minter to authorized list
    storage_mod::add_authorized_minter(env, minter.clone());
    
    Ok(())
}

// Remove an authorized minter
pub fn remove_authorized_minter(env: &Env, admin: Address, minter: Address) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Check contract is not paused
    verify_not_paused(env)?;
    
    // Remove minter from authorized list
    storage_mod::remove_authorized_minter(env, minter.clone());
    
    Ok(())
}

// Set contract URI base
pub fn set_uri_base_for_tokens(
    env: &Env,
    admin: Address,
    base_uri: String,
    suffix: String
) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Check contract is not paused
    verify_not_paused(env)?;
    
    // Set the URI base
    set_uri_base(env, base_uri, suffix);
    
    Ok(())
}

// Pause the contract
pub fn pause_contract(env: &Env, admin: Address) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Set paused flag
    env.storage().instance().set(&crate::types::DataKey::ContractVersion, &PAUSED_FLAG);
    
    // Bump instance storage
    bump_instance(env);
    
    log!(env, "Contract paused by admin: {}", admin);
    
    Ok(())
}

// Unpause the contract
pub fn unpause_contract(env: &Env, admin: Address) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Set unpaused flag
    env.storage().instance().set(&crate::types::DataKey::ContractVersion, &UNPAUSED_FLAG);
    
    // Bump instance storage
    bump_instance(env);
    
    log!(env, "Contract unpaused by admin: {}", admin);
    
    Ok(())
}

// Upgrade contract version (for future use)
pub fn upgrade_contract(env: &Env, admin: Address, new_version: u32) -> Result<(), NFTError> {
    // Verify admin authorization
    verify_admin(env, &admin)?;
    
    // Set new contract version
    set_contract_version(env, new_version);
    
    log!(env, "Contract upgraded to version {} by admin: {}", new_version, admin);
    
    Ok(())
}

// Helper: Verify caller is admin
fn verify_admin(env: &Env, caller: &Address) -> Result<(), NFTError> {
    caller.require_auth();
    
    let admin = get_admin(env)?;
    if *caller != admin {
        log!(env, "Admin check failed: {} is not admin", caller);
        return Err(NFTError::AdminRequired);
    }
    
    Ok(())
}

// Helper: Verify contract is not paused
fn verify_not_paused(env: &Env) -> Result<(), NFTError> {
    let paused: u32 = env.storage()
        .instance()
        .get(&crate::types::DataKey::ContractVersion)
        .unwrap_or(UNPAUSED_FLAG);
    
    if paused == PAUSED_FLAG {
        return Err(NFTError::ContractPaused);
    }
    
    Ok(())
}