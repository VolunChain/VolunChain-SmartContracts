#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, Vec,
};

use crate::{AvailabilityScheduler, AvailabilitySchedulerClient};

#[test]
fn test_set_and_get_availability() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let time_slots = Vec::from_array(&env, [
        (9, 12),  // 9:00 - 12:00
        (14, 17), // 14:00 - 17:00
    ]);

    env.mock_all_auths();
    client.set_availability(&volunteer, &0, &time_slots);
    let retrieved_slots = client.get_availability(&volunteer, &0);

    assert_eq!(retrieved_slots.len(), 2);
    assert_eq!(retrieved_slots.get(0), Some((9, 12)));
    assert_eq!(retrieved_slots.get(1), Some((14, 17)));
}

#[test]
fn test_overlapping_time_slots() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let time_slots = Vec::from_array(&env, [
        (9, 12),  // 9:00 - 12:00
        (11, 14), // 11:00 - 14:00 (overlaps with previous)
    ]);

    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &0, &time_slots);
    assert!(result.is_err());
}

#[test]
fn test_invalid_time_range() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let time_slots = Vec::from_array(&env, [
        (12, 9),  // Invalid: end time before start time
    ]);

    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &0, &time_slots);
    assert!(result.is_err());
}

#[test]
fn test_invalid_day() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let time_slots = Vec::from_array(&env, [
        (9, 12),
    ]);

    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &7, &time_slots);
    assert!(result.is_err());
}

#[test]
fn test_update_availability() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    
    // Set initial availability
    let initial_slots = Vec::from_array(&env, [
        (9, 12),
        (14, 17),
    ]);
    
    env.mock_all_auths();
    client.set_availability(&volunteer, &0, &initial_slots);
    
    // Update availability
    let updated_slots = Vec::from_array(&env, [
        (10, 13),
        (15, 18),
    ]);
    
    client.set_availability(&volunteer, &0, &updated_slots);
    
    // Verify updated slots
    let retrieved_slots = client.get_availability(&volunteer, &0);
    assert_eq!(retrieved_slots.len(), 2);
    assert_eq!(retrieved_slots.get(0), Some((10, 13)));
    assert_eq!(retrieved_slots.get(1), Some((15, 18)));
}

#[test]
fn test_multiple_days_availability() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    
    // Set availability for Monday (0)
    let monday_slots = Vec::from_array(&env, [
        (9, 12),
        (14, 17),
    ]);
    
    // Set availability for Wednesday (2)
    let wednesday_slots = Vec::from_array(&env, [
        (10, 13),
        (15, 18),
    ]);
    
    env.mock_all_auths();
    client.set_availability(&volunteer, &0, &monday_slots);
    client.set_availability(&volunteer, &2, &wednesday_slots);
    
    // Verify Monday slots
    let monday_retrieved = client.get_availability(&volunteer, &0);
    assert_eq!(monday_retrieved.len(), 2);
    assert_eq!(monday_retrieved.get(0), Some((9, 12)));
    assert_eq!(monday_retrieved.get(1), Some((14, 17)));
    
    // Verify Wednesday slots
    let wednesday_retrieved = client.get_availability(&volunteer, &2);
    assert_eq!(wednesday_retrieved.len(), 2);
    assert_eq!(wednesday_retrieved.get(0), Some((10, 13)));
    assert_eq!(wednesday_retrieved.get(1), Some((15, 18)));
}

#[test]
fn test_empty_time_slots() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let empty_slots = Vec::new(&env);
    
    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &0, &empty_slots);
    assert!(result.is_err());
}

#[test]
fn test_invalid_hour_range() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let invalid_slots = Vec::from_array(&env, [
        (25, 26),  // Invalid: hour > 24
    ]);
    
    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &0, &invalid_slots);
    assert!(result.is_err());
}

#[test]
fn test_zero_hours() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer = Address::generate(&env);
    let invalid_slots = Vec::from_array(&env, [
        (0, 5),  // Testing zero hour
    ]);
    
    env.mock_all_auths();
    let result = client.try_set_availability(&volunteer, &0, &invalid_slots);
    assert!(result.is_err());
}

#[test]
fn test_multiple_volunteers() {
    let env = Env::default();
    let contract_id = env.register(AvailabilityScheduler, ());
    let client = AvailabilitySchedulerClient::new(&env, &contract_id);

    let volunteer1 = Address::generate(&env);
    let volunteer2 = Address::generate(&env);
    
    let slots1 = Vec::from_array(&env, [
        (9, 12),
        (14, 17),
    ]);
    
    let slots2 = Vec::from_array(&env, [
        (10, 13),
        (15, 18),
    ]);
    
    env.mock_all_auths();
    client.set_availability(&volunteer1, &0, &slots1);
    client.set_availability(&volunteer2, &0, &slots2);
    
    // Verify volunteer1's slots
    let retrieved1 = client.get_availability(&volunteer1, &0);
    assert_eq!(retrieved1.len(), 2);
    assert_eq!(retrieved1.get(0), Some((9, 12)));
    assert_eq!(retrieved1.get(1), Some((14, 17)));
    
    // Verify volunteer2's slots
    let retrieved2 = client.get_availability(&volunteer2, &0);
    assert_eq!(retrieved2.len(), 2);
    assert_eq!(retrieved2.get(0), Some((10, 13)));
    assert_eq!(retrieved2.get(1), Some((15, 18)));
} 