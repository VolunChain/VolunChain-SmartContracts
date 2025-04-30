use soroban_sdk::{Address, Env, String, Vec, contracttype};

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
}