use crate::storage;
use crate::transaction;
use crate::types::*;
use soroban_sdk::{Address, Env, String, Vec};

pub fn allocate_project_budget(
    env: Env,
    org: Address,
    project_owner: Address,
    total_budget: u32,
    milestone_descriptions: Vec<String>,
    milestone_amounts: Vec<u32>,
) -> u32 {
    org.require_auth();

    // Verify organization is authorized
    let organizations: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::Organizations)
        .unwrap();
    if !organizations.contains(&org) {
        panic!("Unauthorized organization");
    }

    // Verify milestone amounts sum up to total budget
    let mut sum = 0u32;
    for amount in milestone_amounts.iter() {
        sum += amount;
    }
    if sum != total_budget {
        panic!("Milestone amounts must sum to total budget");
    }

    // Get next project ID
    let project_id = storage::next_project_id(&env);

    // Store project budget
    env.storage()
        .instance()
        .set(&DataKey::ProjectBudget(project_id), &total_budget);

    // Store project owner and organization
    env.storage()
        .instance()
        .set(&DataKey::ProjectOwner(project_id), &project_owner);
    env.storage()
        .instance()
        .set(&DataKey::ProjectOrg(project_id), &org);

    // Create milestones
    let mut milestones: Vec<Milestone> = Vec::new(&env);
    for i in 0..milestone_descriptions.len() {
        let milestone = Milestone {
            id: i as u32,
            description: milestone_descriptions.get(i).unwrap(),
            amount: milestone_amounts.get(i).unwrap(),
            completed: false,
            released: false,
        };
        milestones.push_back(milestone);
    }
    env.storage()
        .instance()
        .set(&DataKey::Milestones(project_id), &milestones);

    // Initialize fund requests
    let fund_requests: Vec<FundRequest> = Vec::new(&env);
    env.storage()
        .instance()
        .set(&DataKey::FundRequests(project_id), &fund_requests);

    // Record transaction
    transaction::record_transaction(
        &env,
        project_id,
        total_budget,
        TransactionType::Allocation,
        &org,
        &project_owner,
    );

    project_id
}

pub fn get_project_budget(env: Env, project_id: u32) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::ProjectBudget(project_id))
        .unwrap_or(0)
}
