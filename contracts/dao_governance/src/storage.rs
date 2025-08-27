use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

// Proposal Storage
pub fn save_proposal(env: &Env, proposal: &Proposal) {
    env.storage()
        .instance()
        .set(&DataKey::Proposal(proposal.id), proposal);
}

pub fn get_proposal(env: &Env, id: u32) -> Result<Proposal, DaoError> {
    env.storage()
        .instance()
        .get(&DataKey::Proposal(id))
        .ok_or(DaoError::ProposalNotFound)
}

pub fn get_all_proposals(env: &Env) -> Vec<Proposal> {
    let count = get_proposal_count(env);
    let mut result = Vec::new(env);

    for i in 1..=count {
        if let Some(proposal) = env.storage().instance().get(&DataKey::Proposal(i)) {
            result.push_back(proposal);
        }
    }
    result
}

pub fn get_proposal_count(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::ProposalCount)
        .unwrap_or(0)
}

pub fn increment_proposal_count(env: &Env) -> u32 {
    let count = get_proposal_count(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::ProposalCount, &count);
    count
}

// Vote Storage
pub fn save_vote(env: &Env, vote: &Vote) {
    // Save individual vote record
    env.storage()
        .instance()
        .set(&DataKey::Vote(vote.proposal_id, vote.voter.clone()), vote);

    // Also save to proposal votes list
    let mut votes: Vec<Vote> = env
        .storage()
        .instance()
        .get(&DataKey::ProposalVotes(vote.proposal_id))
        .unwrap_or_else(|| Vec::new(env));

    votes.push_back(vote.clone()); // Add new vote to list

    env.storage()
        .instance()
        .set(&DataKey::ProposalVotes(vote.proposal_id), &votes);
}

pub fn has_voted(env: &Env, proposal_id: u32, voter: &Address) -> bool {
    env.storage()
        .instance()
        .has(&DataKey::Vote(proposal_id, voter.clone()))
}

#[allow(dead_code)]
pub fn get_vote(env: &Env, proposal_id: u32, voter: &Address) -> Option<Vote> {
    env.storage()
        .instance()
        .get(&DataKey::Vote(proposal_id, voter.clone()))
}

#[allow(dead_code)]
pub fn get_votes_for_proposal(env: &Env, proposal_id: u32) -> Vec<Vote> {
    env.storage()
        .instance()
        .get(&DataKey::ProposalVotes(proposal_id))
        .unwrap_or_else(|| Vec::new(env))
}

// DAO Config Storage
pub fn save_config(env: &Env, config: &DaoConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn get_config(env: &Env) -> DaoConfig {
    env.storage()
        .instance()
        .get(&DataKey::Config)
        .expect("DAO config not initialized")
}
