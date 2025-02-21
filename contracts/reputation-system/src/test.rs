#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    symbol_short,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(ReputationSystem {}, ());
    let client = ReputationSystemClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Authorize admin's call
    env.mock_all_auths();

    client.initialize(&admin);
    
    // Verify that require_auth was called with admin
    let auths = env.auths();
    let auth = auths.first().unwrap();
    assert_eq!(auth.0, admin); // Verify that admin was authenticated
    assert_eq!(auths.len(), 1); // Expect 1 authentication call

    // Verify storage within the contract context
    env.as_contract(&contract_id, || {
        let orgs: Vec<Address> = env.storage().instance().get(&DataKey::Organizations).unwrap();
        assert_eq!(orgs.len(), 0);
    });
}

#[test]
fn test_add_organization_unauthorized() {
    let env = Env::default();
    let contract_id = env.register(ReputationSystem {}, ());
    let client = ReputationSystemClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let org = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(&admin);
    
    // Attempting to add an organization without being admin should fail
    client.add_organization(&not_admin, &org);
}

#[test]
fn test_multiple_endorsements() {
    let env = Env::default();
    let contract_id = env.register(ReputationSystem {}, ());
    let client = ReputationSystemClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let org1 = Address::generate(&env);
    let org2 = Address::generate(&env);
    let volunteer = Address::generate(&env);
    
    env.mock_all_auths();

    // Initialize and add organizations
    client.initialize(&admin);
    client.add_organization(&admin, &org1);
    client.add_organization(&admin, &org2);
    
    // Perform multiple endorsements
    client.endorse_volunteer(&org1, &volunteer, &50, &symbol_short!("CODE"));
    client.endorse_volunteer(&org2, &volunteer, &30, &symbol_short!("TEST"));
    
    // Reputation should be the sum
    let reputation = client.get_reputation(&volunteer);
    assert_eq!(reputation, 80);
}

#[test]
fn test_endorsement_flow() {
    let env = Env::default();
    let contract_id = env.register(ReputationSystem {}, ());
    let client = ReputationSystemClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let org = Address::generate(&env);
    let volunteer = Address::generate(&env);
    
    // Authorize all calls
    env.mock_all_auths();

    // Initialize and add organization
    client.initialize(&admin);
    client.add_organization(&admin, &org);
    
    // Verify organization was added
    env.as_contract(&contract_id, || {
        let orgs: Vec<Address> = env.storage().instance().get(&DataKey::Organizations).unwrap();
        assert!(orgs.contains(&org));
    });
    
    // Perform endorsement
    client.endorse_volunteer(
        &org,
        &volunteer,
        &100,
        &symbol_short!("CODE"),
    );
    
    // Verify reputation
    let reputation = client.get_reputation(&volunteer);
    assert_eq!(reputation, 100);
}

#[test]
#[should_panic(expected = "Unauthorized organization")]
fn test_unauthorized_endorsement() {
    let env = Env::default();
    let contract_id = env.register(ReputationSystem {}, ());
    let client = ReputationSystemClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let unauthorized_org = Address::generate(&env);
    let volunteer = Address::generate(&env);
    
    // Authorize all calls
    env.mock_all_auths();

    client.initialize(&admin);
    
    // This call should fail because the organization is not authorized
    client.endorse_volunteer(
        &unauthorized_org,
        &volunteer,
        &100,
        &symbol_short!("CODE"),
    );
}
