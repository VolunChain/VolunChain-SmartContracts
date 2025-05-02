use soroban_sdk::{Address, Env, String, Vec};

use crate::error::ContractError;
use crate::events;
use crate::organization;
use crate::storage;
use crate::participation_storage::{
    self, Participation, MAX_TASK_NAME_LEN, MAX_METADATA_LEN,
    paginate_vec, get_participations_from_keys
};


pub fn register_participation(
    env: &Env,
    organization: &Address,
    volunteer: &Address,
    task_id: &String,
    task_name: &String,
    metadata: Option<String>, 
) -> Result<(), ContractError> {

    organization.require_auth();
    organization::verify_organization(env, organization)?;

    // Validate input lengths
    if task_name.len() > MAX_TASK_NAME_LEN {
        return Err(ContractError::TaskNameTooLong);
    }
    if let Some(ref m) = metadata {
        if m.len() > MAX_METADATA_LEN {
            return Err(ContractError::MetadataTooLong);
        }
    }

    // Check if participation already exists
    if participation_storage::has_participation(env, volunteer, task_id) {
        return Err(ContractError::ParticipationAlreadyRegistered);
    }

    let timestamp = env.ledger().timestamp();

    // Store using participation storage function
    participation_storage::store_participation(
        env,
        organization,
        volunteer,
        task_id,
        task_name,
        &metadata,
        timestamp,
    );

    events::participation_registered(env, organization, volunteer, task_id, task_name, &metadata, timestamp);

    Ok(())
}

pub fn verify_participation(
    env: &Env,
    volunteer: &Address,
    task_id: &String,
) -> bool {
    participation_storage::has_participation(env, volunteer, task_id)
}


pub fn get_participation_details(
    env: &Env,
    volunteer: &Address,
    task_id: &String,
) -> Result<Participation, ContractError> {
    participation_storage::get_participation(env, volunteer, task_id)
        .ok_or(ContractError::ParticipationNotFound)
}



pub fn get_volunteer_participations(
    env: &Env,
    volunteer: &Address,
    offset: u32,
    limit: u32,
) -> Result<Vec<Participation>, ContractError> {

    let participation_keys: Vec<storage::ParticipationKey> = participation_storage::get_volunteer_participation_keys(env, volunteer);
    let paginated_keys = paginate_vec(env, &participation_keys, offset, limit)?;
    let participations = get_participations_from_keys(env, &paginated_keys);

    Ok(participations)
}

pub fn get_task_volunteers(
    env: &Env,
    task_id: &String,
    offset: u32,
    limit: u32,
) -> Result<Vec<Address>, ContractError> {
    let volunteers: Vec<Address> = participation_storage::get_task_volunteers_list(env, task_id);
    paginate_vec(env, &volunteers, offset, limit)
}

pub fn get_organization_participations(
    env: &Env,
    organization: &Address,
    offset: u32,
    limit: u32,
) -> Result<Vec<Participation>, ContractError> {
   
    let participation_keys = participation_storage::get_organization_participation_keys(env, organization);
    let paginated_keys = paginate_vec(env, &participation_keys, offset, limit)?;
    let participations = get_participations_from_keys(env, &paginated_keys);

    Ok(participations)
}