write clean test.rs for this project: see lib.rs:#![no_std]



use soroban_sdk::{

    contract, contractimpl, Address, Env, String, Vec,

};



mod storage;

mod organization;

mod participation;

mod events;

mod error;

mod test;



pub use error::*;

pub use events::*;

pub use organization::*;

pub use participation::*;

pub use storage::*;



#[contract]

pub struct ProofOfParticipationContract;



#[contractimpl]

impl ProofOfParticipationContract {

    /// Initialize the contract with an admin address

    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {

        if storage::has_admin(&env) {

            return Err(ContractError::AlreadyInitialized);

        }

        admin.require_auth();

        storage::set_admin(&env, &admin);

        Ok(())

    }



    /// Register a new organization

    pub fn register_organization(

        env: Env,

        admin: Address,

        organization: Address,

        name: String,

    ) -> Result<(), ContractError> {

        admin.require_auth();

        organization::register_organization(&env, &admin, &organization, &name)

    }



    /// Remove an organization

    pub fn remove_organization(

        env: Env,

        admin: Address,

        organization: Address,

    ) -> Result<(), ContractError> {

        admin.require_auth();

        organization::remove_organization(&env, &admin, &organization)

    }



    /// Register a participation record

    pub fn register_participation(

        env: Env,

        organization: Address,

        volunteer: Address,

        task_id: String,

        task_name: String,

    ) -> Result<(), ContractError> {

        organization.require_auth();

        participation::register_participation(&env, &organization, &volunteer, &task_id, &task_name)

    }



    /// Verify if a volunteer has participated in a specific task

    pub fn verify_participation(

        env: Env,

        volunteer: Address,

        task_id: String,

    ) -> bool {

        participation::verify_participation(&env, &volunteer, &task_id)

    }



    /// Get the timestamp of a participation

    pub fn get_participation_timestamp(

        env: Env,

        volunteer: Address,

        task_id: String,

    ) -> Result<u64, ContractError> {

        participation::get_participation_timestamp(&env, &volunteer, &task_id)

    }



    /// Get all participations for a volunteer

    pub fn get_volunteer_participations(

        env: Env,

        volunteer: Address,

    ) -> Vec<participation::Participation> {

        participation::get_volunteer_participations(&env, &volunteer)

    }



    /// Get all volunteers who participated in a specific task

    pub fn get_task_volunteers(

        env: Env,

        task_id: String,

    ) -> Vec<Address> {

        participation::get_task_volunteers(&env, &task_id)

    }



    /// Check if an address is a registered organization

    pub fn is_organization(env: Env, organization: Address) -> bool {

        organization::is_organization(&env, &organization)

    }

},organization.rs:use soroban_sdk::{Address, Env, String};



use crate::error::ContractError;

use crate::events;

use crate::storage;



pub fn register_organization(

    env: &Env,

    admin: &Address,

    organization: &Address,

    name: &String,

) -> Result<(), ContractError> {

    // Verify admin authorization

    storage::check_admin(env, admin)?;

   

    if storage::is_organization_registered(env, organization) {

        return Err(ContractError::OrganizationAlreadyRegistered);

    }

   

    storage::store_organization(env, organization, name);

   

    events::organization_registered(env, organization, name);

   

    Ok(())

}





pub fn remove_organization(

    env: &Env,

    admin: &Address,

    organization: &Address,

) -> Result<(), ContractError> {

    storage::check_admin(env, admin)?;

   

    if !storage::is_organization_registered(env, organization) {

        return Err(ContractError::OrganizationNotRegistered);

    }

   

    storage::remove_organization_from_storage(env, organization);

   

    events::organization_removed(env, organization);

   

    Ok(())

}



pub fn is_organization(env: &Env, organization: &Address) -> bool {

    storage::is_organization_registered(env, organization)

}



pub fn verify_organization(env: &Env, organization: &Address) -> Result<(), ContractError> {

    if !storage::is_organization_registered(env, organization) {

        return Err(ContractError::OrganizationNotRegistered);

    }

    Ok(())

}, participation.rs:use soroban_sdk::{Address, Env, String, Vec, contracttype};



use crate::error::ContractError;

use crate::events;

use crate::organization;

use crate::storage;



#[derive(Clone, Debug, Eq, PartialEq)]

#[contracttype]

pub struct Participation {

    pub volunteer: Address,

    pub task_id: String,

    pub task_name: String,

    pub timestamp: u64,

}



pub fn register_participation(

    env: &Env,

    organization: &Address,

    volunteer: &Address,

    task_id: &String,

    task_name: &String,

) -> Result<(), ContractError> {

    organization::verify_organization(env, organization)?;

   

    if storage::has_participation(env, volunteer, task_id) {

        return Err(ContractError::ParticipationAlreadyRegistered);

    }

   

    let timestamp = env.ledger().timestamp();

   

    storage::store_participation(env, volunteer, task_id, task_name, timestamp);

   

    events::participation_registered(env, organization, volunteer, task_id, task_name, timestamp);

   

    Ok(())

}



pub fn verify_participation(

    env: &Env,

    volunteer: &Address,

    task_id: &String,

) -> bool {

    storage::has_participation(env, volunteer, task_id)

}



pub fn get_participation_timestamp(

    env: &Env,

    volunteer: &Address,

    task_id: &String,

) -> Result<u64, ContractError> {

    let participation = storage::get_participation(env, volunteer, task_id)

        .ok_or(ContractError::ParticipationNotFound)?;

   

    Ok(participation.timestamp)

}



pub fn get_volunteer_participations(

    env: &Env,

    volunteer: &Address,

) -> Vec<Participation> {

    storage::get_volunteer_participations(env, volunteer)

}



pub fn get_task_volunteers(

    env: &Env,

    task_id: &String,

) -> Vec<Address> {

    storage::get_task_volunteers(env, task_id)

}, error.rs:use soroban_sdk::contracterror;



#[contracterror]

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]

#[repr(u32)]

pub enum ContractError {

    // General errors

    AlreadyInitialized = 1,

    NotAuthorized = 2,

   

    // Organization errors

    OrganizationAlreadyRegistered = 101,

    OrganizationNotRegistered = 102,

   

    // Participation errors

    ParticipationAlreadyRegistered = 201,

    ParticipationNotFound = 202,

}, event.rs:use soroban_sdk::{Address, Env, String, Symbol, contracttype};



#[contracttype]

pub enum ParticipationEventType {

    OrganizationRegistered,

    OrganizationRemoved,

    ParticipationRegistered,

}



// Emit an event when an organization is registered

pub fn organization_registered(env: &Env, organization: &Address, name: &String) {

    let topics = (Symbol::new(env, "organization_registered"), organization.clone());

    env.events().publish(topics, name.clone());

}



// Emit an event when an organization is removed

pub fn organization_removed(env: &Env, organization: &Address) {

    let topics = (Symbol::new(env, "organization_removed"), organization.clone());

    env.events().publish(topics, ());

}



// Emit an event when a participation is registered

pub fn participation_registered(

    env: &Env,

    organization: &Address,

    volunteer: &Address,

    task_id: &String,

    task_name: &String,

    timestamp: u64,

) {

    let topics = (

        Symbol::new(env, "participation_registered"),

        organization.clone(),

        volunteer.clone(),

        task_id.clone(),

    );

   

    // Create event data with task name and timestamp

    let data = (task_name.clone(), timestamp);

   

    env.events().publish(topics, data);

},storage.rs: use soroban_sdk::{Address, Env, String, Vec, contracttype};



use crate::error::ContractError;

use crate::participation::Participation;



// Storage lifetime constants

const DAY_IN_LEDGERS: u32 = 17280; // Assuming 5 seconds per ledger

const LIFETIME_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS; // 30 days



// Storage keys for contract data

#[derive(Clone)]

#[contracttype]

pub enum DataKey {

    Admin,                         // Admin address

    Organization(Address),         // Maps organization address to its name

    OrganizationList,              // List of all registered organizations

    ParticipationRecord(ParticipationKey), // Maps (volunteer, task_id) to timestamp

    VolunteerParticipations(Address), // Maps volunteer to list of participations

    TaskVolunteers(String),        // Maps task_id to list of volunteer addresses

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