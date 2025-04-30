use soroban_sdk::{Address, Env, String, Vec, contracttype};

use crate::error::ContractError;
use crate::participation::Participation;

// Storage lifetime constants
const DAY_IN_LEDGERS: u32 = 17280; // Assuming 5 seconds per ledger
const LIFETIME_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS; // 30 days

// Storage keys for contract data
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                         // Admin address
    Organization(Address),         // Maps organization address to its name
    OrganizationList,              // List of all registered organizations
    ParticipationRecord(ParticipationKey), // Maps (volunteer, task_id) to timestamp
    VolunteerParticipations(Address), // Maps volunteer to list of participations
    TaskVolunteers(String),        // Maps task_id to list of volunteer addresses
}

#[derive(Clone, Eq, PartialEq)]
#[contracttype]
pub struct ParticipationKey {
    pub volunteer: Address,
    pub task_id: String,
}

// Admin functions
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn check_admin(env: &Env, admin: &Address) -> Result<(), ContractError> {
    if get_admin(env) != *admin {
        return Err(ContractError::NotAuthorized);
    }
    Ok(())
}

// Organization storage functions
pub fn store_organization(env: &Env, organization: &Address, name: &String) {
    env.storage().instance().set(&DataKey::Organization(organization.clone()), name);
    
    // Add to list of organizations
    let key = DataKey::OrganizationList;
    let mut organizations: Vec<Address> = env.storage()
        .instance()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    
    organizations.push_back(organization.clone());
    env.storage().instance().set(&key, &organizations);
    
    // Set persistent storage
    env.storage().instance().extend_ttl(LIFETIME_THRESHOLD, LIFETIME_THRESHOLD);
}

pub fn remove_organization_from_storage(env: &Env, organization: &Address) {
    // Remove organization record
    env.storage().instance().remove(&DataKey::Organization(organization.clone()));
    
    // Remove from list
    let key = DataKey::OrganizationList;
    let organizations: Vec<Address> = env.storage()
        .instance()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    
    let mut updated_orgs = Vec::new(env);
    for org in organizations.iter() {
        if org != *organization {
            updated_orgs.push_back(org);
        }
    }
    
    env.storage().instance().set(&key, &updated_orgs);
    env.storage().instance().extend_ttl(LIFETIME_THRESHOLD, LIFETIME_THRESHOLD);
}

pub fn is_organization_registered(env: &Env, organization: &Address) -> bool {
    env.storage().instance().has(&DataKey::Organization(organization.clone()))
}

// Participation storage functions
pub fn store_participation(
    env: &Env, 
    volunteer: &Address,
    task_id: &String,
    task_name: &String,
    timestamp: u64,
) {
    let key = ParticipationKey {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
    };
    
    // Store participation record
    let participation = Participation {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
        task_name: task_name.clone(),
        timestamp,
    };
    
    env.storage().instance().set(&DataKey::ParticipationRecord(key.clone()), &participation);
    
    // Update volunteer's participation list
    let volunteer_key = DataKey::VolunteerParticipations(volunteer.clone());
    let mut volunteer_participations: Vec<Participation> = env.storage()
        .instance()
        .get(&volunteer_key)
        .unwrap_or_else(|| Vec::new(env));
    
    volunteer_participations.push_back(participation.clone());
    env.storage().instance().set(&volunteer_key, &volunteer_participations);
    
    // Update task's volunteer list
    let task_key = DataKey::TaskVolunteers(task_id.clone());
    let mut task_volunteers: Vec<Address> = env.storage()
        .instance()
        .get(&task_key)
        .unwrap_or_else(|| Vec::new(env));
    
    task_volunteers.push_back(volunteer.clone());
    env.storage().instance().set(&task_key, &task_volunteers);
    
    // Extend storage lifetime
    env.storage().instance().extend_ttl(LIFETIME_THRESHOLD, LIFETIME_THRESHOLD);
}

pub fn has_participation(env: &Env, volunteer: &Address, task_id: &String) -> bool {
    let key = ParticipationKey {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
    };
    
    env.storage().instance().has(&DataKey::ParticipationRecord(key))
}

pub fn get_participation(
    env: &Env,
    volunteer: &Address,
    task_id: &String,
) -> Option<Participation> {
    let key = ParticipationKey {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
    };
    
    env.storage().instance().get(&DataKey::ParticipationRecord(key))
}

pub fn get_volunteer_participations(env: &Env, volunteer: &Address) -> Vec<Participation> {
    let key = DataKey::VolunteerParticipations(volunteer.clone());
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn get_task_volunteers(env: &Env, task_id: &String) -> Vec<Address> {
    let key = DataKey::TaskVolunteers(task_id.clone());
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}