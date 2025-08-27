use soroban_sdk::{contracterror, contracttype, Address, String};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
#[contracttype]
pub enum VoteType {
    Upvote = 1,
    Downvote = 2,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    Vote(u32, Address),
    Config,
    ProposalCount,
    ProposalVotes(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DaoError {
    Unauthorized = 1,
    ProposalNotFound = 2,
    ProposalNotActive = 3,
    AlreadyVoted = 4,
    VotingEnded = 5,
    VotingNotEnded = 6,
    ExecutionFailed = 7,
    ProposalAlreadyExecuted = 8,
    InsufficientVotingPower = 9,
    AlreadyInitialized = 10,
    ProposalNotPassed = 11,
    ExecutionDelayNotMet = 12,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
#[contracttype]
pub enum ProposalType {
    Funding = 1,
    Feature = 2,
    Policy = 3,
    Other = 4,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
#[contracttype]
pub enum ProposalStatus {
    Pending = 1,
    Active = 2,
    Passed = 3,
    Rejected = 4,
    Executed = 5,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub proposer: Address,
    pub start_time: u64,
    pub end_time: u64,
    pub status: ProposalStatus,
    pub upvotes: u64,
    pub downvotes: u64,
    pub minimum_quorum: u64,
    pub minimum_approval: u64,
    pub executed: bool,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Vote {
    pub voter: Address,
    pub proposal_id: u32,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct DaoConfig {
    pub admin: Address,
    pub nft_contract: Address,
    pub reputation_contract: Address,
    pub proposal_creation_threshold: u64,
    pub execution_delay: u64,
    pub min_voting_period: u64,
}
