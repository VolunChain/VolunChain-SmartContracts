#![cfg(test)]

use super::*;
use soroban_sdk::{
    Address, Env,
    testutils::{Address as _, Ledger},
};
use types::ProposalStatus;

fn with_contract<R, F>(env: &Env, contract_id: &Address, f: F) -> R
where
    F: FnOnce() -> R,
{
    env.as_contract(contract_id, f)
}

fn create_address(env: &Env, _name: &str) -> Address {
    let address = Address::generate(env);
    env.mock_all_auths();
    address
}

fn create_contract(env: &Env) -> Address {
    let contract_id = env.register(DaoContract, ());
    contract_id
}

fn setup_contract(env: &Env) -> (Address, Address, Address, Address) {
    let contract_id = create_contract(env);
    let admin = create_address(env, "admin");
    let nft_contract = create_address(env, "nft_contract");
    let reputation_contract = create_address(env, "reputation_contract");
    let _proposer = create_address(env, "proposer");

    // Initialize the contract
    let _ = with_contract(&env, &contract_id, || {
        DaoContract::initialize(
            env.clone(),
            admin.clone(),
            nft_contract.clone(),
            reputation_contract.clone(),
            100, // proposal_creation_threshold
            86400, // execution_delay (1 day)
            3600, // min_voting_period (1 hour)
        )
    });

    (contract_id, admin, nft_contract, reputation_contract)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = create_contract(&env);
    let admin = create_address(&env, "admin");
    let nft_contract = create_address(&env, "nft_contract");
    let reputation_contract = create_address(&env, "reputation_contract");

    // Test successful initialization
    let _ = with_contract(&env, &contract_id, || {
        DaoContract::initialize(
            env.clone(),
            admin.clone(),
            nft_contract.clone(),
            reputation_contract.clone(),
            100,
            86400,
            3600,
        )
    });

    // Test that we can get the proposal count (should be 0)
    let proposal_count = with_contract(&env, &contract_id, || {
        DaoContract::get_all_proposals(env.clone())
    });
    assert_eq!(proposal_count.len(), 0);
}

#[test]
fn test_initialize_twice() {
    let env = Env::default();
    let contract_id = create_contract(&env);
    let admin = create_address(&env, "admin");
    let nft_contract = create_address(&env, "nft_contract");
    let reputation_contract = create_address(&env, "reputation_contract");

    // First initialization
    let _ = with_contract(&env, &contract_id, || {
        DaoContract::initialize(
            env.clone(),
            admin.clone(),
            nft_contract.clone(),
            reputation_contract.clone(),
            100,
            86400,
            3600,
        )
    });

    // Second initialization should fail
    let result = with_contract(&env, &contract_id, || {
        DaoContract::initialize(
            env.clone(),
            admin.clone(),
            nft_contract.clone(),
            reputation_contract.clone(),
            100,
            86400,
            3600,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_create_proposal() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");

    // Mock voting power for the proposer
    env.mock_all_auths();

    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200, // voting_period (2 hours)
            50,   // minimum_quorum
            60,   // minimum_approval
        )
    }).unwrap();

    assert_eq!(proposal_id, 1);

    // Verify the proposal was created
    let proposal = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal(env.clone(), 1)
    }).unwrap();
    assert_eq!(proposal.title, String::from_str(&env, "Test Proposal"));
    assert_eq!(proposal.description, String::from_str(&env, "This is a test proposal"));
    assert_eq!(proposal.proposal_type, ProposalType::Funding);
    assert_eq!(proposal.proposer, proposer);
    assert_eq!(proposal.upvotes, 0);
    assert_eq!(proposal.downvotes, 0);
    assert_eq!(proposal.minimum_quorum, 50);
    assert_eq!(proposal.minimum_approval, 60);
    assert_eq!(proposal.executed, false);
}

#[test]
fn test_cast_vote() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter = create_address(&env, "voter");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast an upvote
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Verify the vote was recorded
    let has_voted = with_contract(&env, &contract_id, || {
        DaoContract::has_voted(env.clone(), proposal_id, voter.clone())
    });
    assert!(has_voted);

    // Check proposal results
    let (upvotes, downvotes) = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal_results(env.clone(), proposal_id)
    }).unwrap();
    assert_eq!(upvotes, 105); // Mock voting power from client (100 + 5)
    assert_eq!(downvotes, 0);
}

#[test]
fn test_cast_vote_twice() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter = create_address(&env, "voter");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast first vote
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Try to cast second vote - should fail
    let result = with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter.clone(),
            proposal_id,
            VoteType::Downvote,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_finalize_proposal() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter1 = create_address(&env, "voter1");
    let voter2 = create_address(&env, "voter2");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast votes
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter1.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter2.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Fast forward time to end voting period
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Check that proposal was passed
    let proposal = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal(env.clone(), proposal_id)
    }).unwrap();
    assert_eq!(proposal.status, ProposalStatus::Passed);
}

#[test]
fn test_finalize_proposal_insufficient_quorum() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter = create_address(&env, "voter");

    // Create a proposal with high quorum requirement
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            300, // High quorum requirement
            60,
        )
    }).unwrap();

    // Cast only one vote (insufficient for quorum)
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Fast forward time
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Check that proposal was rejected due to insufficient quorum
    let proposal = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal(env.clone(), proposal_id)
    }).unwrap();
    assert_eq!(proposal.status, ProposalStatus::Rejected);
}

#[test]
fn test_execute_proposal() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter1 = create_address(&env, "voter1");
    let voter2 = create_address(&env, "voter2");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast votes to pass the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter1.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter2.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Fast forward to end voting period
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Fast forward past execution delay
    env.ledger().with_mut(|l| {
        l.timestamp = 7201 + 86400 + 1; // execution_delay + 1
    });

    // Execute the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::execute_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Check that proposal was executed
    let proposal = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal(env.clone(), proposal_id)
    }).unwrap();
    assert_eq!(proposal.executed, true);
}

#[test]
fn test_execute_proposal_before_delay() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter1 = create_address(&env, "voter1");
    let voter2 = create_address(&env, "voter2");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast votes to pass the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter1.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter2.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Fast forward to end voting period
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Try to execute before delay period - should fail
    let result = with_contract(&env, &contract_id, || {
        DaoContract::execute_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_get_voting_power() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let user = create_address(&env, "user");

    // Get voting power for a user
    let voting_power = with_contract(&env, &contract_id, || {
        DaoContract::get_voting_power(env.clone(), user.clone())
    });
    
    // Should return the mock voting power from the client (100 + 5 = 105)
    assert_eq!(voting_power, 105);
}

#[test]
fn test_update_config() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let new_admin = create_address(&env, "new_admin");
    let new_nft_contract = create_address(&env, "new_nft_contract");

    // Update configuration
    with_contract(&env, &contract_id, || {
        DaoContract::update_config(
            env.clone(),
            admin.clone(),
            Some(new_admin.clone()),
            Some(new_nft_contract.clone()),
            None,
            Some(200), // new threshold
            None,
            None,
        )
    }).unwrap();

    // Verify the changes by checking voting power (should use new NFT contract)
    let user = create_address(&env, "user");
    let voting_power = with_contract(&env, &contract_id, || {
        DaoContract::get_voting_power(env.clone(), user.clone())
    });
    assert_eq!(voting_power, 105); // Should still work with new contract
}

#[test]
fn test_update_config_unauthorized() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let unauthorized_user = create_address(&env, "unauthorized_user");
    let new_admin = create_address(&env, "new_admin");

    // Try to update config with unauthorized user
    let result = with_contract(&env, &contract_id, || {
        DaoContract::update_config(
            env.clone(),
            unauthorized_user.clone(),
            Some(new_admin.clone()),
            None,
            None,
            None,
            None,
            None,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_get_all_proposals() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer1 = create_address(&env, "proposer1");
    let proposer2 = create_address(&env, "proposer2");

    // Create multiple proposals
    let _proposal_id1 = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer1.clone(),
            String::from_str(&env, "Proposal 1"),
            String::from_str(&env, "First proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    let _proposal_id2 = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer2.clone(),
            String::from_str(&env, "Proposal 2"),
            String::from_str(&env, "Second proposal"),
            ProposalType::Feature,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Get all proposals
    let all_proposals = with_contract(&env, &contract_id, || {
        DaoContract::get_all_proposals(env.clone())
    });
    assert_eq!(all_proposals.len(), 2);
    assert_eq!(all_proposals.get(0).unwrap().title, String::from_str(&env, "Proposal 1"));
    assert_eq!(all_proposals.get(1).unwrap().title, String::from_str(&env, "Proposal 2"));
}

#[test]
fn test_proposal_not_found() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);

    // Try to get a non-existent proposal
    let result = with_contract(&env, &contract_id, || {
        DaoContract::get_proposal(env.clone(), 999)
    });
    assert!(result.is_err());
}

#[test]
fn test_vote_on_non_existent_proposal() {
    let env = Env::default();
    let (contract_id, _admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let voter = create_address(&env, "voter");

    // Try to vote on a non-existent proposal
    let result = with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter.clone(),
            999,
            VoteType::Upvote,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_finalize_proposal_before_voting_ends() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Try to finalize before voting ends
    let result = with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_execute_non_passed_proposal() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Fast forward to end voting period
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal (should be rejected due to no votes)
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Fast forward past execution delay
    env.ledger().with_mut(|l| {
        l.timestamp = 7201 + 86400 + 1;
    });

    // Try to execute a rejected proposal
    let result = with_contract(&env, &contract_id, || {
        DaoContract::execute_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    });

    assert!(result.is_err());
}

#[test]
fn test_execute_already_executed_proposal() {
    let env = Env::default();
    let (contract_id, admin, _nft_contract, _reputation_contract) = setup_contract(&env);
    let proposer = create_address(&env, "proposer");
    let voter1 = create_address(&env, "voter1");
    let voter2 = create_address(&env, "voter2");

    // Create a proposal
    let proposal_id = with_contract(&env, &contract_id, || {
        DaoContract::create_proposal(
            env.clone(),
            proposer.clone(),
            String::from_str(&env, "Test Proposal"),
            String::from_str(&env, "This is a test proposal"),
            ProposalType::Funding,
            7200,
            50,
            60,
        )
    }).unwrap();

    // Cast votes to pass the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter1.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    with_contract(&env, &contract_id, || {
        DaoContract::cast_vote(
            env.clone(),
            voter2.clone(),
            proposal_id,
            VoteType::Upvote,
        )
    }).unwrap();

    // Fast forward to end voting period
    env.ledger().with_mut(|l| {
        l.timestamp = 7201;
    });

    // Finalize the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::finalize_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Fast forward past execution delay
    env.ledger().with_mut(|l| {
        l.timestamp = 7201 + 86400 + 1;
    });

    // Execute the proposal
    with_contract(&env, &contract_id, || {
        DaoContract::execute_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    }).unwrap();

    // Try to execute again
    let result = with_contract(&env, &contract_id, || {
        DaoContract::execute_proposal(
            env.clone(),
            admin.clone(),
            proposal_id,
        )
    });

    assert!(result.is_err());
}
