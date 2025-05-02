#![cfg(test)]
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    vec, Address, Env, IntoVal, Symbol,
};

use crate::{ContractError, GoalTrackerContract, GoalTrackerContractClient, GoalType};

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
    assert_eq!(result, Err(Ok(ContractError::TargetMustBePositive)));
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
    assert_eq!(result, Err(Ok(ContractError::AmountToAddMustBePositive)));

    // Update to completion
    env.mock_all_auths();
    client.update_progress(&goal_id, &75);
    let goal = client.get_goal(&goal_id);
    assert!(goal.is_completed);

    // Try updating completed goal (should fail)
    env.mock_all_auths();
    let result = client.try_update_progress(&goal_id, &10);
    assert_eq!(result, Err(Ok(ContractError::GoalAlreadyCompleted)));
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

#[test]
fn test_edge_cases() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize with proper auth
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    // Test fetching non-existent goal
    let result = client.try_get_goal(&999);
    assert_eq!(result, Err(Ok(ContractError::GoalNotFound)));

    // Test getting goals for user with none
    let user_with_no_goals = Address::generate(&env);
    let goals = client.get_goals_by_user(&user_with_no_goals);
    assert_eq!(goals.len(), 0);

    // Test reinitialization (should fail)
    env.mock_all_auths();
    let result = client.try_initialize(&admin, &updater);
    assert_eq!(result, Err(Ok(ContractError::AlreadyInitialized)));

    // Test unauthorized set_updater
    let non_admin = Address::generate(&env);
    env.as_contract(&contract_id, || {
        let result = client.try_set_updater(&non_admin);
        assert!(result.is_err(), "Expected panic from unauthorized admin");
    });

    // Test unauthorized update_progress

    env.as_contract(&contract_id, || {
        let result = client.try_update_progress(&1, &10);
        assert!(result.is_err(), "Expected panic from unauthorized updater");
    });
}

#[test]
fn test_reinitialization_attempt() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // First initialization
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    let new_admin = Address::generate(&env);
    let new_updater = Address::generate(&env);
    env.mock_all_auths();
    let result = client.try_initialize(&new_admin, &new_updater);

    assert_eq!(result, Err(Ok(ContractError::AlreadyInitialized)));

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_updater(), updater);
}
#[test]
fn test_unauthorized_access() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    // Initialize
    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    // Create goal
    let volunteer = Address::generate(&env);
    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::HoursVolunteered, &100);

    let attacker = Address::generate(&env);

    //  Unauthorized set_updater attempt
    env.as_contract(&contract_id, || {
        let result = client.try_set_updater(&attacker);
        assert!(result.is_err(), "Should_prevent_non-admin");
    });

    // Unauthorized set_admin attempt
    env.as_contract(&contract_id, || {
        let result = client.try_set_admin(&attacker);
        assert!(
            result.is_err(),
            "Should_prevent_non-admin_from_setting_admin"
        );
    });

    //  Unauthorized update_progress attempt
    env.as_contract(&contract_id, || {
        let result = client.try_update_progress(&goal_id, &10);
        assert!(
            result.is_err(),
            "Should_prevent_non-updater_from_updating_progress"
        );
    });

    // Unauthorized remove_updater attempt
    env.as_contract(&contract_id, || {
        let result = client.try_remove_updater();
        assert!(
            result.is_err(),
            "Should prevent non-admin from removing updater"
        );
    });

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_updater(), updater);
    assert_eq!(client.get_goal(&goal_id).current_amount, 0);
}

#[test]
fn test_authorization_requirements() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    let result = client.try_initialize(&admin, &updater);
    assert!(result.is_err(), "Init_without_auth_should_fail");

    env.mock_all_auths();
    client.initialize(&admin, &updater);

    let volunteer = Address::generate(&env);

    let result = client
        .try_create_goal(&volunteer, &GoalType::TasksCompleted, &5)
        .unwrap();
    assert_eq!(result, Ok(1), "Not_Create_goal");

    env.as_contract(&contract_id, || {
        let result = client.try_create_goal(&volunteer, &GoalType::TasksCompleted, &5);
        assert!(result.is_err(), "Create_goal_with_wrong_auth_should_fail");
    });

    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::TasksCompleted, &5);

    env.as_contract(&contract_id, || {
        let result = client.try_update_progress(&goal_id, &1);
        assert!(result.is_err(), "Update_with_non_updater_auth_should_fail");
    });
}

#[test]
fn test_get_functions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GoalTrackerContract);
    let client = GoalTrackerContractClient::new(&env, &contract_id);

    assert_eq!(
        client.try_get_admin(),
        Err(Ok(ContractError::NotInitialized)),
        "get_admin_should_fail_when_not_initialized"
    );
    assert_eq!(
        client.try_get_updater(),
        Err(Ok(ContractError::NotInitialized)),
        "get_updater_should_fail_when_not_initialized"
    );
    assert_eq!(
        client.try_get_next_id(),
        Err(Ok(ContractError::NotInitialized)),
        "get_next_id_should_fail_when_not_initialized"
    );

    let admin = Address::generate(&env);
    let updater = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &updater);

    assert_eq!(
        client.get_admin(),
        admin,
        "get_admin_should_return_correct_value"
    );
    assert_eq!(
        client.get_updater(),
        updater,
        "get_updater_should_return_correct_value"
    );
    assert_eq!(
        client.get_next_id(),
        1,
        "get_next_id_should_return_initial_value"
    );

    let volunteer = Address::generate(&env);
    env.mock_all_auths();
    let goal_id = client.create_goal(&volunteer, &GoalType::TasksCompleted, &10);

    let goal = client.get_goal(&goal_id);
    assert_eq!(goal.id, goal_id, "get_goal_should_return_correct_goal");
    assert_eq!(
        goal.volunteer, volunteer,
        "get_goal_should_have_correct_volunteer"
    );
    assert_eq!(
        client.try_get_goal(&999),
        Err(Ok(ContractError::GoalNotFound)),
        "get_goal_should_fail_for_invalid_id"
    );

    let user_goals = client.get_goals_by_user(&volunteer);
    assert_eq!(
        user_goals.len(),
        1,
        "get_goals_by_user_should_return_correct_count"
    );
    assert_eq!(
        user_goals.get(0).unwrap(),
        goal_id,
        "get_goals_by_user_should_include_goal_id"
    );

    let new_user = Address::generate(&env);
    let empty_goals = client.get_goals_by_user(&new_user);
    assert!(
        empty_goals.is_empty(),
        "get_goals_by_user_should_return_empty_vec_for_new_user"
    );
}
