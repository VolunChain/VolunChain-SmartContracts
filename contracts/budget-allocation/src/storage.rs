use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn is_organization_authorized(env: &Env, org: &Address) -> bool {
    let organizations: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::Organizations)
        .unwrap_or_else(|| Vec::new(&env));

    organizations.contains(org)
}

pub fn get_project_owner(env: &Env, project_id: u32) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::ProjectOwner(project_id))
        .unwrap()
}

pub fn get_project_org(env: &Env, project_id: u32) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::ProjectOrg(project_id))
        .unwrap()
}

pub fn next_project_id(env: &Env) -> u32 {
    let project_id = env
        .storage()
        .instance()
        .get(&DataKey::NextProjectId)
        .unwrap_or(1u32);

    env.storage()
        .instance()
        .set(&DataKey::NextProjectId, &(project_id + 1));

    project_id
}

pub fn get_milestones(env: &Env, project_id: u32) -> Vec<Milestone> {
    env.storage()
        .instance()
        .get(&DataKey::Milestones(project_id))
        .unwrap_or_else(|| Vec::new(&env))
}

pub fn get_requests(env: &Env, project_id: u32) -> Vec<FundRequest> {
    env.storage()
        .instance()
        .get(&DataKey::FundRequests(project_id))
        .unwrap_or_else(|| Vec::new(&env))
}
