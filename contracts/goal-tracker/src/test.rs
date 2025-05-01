#![cfg(test)]
use soroban_sdk::{
    testutils::{Address as _, Events},
    vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::{Error, GoalTrackerContract, GoalTrackerContractClient, GoalType};

#[test]
fn test_goal_creation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize the contract with proper auth
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);

    // Mock auth for initialize call
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    // Test creating a goal
    let volunteer = Address::generate(&env);

    // Mock auth for create_goal call
    env.mock_all_auths();

    // Create goal should succeed
    let goal_id = client.create_goal(&volunteer, &GoalType::TasksCompleted, &10);
    assert_eq!(goal_id, 1);

    // Verify goal data
    let goal = client.get_goal(&goal_id);
    assert_eq!(goal.volunteer, volunteer);
    assert_eq!(goal.goal_type, GoalType::TasksCompleted);
    assert_eq!(goal.target_amount, 10);
    assert_eq!(goal.current_amount, 0);
    assert!(!goal.is_completed);

    // Verify user goals list
    let user_goals = client.get_goals_by_user(&volunteer);
    assert_eq!(user_goals.len(), 1);
    assert_eq!(user_goals.get(0).unwrap(), 1);

    // Test creating goal with zero target (should fail)
    env.mock_all_auths();
    let result = client.try_create_goal(&volunteer, &GoalType::HoursVolunteered, &0);
    assert_eq!(result, Err(Ok(Error::TargetMustBePositive)));
}

#[test]
fn test_progress_updates() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize with proper auth
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    // Create goal
    let volunteer = Address::generate(&env);
    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::HoursVolunteered, &100);

    // Test valid progress update (updater only)
    env.mock_all_auths();
    let result = client.update_progress(&goal_id, &25);
    assert!(result);

    let goal = client.get_goal(&goal_id);
    assert_eq!(goal.current_amount, 25);
    assert!(!goal.is_completed);

    // Test invalid progress updates
    env.mock_all_auths();

    // Non-updater trying to update (should fail)
    let non_updater = Address::generate(&env);
    env.as_contract(&contract_id, || {
        let result = client.try_update_progress(&goal_id, &10);
        assert!(result.is_err(), "Expected panic from unauthorized updater");
    });

    // Zero amount (should fail)
    env.mock_all_auths();
    let result = client.try_update_progress(&goal_id, &0);
    assert_eq!(result, Err(Ok(Error::AmountToAddMustBePositive)));

    // Update to completion
    env.mock_all_auths();
    client.update_progress(&goal_id, &75);
    let goal = client.get_goal(&goal_id);
    assert!(goal.is_completed);

    // Try updating completed goal (should fail)
    env.mock_all_auths();
    let result = client.try_update_progress(&goal_id, &10);
    assert_eq!(result, Err(Ok(Error::GoalAlreadyCompleted)));
}

#[test]
fn test_multiple_goals_tracking() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize with proper auth
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    // Create multiple goals for same user
    let volunteer = Address::generate(&env);
    env.mock_all_auths();

    let goal1 = client.create_goal(&volunteer, &GoalType::TasksCompleted, &5);
    let goal2 = client.create_goal(&volunteer, &GoalType::CertificationsEarned, &3);
    let goal3 = client.create_goal(&volunteer, &GoalType::HoursVolunteered, &50);

    // Verify all goals exist
    let user_goals = client.get_goals_by_user(&volunteer);
    assert_eq!(user_goals.len(), 3);
    assert!(user_goals.contains(&goal1));
    assert!(user_goals.contains(&goal2));
    assert!(user_goals.contains(&goal3));

    // Update progress on different goals
    env.mock_all_auths();
    client.update_progress(&goal1, &2);
    client.update_progress(&goal2, &1);
    client.update_progress(&goal3, &25);

    // Verify progress
    assert_eq!(client.get_goal(&goal1).current_amount, 2);
    assert_eq!(client.get_goal(&goal2).current_amount, 1);
    assert_eq!(client.get_goal(&goal3).current_amount, 25);

    // Complete one goal
    env.mock_all_auths();
    client.update_progress(&goal2, &2); // Now current = 3 (target reached)

    let goal2_data = client.get_goal(&goal2);
    assert!(goal2_data.is_completed);
    assert_eq!(goal2_data.current_amount, 3);

    // Other goals should remain unchanged
    assert_eq!(client.get_goal(&goal1).current_amount, 2);
    assert_eq!(client.get_goal(&goal3).current_amount, 25);
}

#[test]
fn test_goal_completion_events() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize and create a goal (setup)
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    let volunteer = Address::generate(&env);
    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::CertificationsEarned, &1);

    // Clear previous events for this test
    env.events();

    // Complete the goal
    env.mock_all_auths();
    client.update_progress(&goal_id, &1);

    // Verify both the update and completion events
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            // Progress update event
            (
                contract_id.clone(),
                (Symbol::short("updated"), Symbol::short("goal_prog")).into_val(&env),
                (goal_id, 1u64, 1u64).into_val(&env)
            ),
            // Goal completion event
            (
                contract_id,
                (Symbol::short("completed"), Symbol::short("goal_done")).into_val(&env),
                (goal_id, volunteer, GoalType::CertificationsEarned).into_val(&env)
            ),
        ]
    );
}

#[test]
fn test_progress_update_events() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize and create a goal (setup)
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    let volunteer = Address::generate(&env);
    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::HoursVolunteered, &100);

    env.events();

    // Update progress
    env.mock_all_auths();
    client.update_progress(&goal_id, &25);

    // Verify the update event
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id,
                (Symbol::short("updated"), Symbol::short("goal_prog")).into_val(&env),
                (goal_id, 25u64, 25u64).into_val(&env)
            ),
        ]
    );
}
