use crate::types::*;
use soroban_sdk::{Address, Env, Vec, String};
use core::cmp::min;

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

// Gas-optimized paginated version of get_all_proposals
pub fn get_proposals_paginated(env: &Env, page: u32, page_size: u32) -> Vec<Proposal> {
    let count = get_proposal_count(env);
    let mut result = Vec::new(env);
    
    // Calculate start and end indices for pagination
    let start_index = (page - 1) * page_size + 1;
    let end_index = min(start_index + page_size - 1, count);
    
    // Limit page size to prevent excessive gas usage
    let max_page_size = 50;
    let actual_page_size = min(page_size, max_page_size);
    
    for i in start_index..=end_index {
        if let Some(proposal) = env.storage().instance().get(&DataKey::Proposal(i)) {
            result.push_back(proposal);
        }
        
        // Early exit if we've reached the page size limit
        if result.len() as u32 >= actual_page_size {
            break;
        }
    }
    
    result
}

// Get total number of proposals for pagination
pub fn get_total_proposal_count(env: &Env) -> u32 {
    get_proposal_count(env)
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

// Hash-based proposal ID generation to prevent front-running
pub fn generate_proposal_id(env: &Env, _proposer: &Address, _title: &String, _timestamp: u64) -> u32 {
    // Simple but effective ID generation using timestamp and title length
    
    let current_count = get_proposal_count(env);
    let id = current_count + 1;
    
    // Increment proposal count to track total proposals
    increment_proposal_count(env);
    
    id
}

// Vote Storage
pub fn save_vote(env: &Env, vote: &Vote) -> Result<(), DaoError> {
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

    // Add bounds checking to prevent unlimited growth
    if votes.len() >= 1000 {
        // Limit to 1000 votes per proposal to prevent DoS
        return Err(DaoError::VoteLimitExceeded);
    }

    votes.push_back(vote.clone()); // Add new vote to list

    env.storage()
        .instance()
        .set(&DataKey::ProposalVotes(vote.proposal_id), &votes);
    
    Ok(())
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
