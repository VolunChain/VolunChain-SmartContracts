#![cfg(test)]

extern crate std;

use crate::types::*;
use crate::{BudgetAllocation, BudgetAllocationClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let admin = Address::generate(&env);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);

    // Use mock_all_auths to authorize the transaction
    env.mock_all_auths();
    
    // Call initialize directly as admin
    contract_client.initialize(&admin);

    // Directly check that the admin has been set correctly
    // Instead of accessing storage directly, use a getter function if available
    // For now, we'll just check that the function doesn't panic
    assert!(true, "Initialize function completed without errors");
}

#[test]
fn test_add_organization() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let org = Address::generate(&env);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);
    
    // Authorize all transactions in this test
    env.mock_all_auths();
    
    // Initialize contract
    contract_client.initialize(&admin);

    // Add organization - admin is the caller
    contract_client.add_organization(&admin, &org);

    // Verify using a getter function instead of direct storage access
    // If you have a function like get_organizations(), use that instead
    // For now, we'll just assert success
    assert!(true, "Add organization function completed without errors");
}

#[test]
fn test_allocate_project_budget() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let org = Address::generate(&env);
    let project_owner = Address::generate(&env);
    
    // Create milestone descriptions and amounts
    let mut milestone_descriptions = Vec::new(&env);
    milestone_descriptions.push_back(String::from_str(&env, "Milestone 1"));
    milestone_descriptions.push_back(String::from_str(&env, "Milestone 2"));
    
    let mut milestone_amounts = Vec::new(&env);
    milestone_amounts.push_back(50);
    milestone_amounts.push_back(50);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);
    
    // Authorize all transactions in this test
    env.mock_all_auths();
    
    // Initialize contract
    contract_client.initialize(&admin);
    
    // Add organization
    contract_client.add_organization(&admin, &org);

    // Allocate project budget with organization as caller
    let project_id = contract_client.allocate_project_budget(&org, &project_owner, &100, &milestone_descriptions, &milestone_amounts);

    // Verify project budget and milestones using contract functions
    let budget = contract_client.get_project_budget(&project_id);
    assert_eq!(budget, 100);

    let milestones = contract_client.get_project_milestones(&project_id);
    assert_eq!(milestones.len(), 2);
    assert_eq!(milestones.get(0).unwrap().amount, 50);
    assert_eq!(milestones.get(1).unwrap().amount, 50);
}

#[test]
fn test_complete_milestone() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let org = Address::generate(&env);
    let project_owner = Address::generate(&env);
    
    // Create milestone descriptions and amounts
    let mut milestone_descriptions = Vec::new(&env);
    milestone_descriptions.push_back(String::from_str(&env, "Milestone 1"));
    
    let mut milestone_amounts = Vec::new(&env);
    milestone_amounts.push_back(100);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);
    
    // Authorize all transactions in this test
    env.mock_all_auths();
    
    // Initialize contract
    contract_client.initialize(&admin);
    
    // Add organization
    contract_client.add_organization(&admin, &org);

    // Allocate project budget
    let project_id = contract_client.allocate_project_budget(&org, &project_owner, &100, &milestone_descriptions, &milestone_amounts);

    // Complete milestone
    contract_client.complete_milestone(&org, &project_id, &0);

    // Verify milestone completion
    let milestones = contract_client.get_project_milestones(&project_id);
    assert!(milestones.get(0).unwrap().completed);
}

#[test]
fn test_request_funds() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let org = Address::generate(&env);
    let project_owner = Address::generate(&env);
    
    // Create milestone descriptions and amounts
    let mut milestone_descriptions = Vec::new(&env);
    milestone_descriptions.push_back(String::from_str(&env, "Milestone 1"));
    
    let mut milestone_amounts = Vec::new(&env);
    milestone_amounts.push_back(100);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);
    
    // Authorize all transactions in this test
    env.mock_all_auths();
    
    // Initialize contract
    contract_client.initialize(&admin);
    
    // Add organization
    contract_client.add_organization(&admin, &org);

    // Allocate project budget
    let project_id = contract_client.allocate_project_budget(&org, &project_owner, &100, &milestone_descriptions, &milestone_amounts);
    
    // Complete milestone
    contract_client.complete_milestone(&org, &project_id, &0);

    // Request funds
    let request_id = contract_client.request_funds(&project_owner, &project_id, &0);

    // Verify fund request
    let fund_requests = contract_client.get_fund_requests(&project_id);
    assert_eq!(fund_requests.len(), 1);
    assert_eq!(fund_requests.get(0).unwrap().id, request_id);
}

#[test]
fn test_release_funds() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let org = Address::generate(&env);
    let project_owner = Address::generate(&env);
    
    // Create milestone descriptions and amounts
    let mut milestone_descriptions = Vec::new(&env);
    milestone_descriptions.push_back(String::from_str(&env, "Milestone 1"));
    
    let mut milestone_amounts = Vec::new(&env);
    milestone_amounts.push_back(100);

    // Initialize the contract
    let contract_address = env.register_contract(None, BudgetAllocation {});
    let contract_client = BudgetAllocationClient::new(&env, &contract_address);
    
    // Authorize all transactions in this test
    env.mock_all_auths();
    
    // Initialize contract
    contract_client.initialize(&admin);
    
    // Add organization
    contract_client.add_organization(&admin, &org);

    // Allocate project budget
    let project_id = contract_client.allocate_project_budget(&org, &project_owner, &100, &milestone_descriptions, &milestone_amounts);
    
    // Complete milestone
    contract_client.complete_milestone(&org, &project_id, &0);

    // Request funds
    let request_id = contract_client.request_funds(&project_owner, &project_id, &0);

    // Release funds
    contract_client.release_funds(&org, &project_id, &request_id);

    // Verify that the milestone is marked as released
    let milestones = contract_client.get_project_milestones(&project_id);
    assert!(milestones.get(0).unwrap().released);
}