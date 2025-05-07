// src/clients.rs
use soroban_sdk::{Address, Env};

/// A client for the reputation contract.
pub struct ReputationClient<'a> {
    env: &'a Env,
    contract_id: &'a Address,
}

impl<'a> ReputationClient<'a> {
    pub fn new(env: &'a Env, contract_id: &'a Address) -> Self {
        ReputationClient { env, contract_id }
    }

    /// Calls the reputation contract to get the reputation score for a given voter.
    /// Assumes the reputation contract has a function named `get_reputation` that takes an Address and returns a u128.
    pub fn get_reputation(&self, _voter: &Address) -> u64 {
        // In a real implementation, you’d invoke the contract.
        // For example, if you had a generated client or using env.invoke_contract:
        // self.env.invoke_contract(self.contract_id, &Symbol::short("get_rep"), voter)
        //   .unwrap_or(0);
        // For illustration, we'll return a dummy value.
        100 // dummy reputation score
    }
}

/// A client for an ERC721 (NFT) contract.
pub struct ERC721Client<'a> {
    env: &'a Env,
    contract_id: &'a Address,
}

impl<'a> ERC721Client<'a> {
    pub fn new(env: &'a Env, contract_id: &'a Address) -> Self {
        ERC721Client { env, contract_id }
    }

    /// Calls the ERC721 contract to get the NFT balance (voting power) for a given owner.
    /// Assumes the ERC721 contract exposes a function named `balance_of` returning a u128.
    pub fn balance_of(&self, _owner: &Address) -> u64 {
        // Similar to the ReputationClient, you would call the contract.
        // For illustration, we'll return a dummy value.
        5 // dummy NFT balance (voting power)
    }
}

/// A helper function to calculate the overall voting power,
/// combining reputation and NFT-based power. The final value is a u128 (or you could use u256 if desired).
pub fn calculate_voting_power(
    env: &Env,
    nft_contract: &Address,
    reputation_contract: &Address,
    voter: &Address,
) -> u64 {
    let rep_client = ReputationClient::new(env, reputation_contract);
    let nft_client = ERC721Client::new(env, nft_contract);
    let reputation = rep_client.get_reputation(voter);
    let nft_balance = nft_client.balance_of(voter);

    // Combine them (e.g., simple addition, or apply weights as needed)
    reputation + nft_balance
}
