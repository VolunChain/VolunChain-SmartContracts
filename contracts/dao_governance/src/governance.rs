// src/governance.rs
use crate::client::calculate_voting_power;
use crate::events::{
    emit_proposal_created, emit_proposal_executed, emit_proposal_finalized, emit_vote_cast,
};
use crate::storage::{
    get_config, get_proposal, get_proposal_count, has_voted, increment_proposal_count,
    save_proposal, save_vote,
};
use crate::types::{DaoError, Proposal, ProposalStatus, ProposalType, Vote, VoteType};
use core::cmp::max;
use soroban_sdk::{Address, Env, String, Vec};

pub fn create_proposal(
    env: &Env,
    proposer: &Address,
    title: String,
    description: String,
    proposal_type: ProposalType,
    voting_period: u64,
    minimum_quorum: u64,
    minimum_approval: u64,
) -> Result<u32, DaoError> {
    let config = get_config(env);
    let voting_power = calculate_voting_power(
        env,
        &config.nft_contract,
        &config.reputation_contract,
        proposer,
    );

    if voting_power < config.proposal_creation_threshold {
        return Err(DaoError::InsufficientVotingPower);
    }

    let actual_voting_period = max(voting_period, config.min_voting_period);

    let id = increment_proposal_count(env);
    let now = env.ledger().timestamp();

    let proposal = Proposal {
        id,
        title,
        description,
        proposal_type,
        proposer: proposer.clone(),
        start_time: now,
        end_time: now + actual_voting_period,
        status: ProposalStatus::Active,
        upvotes: 0,
        downvotes: 0,
        minimum_quorum,
        minimum_approval,
        executed: false,
    };

    save_proposal(env, &proposal);
    emit_proposal_created(env, proposal.id, proposal.proposer.clone());
    Ok(id)
}

pub fn cast_vote(
    env: &Env,
    voter: &Address,
    proposal_id: u32,
    vote_type: VoteType,
) -> Result<(), DaoError> {
    let mut proposal = get_proposal(env, proposal_id)?;
    if !matches!(proposal.status, ProposalStatus::Active) {
        return Err(DaoError::ProposalNotActive);
    }
    let now = env.ledger().timestamp();
    if now > proposal.end_time {
        return Err(DaoError::VotingEnded);
    }
    if has_voted(env, proposal_id, voter) {
        return Err(DaoError::AlreadyVoted);
    }

    let config = get_config(env);
    let voting_power = calculate_voting_power(
        env,
        &config.nft_contract,
        &config.reputation_contract,
        voter,
    );

    let vote = Vote {
        voter: voter.clone(),
        proposal_id,
        vote_type: vote_type.clone(),
        voting_power,
        timestamp: now,
    };

    save_vote(env, &vote);

    match vote_type {
        VoteType::Upvote => proposal.upvotes += voting_power,
        VoteType::Downvote => proposal.downvotes += voting_power,
    }

    save_proposal(env, &proposal);
    // Emit vote event: support is true if VoteType::For, false otherwise.
    let support = matches!(vote_type, VoteType::Upvote);
    emit_vote_cast(env, vote.proposal_id, vote.voter.clone(), support);
    Ok(())
}

pub fn finalize_proposal(env: &Env, proposal_id: u32) -> Result<(), DaoError> {
    let mut proposal = get_proposal(env, proposal_id)?;
    let now = env.ledger().timestamp();
    if now <= proposal.end_time {
        return Err(DaoError::VotingNotEnded);
    }
    if !matches!(proposal.status, ProposalStatus::Active) {
        return Err(DaoError::ProposalNotActive);
    }

    let total_votes = proposal.upvotes + proposal.downvotes;
    let config = get_config(env);
    if total_votes < proposal.minimum_quorum {
        proposal.status = ProposalStatus::Rejected;
        save_proposal(env, &proposal);
        emit_proposal_finalized(env, proposal_id, false);
        return Ok(());
    }

    let approval_percentage = (proposal.upvotes * 100) / total_votes;
    if approval_percentage >= proposal.minimum_approval {
        proposal.status = ProposalStatus::Passed;
    } else {
        proposal.status = ProposalStatus::Rejected;
    }

    save_proposal(env, &proposal);
    let approved = matches!(proposal.status, ProposalStatus::Passed);
    emit_proposal_finalized(env, proposal_id, approved);
    Ok(())
}

pub fn execute_proposal(env: &Env, proposal_id: u32) -> Result<(), DaoError> {
    let mut proposal = get_proposal(env, proposal_id)?;
    if !matches!(proposal.status, ProposalStatus::Passed) {
        return Err(DaoError::ProposalNotPassed);
    }
    if proposal.executed {
        return Err(DaoError::ProposalAlreadyExecuted);
    }
    let now = env.ledger().timestamp();
    let config = get_config(env);
    if now < proposal.end_time + config.execution_delay {
        return Err(DaoError::ExecutionDelayNotMet);
    }
    proposal.executed = true;
    save_proposal(env, &proposal);

    emit_proposal_executed(env, proposal_id);
    Ok(())
}

pub fn get_proposal_results(env: &Env, proposal_id: u32) -> Result<(u64, u64), DaoError> {
    let proposal = get_proposal(env, proposal_id)?;
    Ok((proposal.upvotes, proposal.downvotes))
}
