use soroban_sdk::{Address, Env, String, Vec, contracttype};
use crate::storage::{DataKey, ParticipationKey, bump_persistent_ttl, get_vec_from_persistent_storage, bump_instance_ttl};
use crate::error::ContractError;

// Define maximum lengths
pub const MAX_TASK_NAME_LEN: u32 = 64;
pub const MAX_METADATA_LEN: u32 = 128;

// Participation data structure including optional metadata
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Participation {
    pub volunteer: Address,
    pub task_id: String,
    pub task_name: String,
    pub timestamp: u64,
    pub organization: Address,
    pub metadata: Option<String>,
}


pub fn store_participation(
    env: &Env,
    organization: &Address,
    volunteer: &Address,
    task_id: &String,
    task_name: &String,
    metadata: &Option<String>,
    timestamp: u64,
) {
    let p_key = ParticipationKey {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
    };

    let participation = Participation {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
        task_name: task_name.clone(),
        timestamp,
        organization: organization.clone(),
        metadata: metadata.clone(),
    };

    let record_key = DataKey::ParticipationRecord(p_key.clone());
    env.storage().persistent().set(&record_key, &participation);
    bump_persistent_ttl(env, &record_key);

    // Update volunteer's participation list (storing keys now)
    let volunteer_list_key = DataKey::VolunteerParticipations(volunteer.clone());
    let mut volunteer_participation_keys: Vec<ParticipationKey> = get_vec_from_persistent_storage(env, &volunteer_list_key);
    volunteer_participation_keys.push_back(p_key.clone());
    env.storage().persistent().set(&volunteer_list_key, &volunteer_participation_keys);
    bump_persistent_ttl(env, &volunteer_list_key);

    // Update task's volunteer list
    let task_list_key = DataKey::TaskVolunteers(task_id.clone());
    let mut task_volunteers: Vec<Address> = get_vec_from_persistent_storage(env, &task_list_key);
    // Avoid duplicates in task volunteer list
    if !task_volunteers.contains(volunteer) {
        task_volunteers.push_back(volunteer.clone());
        env.storage().persistent().set(&task_list_key, &task_volunteers);
    }
    bump_persistent_ttl(env, &task_list_key);

    // Update organization's participation list
    let org_list_key = DataKey::OrgParticipationList(organization.clone());
    let mut org_participation_keys: Vec<ParticipationKey> = get_vec_from_persistent_storage(env, &org_list_key);
    org_participation_keys.push_back(p_key.clone()); // p_key includes volunteer and task_id
    env.storage().persistent().set(&org_list_key, &org_participation_keys);
    bump_persistent_ttl(env, &org_list_key);

    // Bump instance TTL for general activity
    bump_instance_ttl(env);
}

pub fn has_participation(env: &Env, volunteer: &Address, task_id: &String) -> bool {
    let key = ParticipationKey {
        volunteer: volunteer.clone(),
        task_id: task_id.clone(),
    };
    env.storage().persistent().has(&DataKey::ParticipationRecord(key))
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
    env.storage().persistent().get(&DataKey::ParticipationRecord(key))
}

pub fn get_volunteer_participation_keys(env: &Env, volunteer: &Address) -> Vec<ParticipationKey> {
    get_vec_from_persistent_storage(env, &DataKey::VolunteerParticipations(volunteer.clone()))
}

pub fn get_task_volunteers_list(env: &Env, task_id: &String) -> Vec<Address> {
    get_vec_from_persistent_storage(env, &DataKey::TaskVolunteers(task_id.clone()))
}

pub fn get_organization_participation_keys(env: &Env, organization: &Address) -> Vec<ParticipationKey> {
    get_vec_from_persistent_storage(env, &DataKey::OrgParticipationList(organization.clone()))
}

/// Helper to paginate a Vec<T>.
pub fn paginate_vec<T>(env: &Env, vec: &Vec<T>, offset: u32, limit: u32) -> Result<Vec<T>, ContractError>
where
    T: Clone + soroban_sdk::TryFromVal<Env, soroban_sdk::Val> + soroban_sdk::IntoVal<Env, soroban_sdk::Val>,
{
    let total_len = vec.len();

    if limit == 0 {
        return Err(ContractError::InvalidPaginationArguments); // Limit cannot be zero
    }

    let start = offset;

    if start >= total_len && total_len > 0 {
        return Ok(Vec::new(env));
    }
     if start >= total_len && total_len == 0 {
        return Ok(Vec::new(env)); // Ret
    }

    let end = start.saturating_add(limit);
    let end = end.min(total_len);

    let mut result_vec = Vec::new(env);
    if start < end { 
        for i in start..end {
             match vec.get(i) {
                Some(item) => result_vec.push_back(item),
                None => {
                    return Err(ContractError::InvalidPaginationArguments) 
                }
            }
        }
    }

    Ok(result_vec)
}

pub fn get_participations_from_keys(env: &Env, keys: &Vec<ParticipationKey>) -> Vec<Participation> {
    let mut participations = Vec::new(env);
    for p_key in keys.iter() {
        if let Some(participation) = get_participation(env, &p_key.volunteer, &p_key.task_id) {
            participations.push_back(participation);
        }
    }
     participations
}