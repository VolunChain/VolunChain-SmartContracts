#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod client;
mod events;
mod governance;
mod storage;
mod types;

use types::{DaoConfig, DaoError, Proposal, ProposalType, VoteType};

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContract {
    // Initialize the DAO contract
    pub fn initialize(
        env: Env,
        admin: Address,
        nft_contract: Address,
        reputation_contract: Address,
        proposal_creation_threshold: u64,
        execution_delay: u64,
        min_voting_period: u64,
    ) -> Result<(), DaoError> {
        if storage::get_proposal_count(&env) > 0 {
            return Err(DaoError::AlreadyInitialized);
        }

        let config = DaoConfig {
            admin,
            nft_contract,
            reputation_contract,
            proposal_creation_threshold,
            execution_delay,
            min_voting_period,
        };
        storage::save_config(&env, &config);
        events::emit_contract_initialized(&env);

        Ok(())
    }

    // Create a new proposal
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_period: u64,
        minimum_quorum: u64,
        minimum_approval: u64,
    ) -> Result<u32, DaoError> {
        proposer.require_auth();
        let proposal_id = governance::create_proposal(
            &env,
            &proposer,
            title,
            description,
            proposal_type,
            voting_period,
            minimum_quorum,
            minimum_approval,
        )?;
        events::emit_proposal_created(&env, proposal_id, proposer);

        Ok(proposal_id)
    }

    // Cast a vote on a proposal
    pub fn cast_vote(
        env: Env,
        voter: Address,
        proposal_id: u32,
        vote_type: VoteType,
    ) -> Result<(), DaoError> {
        voter.require_auth();
        governance::cast_vote(&env, &voter, proposal_id, vote_type)?;
        let support = match vote_type {
            VoteType::Upvote => true,
            VoteType::Downvote => false,
        };

        events::emit_vote_cast(&env, proposal_id, voter, support);
        Ok(())
    }

    // Finalize a proposal
    pub fn finalize_proposal(env: Env, _caller: Address, proposal_id: u32) -> Result<(), DaoError> {
        governance::finalize_proposal(&env, proposal_id)
    }

    // Execute a passed proposal
    pub fn execute_proposal(env: Env, caller: Address, proposal_id: u32) -> Result<(), DaoError> {
        caller.require_auth();
        governance::execute_proposal(&env, proposal_id)?;
        events::emit_proposal_executed(&env, proposal_id);
        Ok(())
    }

    // Get a single proposal
    pub fn get_proposal(env: Env, id: u32) -> Result<Proposal, DaoError> {
        storage::get_proposal(&env, id)
    }

    // Get all proposals
    pub fn get_all_proposals(env: Env) -> Vec<Proposal> {
        storage::get_all_proposals(&env)
    }

    // Get voting results for a proposal
    pub fn get_proposal_results(env: Env, id: u32) -> Result<(u64, u64), DaoError> {
        governance::get_proposal_results(&env, id)
    }

    // Get a user's voting power
    pub fn get_voting_power(env: Env, user: Address) -> u64 {
        let config = storage::get_config(&env);
        client::calculate_voting_power(
            &env,
            &config.nft_contract,
            &config.reputation_contract,
            &user,
        )
    }

    // Check if a user has voted on a proposal
    pub fn has_voted(env: Env, proposal_id: u32, voter: Address) -> bool {
        storage::has_voted(&env, proposal_id, &voter)
    }

    // Update contract configuration (admin only)
    pub fn update_config(
        env: Env,
        caller: Address,
        new_admin: Option<Address>,
        new_nft_contract: Option<Address>,
        new_reputation_contract: Option<Address>,
        new_threshold: Option<u64>,
        new_execution_delay: Option<u64>,
        new_min_voting_period: Option<u64>,
    ) -> Result<(), DaoError> {
        caller.require_auth();
        let mut config = storage::get_config(&env);
        if caller != config.admin {
            return Err(DaoError::Unauthorized);
        }

        if let Some(admin) = new_admin {
            config.admin = admin;
        }
        if let Some(nft_contract) = new_nft_contract {
            config.nft_contract = nft_contract;
        }
        if let Some(reputation_contract) = new_reputation_contract {
            config.reputation_contract = reputation_contract;
        }
        if let Some(threshold) = new_threshold {
            config.proposal_creation_threshold = threshold;
        }
        if let Some(delay) = new_execution_delay {
            config.execution_delay = delay;
        }
        if let Some(period) = new_min_voting_period {
            config.min_voting_period = period;
        }

        storage::save_config(&env, &config);

        events::emit_config_updated(&env);
        Ok(())
    }
}
