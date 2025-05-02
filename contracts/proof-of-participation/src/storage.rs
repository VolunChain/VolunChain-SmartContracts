use soroban_sdk::{Address, Env, String, Vec, contracttype};

use crate::error::ContractError;

// Storage lifetime constants
// Bump these values if more complex operations require more budget
const DAY_IN_LEDGERS: u32 = 17280; // Assuming 5 seconds per ledger
const INSTANCE_LIFETIME_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS; // ~30 days for instance bump
const INSTANCE_BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS; // ~60 days bump amount

const PERSISTENT_LIFETIME_THRESHOLD: u32 = 90 * DAY_IN_LEDGERS; // ~90 days for persistent bump
const PERSISTENT_BUMP_AMOUNT: u32 = 180 * DAY_IN_LEDGERS; // ~180 days bump amount

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                              // Admin address (Instance storage)
    Organization(Address),              // Maps organization address -> name (Persistent)
    OrganizationList,                   // List of all registered organizations (Persistent)
    ParticipationRecord(ParticipationKey),// Maps (volunteer, task_id) -> Participation (Persistent)
    VolunteerParticipations(Address),   // Maps volunteer -> list of ParticipationKeys (Persistent)
    TaskVolunteers(String),             // Maps task_id -> list of volunteer Addresses (Persistent)
    OrgParticipationList(Address),      // Maps organization -> list of ParticipationKeys (Persistent)
}

// Key struct for Participation records
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ParticipationKey {
    pub volunteer: Address,
    pub task_id: String, 
}


// --- Admin Functions ---

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}
pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).expect("Admin not initialized")
}

/// Sets the admin address during initialization.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    // Bump instance TTL on initialization
    bump_instance_ttl(env);
}

pub fn check_admin(env: &Env, admin: &Address) -> Result<(), ContractError> {
    if get_admin(env) != *admin {
        Err(ContractError::NotAuthorized)
    } else {
        Ok(())
    }
}

// --- TTL Bump Helpers ---

/// Bumps the TTL for instance storage.
pub fn bump_instance_ttl(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Bumps the TTL for persistent storage entries related to a specific DataKey.
pub fn bump_persistent_ttl(env: &Env, key: &DataKey) {
     env.storage().persistent().extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

// --- General Storage Helpers ---

/// Helper to get a Vec<T> from storage or return a new empty Vec<T>.
pub fn get_vec_from_persistent_storage<T: soroban_sdk::TryFromVal<Env, soroban_sdk::Val> + soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(
    env: &Env,
    key: &DataKey,
) -> Vec<T> {
    match env.storage().persistent().get(key) {
        Some(vec) => vec,
        None => Vec::new(env),
    }
}