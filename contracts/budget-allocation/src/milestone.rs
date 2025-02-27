use crate::storage;
use crate::transaction;
use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn complete_milestone(env: Env, org: Address, project_id: u32, milestone_id: u32) {
    org.require_auth();

    // Verify organization is authorized and owns the project
    let project_org: Address = env
        .storage()
        .instance()
        .get(&DataKey::ProjectOrg(project_id))
        .unwrap();
    if project_org != org {
        panic!("Only the project's organization can mark milestones as completed");
    }

    // Update milestone status
    let mut milestones: Vec<Milestone> = env
        .storage()
        .instance()
        .get(&DataKey::Milestones(project_id))
        .unwrap();

    let mut milestone = milestones.get(milestone_id).unwrap();

    if milestone.completed {
        panic!("Milestone already completed");
    }

    milestone.completed = true;
    milestones.set(milestone_id, milestone);
    env.storage()
        .instance()
        .set(&DataKey::Milestones(project_id), &milestones);
}

pub fn request_funds(env: Env, requester: Address, project_id: u32, milestone_id: u32) -> u32 {
    requester.require_auth();

    // Verify requester is project owner
    let project_owner: Address = env
        .storage()
        .instance()
        .get(&DataKey::ProjectOwner(project_id))
        .unwrap();
    if project_owner != requester {
        panic!("Only project owner can request funds");
    }

    // Check if milestone is completed but not yet released
    let milestones: Vec<Milestone> = env
        .storage()
        .instance()
        .get(&DataKey::Milestones(project_id))
        .unwrap();

    let milestone = milestones.get(milestone_id).unwrap();

    if !milestone.completed {
        panic!("Milestone not completed yet");
    }

    if milestone.released {
        panic!("Funds already released for this milestone");
    }

    // Create fund request
    let mut fund_requests: Vec<FundRequest> = env
        .storage()
        .instance()
        .get(&DataKey::FundRequests(project_id))
        .unwrap_or_else(|| Vec::new(&env));

    let request_id = fund_requests.len() as u32;

    let request = FundRequest {
        id: request_id,
        milestone_id,
        amount: milestone.amount,
        status: RequestStatus::Pending,
        requester: requester.clone(),
        timestamp: env.ledger().timestamp(),
    };

    fund_requests.push_back(request);
    env.storage()
        .instance()
        .set(&DataKey::FundRequests(project_id), &fund_requests);

    request_id
}

pub fn release_funds(env: Env, org: Address, project_id: u32, request_id: u32) {
    org.require_auth();

    // Verify organization is authorized and owns the project
    let project_org: Address = env
        .storage()
        .instance()
        .get(&DataKey::ProjectOrg(project_id))
        .unwrap();
    if project_org != org {
        panic!("Only the project's organization can release funds");
    }

    // Get the fund request
    let mut fund_requests: Vec<FundRequest> = env
        .storage()
        .instance()
        .get(&DataKey::FundRequests(project_id))
        .unwrap();

    let mut request = fund_requests.get(request_id).unwrap();

    if request.status != RequestStatus::Pending {
        panic!("Request is not in pending state");
    }

    // Update request status
    request.status = RequestStatus::Approved;
    fund_requests.set(request_id, request.clone());
    env.storage()
        .instance()
        .set(&DataKey::FundRequests(project_id), &fund_requests);

    // Update milestone status to released
    let mut milestones: Vec<Milestone> = env
        .storage()
        .instance()
        .get(&DataKey::Milestones(project_id))
        .unwrap();

    let mut milestone = milestones.get(request.milestone_id).unwrap();
    milestone.released = true;
    milestones.set(request.milestone_id, milestone);
    env.storage()
        .instance()
        .set(&DataKey::Milestones(project_id), &milestones);

    // Deduct the amount from project budget
    let mut project_budget: u32 = env
        .storage()
        .instance()
        .get(&DataKey::ProjectBudget(project_id))
        .unwrap();
    
    if project_budget < request.amount {
        panic!("Insufficient project funds");
    }

    project_budget -= request.amount;
    env.storage()
        .instance()
        .set(&DataKey::ProjectBudget(project_id), &project_budget);

    // Record transaction
    let project_owner = env
        .storage()
        .instance()
        .get(&DataKey::ProjectOwner(project_id))
        .unwrap();
    transaction::record_transaction(
        &env,
        project_id,
        request.amount,
        TransactionType::Release,
        &org,
        &project_owner,
    );
}

pub fn get_project_milestones(env: Env, project_id: u32) -> Vec<Milestone> {
    storage::get_milestones(&env, project_id)
}

pub fn get_fund_requests(env: Env, project_id: u32) -> Vec<FundRequest> {
    storage::get_requests(&env, project_id)
}
