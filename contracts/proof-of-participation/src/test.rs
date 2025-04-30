#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, String as SdkString,
};

fn create_env() -> Env {
    Env::default()
}

fn register_contract(env: &Env) -> Address {
    env.register_contract(None, ProofOfParticipationContract)
}

fn setup_ledger_time(env: &Env, timestamp: u64) {
    env.ledger().set(LedgerInfo {
        timestamp,
        protocol_version: 22,
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 1_000_000,
        min_persistent_entry_ttl: 1_000_000,
        max_entry_ttl: 6_312_000,
    });
}

fn create_client<'a>(env: &'a Env, contract_id: &'a Address) -> ProofOfParticipationContractClient<'a> {
    ProofOfParticipationContractClient::new(env, contract_id)
}

#[test]
fn test_initialize_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Verify admin is set correctly
    env.as_contract(&contract_id, || {
        let stored_admin = storage::get_admin(&env);
        assert_eq!(stored_admin, admin);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_initialize_already_initialized() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Initialize the contract once
    client.initialize(&admin);
    
    // Try to initialize again - should panic with AlreadyInitialized (code 1)
    client.initialize(&admin);
}

#[test]
fn test_register_organization_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = SdkString::from_str(&env, "Test Organization");
    
    // Initialize first
    client.initialize(&admin);
    
    // Register organization
    client.register_organization(&admin, &organization, &name);
    
    // Verify organization is registered
    let is_org = client.is_organization(&organization);
    assert!(is_org);
    
    // Verify we can access organization info from storage
    env.as_contract(&contract_id, || {
        assert!(storage::is_organization_registered(&env, &organization));
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_register_organization_not_admin() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = SdkString::from_str(&env, "Test Organization");
    
    // Initialize first
    client.initialize(&admin);
    
    // Try to register organization with non-admin account - should panic with NotAuthorized (code 2)
    client.register_organization(&not_admin, &organization, &name);
}

#[test]
#[should_panic(expected = "Error(Contract, #101)")]
fn test_register_organization_already_registered() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = SdkString::from_str(&env, "Test Organization");
    
    // Initialize first
    client.initialize(&admin);
    
    // Register organization
    client.register_organization(&admin, &organization, &name);
    
    // Try to register same organization again - should panic with OrganizationAlreadyRegistered (code 101)
    client.register_organization(&admin, &organization, &name);
}

#[test]
fn test_remove_organization_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = SdkString::from_str(&env, "Test Organization");
    
    // Initialize first
    client.initialize(&admin);
    
    // Register organization
    client.register_organization(&admin, &organization, &name);
    
    // Remove organization
    client.remove_organization(&admin, &organization);
    
    // Verify organization is not registered anymore
    let is_org = client.is_organization(&organization);
    assert!(!is_org);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_remove_organization_not_admin() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = SdkString::from_str(&env, "Test Organization");
    
    // Initialize first
    client.initialize(&admin);
    
    // Register organization
    client.register_organization(&admin, &organization, &name);
    
    // Try to remove organization with non-admin account - should panic with NotAuthorized (code 2)
    client.remove_organization(&not_admin, &organization);
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_remove_organization_not_registered() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    
    // Initialize first
    client.initialize(&admin);
    
    // Try to remove non-existent organization - should panic with OrganizationNotRegistered (code 102)
    client.remove_organization(&admin, &organization);
}

#[test]
fn test_register_participation_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = SdkString::from_str(&env, "Test Organization");
    let task_id = SdkString::from_str(&env, "task-123");
    let task_name = SdkString::from_str(&env, "Clean the park");
    
    // Setup: Initialize and register organization
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    
    // Set a specific timestamp for testing
    let timestamp = 1645000000u64;
    setup_ledger_time(&env, timestamp);
    
    // Register participation
    client.register_participation(&organization, &volunteer, &task_id, &task_name);
    
    // Verify participation is registered
    let has_participation = client.verify_participation(&volunteer, &task_id);
    assert!(has_participation);
    
    // Verify timestamp is correct
    let stored_timestamp = client.get_participation_timestamp(&volunteer, &task_id);
    assert_eq!(stored_timestamp, timestamp);
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_register_participation_not_organization() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let not_organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let task_id = SdkString::from_str(&env, "task-123");
    let task_name = SdkString::from_str(&env, "Clean the park");
    
    // Setup: Initialize but don't register the organization
    client.initialize(&admin);
    
    // Try to register participation with non-registered organization - should panic with OrganizationNotRegistered (code 102)
    client.register_participation(&not_organization, &volunteer, &task_id, &task_name);
}

#[test]
#[should_panic(expected = "Error(Contract, #201)")]
fn test_register_participation_already_registered() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = SdkString::from_str(&env, "Test Organization");
    let task_id = SdkString::from_str(&env, "task-123");
    let task_name = SdkString::from_str(&env, "Clean the park");
    
    // Setup: Initialize and register organization
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    
    // Register participation first time
    client.register_participation(&organization, &volunteer, &task_id, &task_name);
    
    // Try to register same participation again - should panic with ParticipationAlreadyRegistered (code 201)
    client.register_participation(&organization, &volunteer, &task_id, &task_name);
}

#[test]
fn test_get_volunteer_participations() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = SdkString::from_str(&env, "Test Organization");
    let task_id1 = SdkString::from_str(&env, "task-123");
    let task_name1 = SdkString::from_str(&env, "Clean the park");
    let task_id2 = SdkString::from_str(&env, "task-456");
    let task_name2 = SdkString::from_str(&env, "Plant trees");
    
    // Setup: Initialize and register organization
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    
    // Register two participations
    let timestamp1 = 1645000000u64;
    setup_ledger_time(&env, timestamp1);
    client.register_participation(&organization, &volunteer, &task_id1, &task_name1);
    
    let timestamp2 = 1646000000u64;
    setup_ledger_time(&env, timestamp2);
    client.register_participation(&organization, &volunteer, &task_id2, &task_name2);
    
    // Get volunteer participations
    let participations = client.get_volunteer_participations(&volunteer);
    
    // Verify results
    assert_eq!(participations.len(), 2);
    
    // We can't guarantee order, so check both entries exist
    let has_task1 = participations.iter().any(|p| {
        p.task_id == task_id1 && 
        p.task_name == task_name1 && 
        p.volunteer == volunteer &&
        p.timestamp == timestamp1
    });
    
    let has_task2 = participations.iter().any(|p| {
        p.task_id == task_id2 && 
        p.task_name == task_name2 && 
        p.volunteer == volunteer &&
        p.timestamp == timestamp2
    });
    
    assert!(has_task1, "Missing first task participation");
    assert!(has_task2, "Missing second task participation");
}

#[test]
fn test_get_task_volunteers() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer1 = Address::generate(&env);
    let volunteer2 = Address::generate(&env);
    let org_name = SdkString::from_str(&env, "Test Organization");
    let task_id = SdkString::from_str(&env, "task-123");
    let task_name = SdkString::from_str(&env, "Clean the park");
    
    // Setup: Initialize and register organization
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    
    // Register participations for two different volunteers
    setup_ledger_time(&env, 1645000000u64);
    client.register_participation(&organization, &volunteer1, &task_id, &task_name);
    
    setup_ledger_time(&env, 1646000000u64);
    client.register_participation(&organization, &volunteer2, &task_id, &task_name);
    
    // Get task volunteers
    let volunteers = client.get_task_volunteers(&task_id);
    
    // Verify results
    assert_eq!(volunteers.len(), 2);
    
    // Check both volunteers are included
    let has_volunteer1 = volunteers.iter().any(|v| v == volunteer1);
    let has_volunteer2 = volunteers.iter().any(|v| v == volunteer2);
    
    assert!(has_volunteer1, "Missing first volunteer");
    assert!(has_volunteer2, "Missing second volunteer");
}

#[test]
#[should_panic(expected = "Error(Contract, #202)")]
fn test_participation_timestamp_not_found() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let task_id = SdkString::from_str(&env, "task-123");
    
    // Setup: Initialize
    client.initialize(&admin);
    
    // Try to get timestamp for non-existent participation - should panic with ParticipationNotFound (code 202)
    client.get_participation_timestamp(&volunteer, &task_id);
}

#[test]
fn test_get_empty_volunteer_participations() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let volunteer = Address::generate(&env);
    
    // Setup: Initialize
    client.initialize(&admin);
    
    // Get participations for volunteer with no participations
    let participations = client.get_volunteer_participations(&volunteer);
    
    // Verify empty result
    assert_eq!(participations.len(), 0);
}

#[test]
fn test_get_empty_task_volunteers() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let task_id = SdkString::from_str(&env, "task-123");
    
    // Setup: Initialize
    client.initialize(&admin);
    
    // Get volunteers for task with no participations
    let volunteers = client.get_task_volunteers(&task_id);
    
    // Verify empty result
    assert_eq!(volunteers.len(), 0);
}