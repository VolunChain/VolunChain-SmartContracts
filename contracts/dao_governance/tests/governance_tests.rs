// Example test in src/test.rs
use crate::clients::{calculate_voting_power, ERC721Client, ReputationClient};
use soroban_sdk::{Address, Env};

#[test]
fn test_calculate_voting_power() {
    let env = Env::default();
    // Suppose these are the IDs of your dummy contracts
    let reputation_contract = Address::random(&env);
    let nft_contract = Address::random(&env);
    let voter = Address::random(&env);

    // In your test, you can override or simulate responses for these dummy contracts.
    // For example, you could use a mock mechanism if available,
    // or simply have your dummy contracts return hardcoded values (as shown in the client impls).

    let power = calculate_voting_power(&env, &nft_contract, &reputation_contract, &voter);
    // For our dummy implementation: 100 (reputation) + 5 (NFT balance) = 105
    assert_eq!(power, 105);
}
