#![cfg(test)]
use super::*;

use crate::storage as core_storage;
use crate::organization_storage;
use crate::participation_storage::{self, Participation};

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo, Logs, storage::Persistent, storage::Instance},
    Address, Env, String as SdkString, Vec,
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

// Helper to create String for tests
fn str_to_sdkstring(env: &Env, s: &str) -> SdkString {
    SdkString::from_str(env, s)
}

// --- Test Cases ---

#[test]
fn test_initialize_success() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin);

    env.as_contract(&contract_id, || {
        let stored_admin = core_storage::get_admin(&env);
        assert_eq!(stored_admin, admin);
        assert!(env.storage().instance().get_ttl() > 0);
    });
    env.logs().print();
}


#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_initialize_already_initialized() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin);
    client.initialize(&admin); // Should panic
}

#[test]
fn test_register_organization_success() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = str_to_sdkstring(&env, "Test_Org");
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &name);
    assert!(client.is_organization(&organization));

    env.as_contract(&contract_id, || {
        assert!(organization_storage::is_organization_registered(&env, &organization));
        assert_eq!(organization_storage::get_organization_name(&env, &organization), Some(name.clone()));
        let org_key = crate::storage::DataKey::Organization(organization.clone());
        assert!(env.storage().persistent().has(&org_key));
        assert!(env.storage().persistent().get_ttl(&org_key) > 0);
        let list_key = crate::storage::DataKey::OrganizationList;
         assert!(env.storage().persistent().has(&list_key));
        assert!(env.storage().persistent().get_ttl(&list_key) > 0);
    });
    env.logs().print();
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_register_organization_not_admin() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = str_to_sdkstring(&env, "Test_Org");

    env.mock_all_auths(); // Mocking all auths for simplicity here
    client.initialize(&admin);
    client.register_organization(&not_admin, &organization, &name); // Should panic
}


#[test]
#[should_panic(expected = "Error(Contract, #101)")]
fn test_register_organization_already_registered() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = str_to_sdkstring(&env, "Test_Org");
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &name);
    client.register_organization(&admin, &organization, &name); // Should panic
}


#[test]
fn test_remove_organization_success() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = str_to_sdkstring(&env, "Test_Org");
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &name);
    assert!(client.is_organization(&organization));
    client.remove_organization(&admin, &organization);
    assert!(!client.is_organization(&organization));

    env.as_contract(&contract_id, || {
         assert!(!organization_storage::is_organization_registered(&env, &organization));
         let orgs = organization_storage::get_all_organizations(&env);
         assert!(!orgs.contains(&organization));
    });
    env.logs().print();
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_remove_organization_not_admin() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let name = str_to_sdkstring(&env, "Test_Org");
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &name);
    client.remove_organization(&not_admin, &organization); // Should panic
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_remove_organization_not_registered() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin);
    client.remove_organization(&admin, &organization); // Should panic
}


#[test]
fn test_register_participation_success() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Test_Org");
    let task_id = str_to_sdkstring(&env, "task-abc");
    let task_name = str_to_sdkstring(&env, "Plant_a_tree");
    let metadata: Option<SdkString> = Some(str_to_sdkstring(&env, "{\"location\":\"park\"}"));
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    let timestamp = 1700000000u64;
    setup_ledger_time(&env, timestamp);
    client.register_participation(&organization, &volunteer, &task_id, &task_name, &metadata);

    assert!(client.verify_participation(&volunteer, &task_id));
    let details = client.get_participation_details(&volunteer, &task_id);
    assert_eq!(details.volunteer, volunteer);
    assert_eq!(details.task_id, task_id);
    assert_eq!(details.task_name, task_name);
    assert_eq!(details.timestamp, timestamp);
    assert_eq!(details.organization, organization);
    assert_eq!(details.metadata, metadata);

    env.as_contract(&contract_id, || {
        let p_key = crate::storage::ParticipationKey { volunteer: volunteer.clone(), task_id: task_id.clone() };
        assert!(participation_storage::has_participation(&env, &volunteer, &task_id));
        let record_key = crate::storage::DataKey::ParticipationRecord(p_key.clone());
         assert!(env.storage().persistent().has(&record_key)); // Check record exists
        assert!(env.storage().persistent().get_ttl(&record_key) > 0);

        let vol_list_key = crate::storage::DataKey::VolunteerParticipations(volunteer.clone());
         assert!(env.storage().persistent().has(&vol_list_key)); // Check list exists
        let vol_keys: Vec<crate::storage::ParticipationKey> = env.storage().persistent().get(&vol_list_key).unwrap();
        assert!(vol_keys.contains(&p_key));
        assert!(env.storage().persistent().get_ttl(&vol_list_key) > 0);

        let task_list_key = crate::storage::DataKey::TaskVolunteers(task_id.clone());
        assert!(env.storage().persistent().has(&task_list_key)); // Check list exists
        let task_vols: Vec<Address> = env.storage().persistent().get(&task_list_key).unwrap();
        assert!(task_vols.contains(&volunteer));
        assert!(env.storage().persistent().get_ttl(&task_list_key) > 0);

        let org_list_key = crate::storage::DataKey::OrgParticipationList(organization.clone());
        assert!(env.storage().persistent().has(&org_list_key)); // Check list exists
        let org_keys: Vec<crate::storage::ParticipationKey> = env.storage().persistent().get(&org_list_key).unwrap();
        assert!(org_keys.contains(&p_key));
        assert!(env.storage().persistent().get_ttl(&org_list_key) > 0);
    });
    env.logs().print();
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_register_participation_org_not_registered() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let invalid_org = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let task_id = str_to_sdkstring(&env, "task-xyz");
    let task_name = str_to_sdkstring(&env, "Task_Name");
    let metadata: Option<SdkString> = None;
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_participation(&invalid_org, &volunteer, &task_id, &task_name, &metadata); // Should panic
}

#[test]
#[should_panic(expected = "Error(Contract, #201)")]
fn test_register_participation_already_registered() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Test_Org");
    let task_id = str_to_sdkstring(&env, "task-abc");
    let task_name = str_to_sdkstring(&env, "Plant_a_tree");
    let metadata: Option<SdkString> = None;
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    client.register_participation(&organization, &volunteer, &task_id, &task_name, &metadata);
    client.register_participation(&organization, &volunteer, &task_id, &task_name, &metadata); // Should panic
}

#[test]
#[should_panic(expected = "Error(Contract, #10)")]
fn test_register_participation_task_name_too_long() {
     let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Test_Org");
    let task_id = str_to_sdkstring(&env, "task-len");
     let long_task_name = str_to_sdkstring(&env, "a".repeat(participation_storage::MAX_TASK_NAME_LEN as usize + 1).as_str());
     let metadata: Option<SdkString> = None;
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    client.register_participation(&organization, &volunteer, &task_id, &long_task_name, &metadata); // Should panic
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn test_register_participation_metadata_too_long() {
     let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Test_Org");
    let task_id = str_to_sdkstring(&env, "task-meta");
    let task_name = str_to_sdkstring(&env, "Valid_Name");
    let long_metadata = Some(str_to_sdkstring(&env, "m".repeat(participation_storage::MAX_METADATA_LEN as usize + 1).as_str()));
    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);
    client.register_participation(&organization, &volunteer, &task_id, &task_name, &long_metadata); // Should panic
}


#[test]
fn test_verify_participation_not_found() {
     let env = create_env();
     let contract_id = register_contract(&env);
     let client = create_client(&env, &contract_id);
     let volunteer = Address::generate(&env);
     let task_id = str_to_sdkstring(&env, "non-existent-task");
     let exists = client.verify_participation(&volunteer, &task_id);
     assert!(!exists);
}

#[test]
#[should_panic(expected = "Error(Contract, #202)")]
fn test_get_participation_details_not_found() {
     let env = create_env();
     let contract_id = register_contract(&env);
     let client = create_client(&env, &contract_id);
     let volunteer = Address::generate(&env);
     let task_id = str_to_sdkstring(&env, "non-existent-task");
     client.get_participation_details(&volunteer, &task_id);
}

// --- Pagination Tests ---

#[test]
#[should_panic(expected = "Error(Contract, #12)")] // Expect InvalidPaginationArguments (#12)
fn test_get_volunteer_participations_pagination() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let volunteer = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Paginate_Org");

    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);

    let mut expected_participations = Vec::new(&env);
    let metadata: Option<SdkString> = None;

    // Participation 0
    let task_id_0 = str_to_sdkstring(&env, "task-p-0");
    let task_name_0 = str_to_sdkstring(&env, "Paginated_Task_0");
    let timestamp_0 = 1700000000u64;
    setup_ledger_time(&env, timestamp_0);
    client.register_participation(&organization, &volunteer, &task_id_0, &task_name_0, &metadata);
    expected_participations.push_back(Participation { volunteer: volunteer.clone(), task_id: task_id_0.clone(), task_name: task_name_0.clone(), timestamp: timestamp_0, organization: organization.clone(), metadata: metadata.clone() });

    // Participation 1
    let task_id_1 = str_to_sdkstring(&env, "task-p-1");
    let task_name_1 = str_to_sdkstring(&env, "Paginated_Task_1");
    let timestamp_1 = 1700001000u64;
    setup_ledger_time(&env, timestamp_1);
    client.register_participation(&organization, &volunteer, &task_id_1, &task_name_1, &metadata);
    expected_participations.push_back(Participation { volunteer: volunteer.clone(), task_id: task_id_1.clone(), task_name: task_name_1.clone(), timestamp: timestamp_1, organization: organization.clone(), metadata: metadata.clone() });

    // Participation 2
    let task_id_2 = str_to_sdkstring(&env, "task-p-2");
    let task_name_2 = str_to_sdkstring(&env, "Paginated_Task_2");
    let timestamp_2 = 1700002000u64;
    setup_ledger_time(&env, timestamp_2);
    client.register_participation(&organization, &volunteer, &task_id_2, &task_name_2, &metadata);
    expected_participations.push_back(Participation { volunteer: volunteer.clone(), task_id: task_id_2.clone(), task_name: task_name_2.clone(), timestamp: timestamp_2, organization: organization.clone(), metadata: metadata.clone() });

    // Participation 3
    let task_id_3 = str_to_sdkstring(&env, "task-p-3");
    let task_name_3 = str_to_sdkstring(&env, "Paginated_Task_3");
    let timestamp_3 = 1700003000u64;
    setup_ledger_time(&env, timestamp_3);
    client.register_participation(&organization, &volunteer, &task_id_3, &task_name_3, &metadata);
    expected_participations.push_back(Participation { volunteer: volunteer.clone(), task_id: task_id_3.clone(), task_name: task_name_3.clone(), timestamp: timestamp_3, organization: organization.clone(), metadata: metadata.clone() });

    // Participation 4
    let task_id_4 = str_to_sdkstring(&env, "task-p-4");
    let task_name_4 = str_to_sdkstring(&env, "Paginated_Task_4");
    let timestamp_4 = 1700004000u64;
    setup_ledger_time(&env, timestamp_4);
    client.register_participation(&organization, &volunteer, &task_id_4, &task_name_4, &metadata);
    expected_participations.push_back(Participation { volunteer: volunteer.clone(), task_id: task_id_4.clone(), task_name: task_name_4.clone(), timestamp: timestamp_4, organization: organization.clone(), metadata: metadata.clone() });

    // Test pagination scenarios
    let page1 = client.get_volunteer_participations(&volunteer, &0, &2);
    assert_eq!(page1.len(), 2);
    assert_eq!(page1.get_unchecked(0), expected_participations.get_unchecked(0));
    assert_eq!(page1.get_unchecked(1), expected_participations.get_unchecked(1));

    let page2 = client.get_volunteer_participations(&volunteer, &2, &2);
    assert_eq!(page2.len(), 2);
    assert_eq!(page2.get_unchecked(0), expected_participations.get_unchecked(2));
    assert_eq!(page2.get_unchecked(1), expected_participations.get_unchecked(3));

    let page3 = client.get_volunteer_participations(&volunteer, &4, &2);
    assert_eq!(page3.len(), 1);
    assert_eq!(page3.get_unchecked(0), expected_participations.get_unchecked(4));

    let page4 = client.get_volunteer_participations(&volunteer, &5, &2); 
    assert_eq!(page4.len(), 0);

    let page5 = client.get_volunteer_participations(&volunteer, &0, &10);
    assert_eq!(page5.len(), 5);
    assert_eq!(page5, expected_participations);

    client.get_volunteer_participations(&volunteer, &0, &0);

    let other_volunteer = Address::generate(&env);
    let page7 = client.get_volunteer_participations(&other_volunteer, &0, &5);
    assert_eq!(page7.len(), 0);
}

#[test]
fn test_get_task_volunteers_pagination() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    let admin = Address::generate(&env);
    let organization = Address::generate(&env);
    let org_name = str_to_sdkstring(&env, "Task_Vol_Org");
    let task_id = str_to_sdkstring(&env, "common-task");
    let task_name = str_to_sdkstring(&env, "Shared_Task");
    let metadata: Option<SdkString> = None;

    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &organization, &org_name);

    let mut expected_volunteers = Vec::new(&env);

    // Volunteer 0
    let volunteer_0 = Address::generate(&env);
    expected_volunteers.push_back(volunteer_0.clone());
    setup_ledger_time(&env, 1700000000u64);
    client.register_participation(&organization, &volunteer_0, &task_id, &task_name, &metadata);

    // Volunteer 1
    let volunteer_1 = Address::generate(&env);
    expected_volunteers.push_back(volunteer_1.clone());
    setup_ledger_time(&env, 1700001000u64);
    client.register_participation(&organization, &volunteer_1, &task_id, &task_name, &metadata);

    // Volunteer 2
    let volunteer_2 = Address::generate(&env);
    expected_volunteers.push_back(volunteer_2.clone());
    setup_ledger_time(&env, 1700002000u64);
    client.register_participation(&organization, &volunteer_2, &task_id, &task_name, &metadata);

    // Volunteer 3
    let volunteer_3 = Address::generate(&env);
    expected_volunteers.push_back(volunteer_3.clone());
    setup_ledger_time(&env, 1700003000u64);
    client.register_participation(&organization, &volunteer_3, &task_id, &task_name, &metadata);

    // Volunteer 4
    let volunteer_4 = Address::generate(&env);
    expected_volunteers.push_back(volunteer_4.clone());
    setup_ledger_time(&env, 1700004000u64);
    client.register_participation(&organization, &volunteer_4, &task_id, &task_name, &metadata);

    // Test pagination scenarios
    let page1 = client.get_task_volunteers(&task_id, &0, &2);
    assert_eq!(page1.len(), 2);
    assert_eq!(page1.get_unchecked(0), expected_volunteers.get_unchecked(0));
    assert_eq!(page1.get_unchecked(1), expected_volunteers.get_unchecked(1));

    let page2 = client.get_task_volunteers(&task_id, &2, &2);
    assert_eq!(page2.len(), 2);
    assert_eq!(page2.get_unchecked(0), expected_volunteers.get_unchecked(2));
    assert_eq!(page2.get_unchecked(1), expected_volunteers.get_unchecked(3));

    let page3 = client.get_task_volunteers(&task_id, &4, &2);
    assert_eq!(page3.len(), 1);
    assert_eq!(page3.get_unchecked(0), expected_volunteers.get_unchecked(4));

    let page4 = client.get_task_volunteers(&task_id, &5, &2);
    assert_eq!(page4.len(), 0);

    let page5 = client.get_task_volunteers(&task_id, &0, &10);
    assert_eq!(page5.len(), 5);
    assert_eq!(page5, expected_volunteers);

    let other_task = str_to_sdkstring(&env, "other-task");
    let page7 = client.get_task_volunteers(&other_task, &0, &5);
    assert_eq!(page7.len(), 0);
}

#[test]
fn test_get_organization_participations_pagination() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let org1 = Address::generate(&env);
    let org2 = Address::generate(&env);
    let org1_name = str_to_sdkstring(&env, "Org_One");
    let org2_name = str_to_sdkstring(&env, "Org_Two");
    let metadata: Option<SdkString> = None;

    env.mock_all_auths();
    client.initialize(&admin);
    client.register_organization(&admin, &org1, &org1_name);
    client.register_organization(&admin, &org2, &org2_name);

    let mut org1_expected_participations = Vec::new(&env);

    // Org1 Participation 0
    let volunteer_o1_0 = Address::generate(&env);
    let task_id_o1_0 = str_to_sdkstring(&env, "org1-task-0");
    let task_name_o1_0 = str_to_sdkstring(&env, "Org1_Task_0");
    let timestamp_o1_0 = 1700000000u64;
    setup_ledger_time(&env, timestamp_o1_0);
    client.register_participation(&org1, &volunteer_o1_0, &task_id_o1_0, &task_name_o1_0, &metadata);
    org1_expected_participations.push_back(Participation { volunteer: volunteer_o1_0.clone(), task_id: task_id_o1_0.clone(), task_name: task_name_o1_0.clone(), timestamp: timestamp_o1_0, organization: org1.clone(), metadata: metadata.clone() });

    // Org1 Participation 1
    let volunteer_o1_1 = Address::generate(&env);
    let task_id_o1_1 = str_to_sdkstring(&env, "org1-task-1");
    let task_name_o1_1 = str_to_sdkstring(&env, "Org1_Task_1");
    let timestamp_o1_1 = 1700001000u64;
    setup_ledger_time(&env, timestamp_o1_1);
    client.register_participation(&org1, &volunteer_o1_1, &task_id_o1_1, &task_name_o1_1, &metadata);
    org1_expected_participations.push_back(Participation { volunteer: volunteer_o1_1.clone(), task_id: task_id_o1_1.clone(), task_name: task_name_o1_1.clone(), timestamp: timestamp_o1_1, organization: org1.clone(), metadata: metadata.clone() });

    // Org1 Participation 2
    let volunteer_o1_2 = Address::generate(&env);
    let task_id_o1_2 = str_to_sdkstring(&env, "org1-task-2");
    let task_name_o1_2 = str_to_sdkstring(&env, "Org1_Task_2");
    let timestamp_o1_2 = 1700002000u64;
    setup_ledger_time(&env, timestamp_o1_2);
    client.register_participation(&org1, &volunteer_o1_2, &task_id_o1_2, &task_name_o1_2, &metadata);
    org1_expected_participations.push_back(Participation { volunteer: volunteer_o1_2.clone(), task_id: task_id_o1_2.clone(), task_name: task_name_o1_2.clone(), timestamp: timestamp_o1_2, organization: org1.clone(), metadata: metadata.clone() });

    // Org1 Participation 3
    let volunteer_o1_3 = Address::generate(&env);
    let task_id_o1_3 = str_to_sdkstring(&env, "org1-task-3");
    let task_name_o1_3 = str_to_sdkstring(&env, "Org1_Task_3");
    let timestamp_o1_3 = 1700003000u64;
    setup_ledger_time(&env, timestamp_o1_3);
    client.register_participation(&org1, &volunteer_o1_3, &task_id_o1_3, &task_name_o1_3, &metadata);
    org1_expected_participations.push_back(Participation { volunteer: volunteer_o1_3.clone(), task_id: task_id_o1_3.clone(), task_name: task_name_o1_3.clone(), timestamp: timestamp_o1_3, organization: org1.clone(), metadata: metadata.clone() });

    // Org1 Participation 4
    let volunteer_o1_4 = Address::generate(&env);
    let task_id_o1_4 = str_to_sdkstring(&env, "org1-task-4");
    let task_name_o1_4 = str_to_sdkstring(&env, "Org1_Task_4");
    let timestamp_o1_4 = 1700004000u64;
    setup_ledger_time(&env, timestamp_o1_4);
    client.register_participation(&org1, &volunteer_o1_4, &task_id_o1_4, &task_name_o1_4, &metadata);
    org1_expected_participations.push_back(Participation { volunteer: volunteer_o1_4.clone(), task_id: task_id_o1_4.clone(), task_name: task_name_o1_4.clone(), timestamp: timestamp_o1_4, organization: org1.clone(), metadata: metadata.clone() });

    // Register 1 participation for Org2
    let vol_org2 = Address::generate(&env);
    let task_id_org2 = str_to_sdkstring(&env, "org2-task-0");
    let task_name_org2 = str_to_sdkstring(&env,"Org2_Task");
    let timestamp_org2 = 1700000100u64;
    setup_ledger_time(&env, timestamp_org2);
    client.register_participation(&org2, &vol_org2, &task_id_org2, &task_name_org2, &metadata);
    let org2_participation = Participation { volunteer: vol_org2.clone(), task_id: task_id_org2.clone(), task_name: task_name_org2.clone(), timestamp: timestamp_org2, organization: org2.clone(), metadata: metadata.clone() };

    // Test pagination scenarios for Org1
    let page1 = client.get_organization_participations(&org1, &0, &2);
    assert_eq!(page1.len(), 2);
    assert_eq!(page1.get_unchecked(0), org1_expected_participations.get_unchecked(0));
    assert_eq!(page1.get_unchecked(1), org1_expected_participations.get_unchecked(1));

    let page2 = client.get_organization_participations(&org1, &2, &2);
    assert_eq!(page2.len(), 2);
    assert_eq!(page2.get_unchecked(0), org1_expected_participations.get_unchecked(2));
    assert_eq!(page2.get_unchecked(1), org1_expected_participations.get_unchecked(3));

    let page3 = client.get_organization_participations(&org1, &4, &2);
    assert_eq!(page3.len(), 1);
    assert_eq!(page3.get_unchecked(0), org1_expected_participations.get_unchecked(4));

    let page4 = client.get_organization_participations(&org1, &5, &2);
    assert_eq!(page4.len(), 0);

    let page5 = client.get_organization_participations(&org1, &0, &10);
    assert_eq!(page5.len(), 5);
    assert_eq!(page5, org1_expected_participations);

    // Check Org3 (no participations)
    let org3 = Address::generate(&env);
    client.register_organization(&admin, &org3, &str_to_sdkstring(&env, "Org_Three"));
    let page7 = client.get_organization_participations(&org3, &0, &5);
    assert_eq!(page7.len(), 0);

    // Check Org2 returns only its own participation
    let page8 = client.get_organization_participations(&org2, &0, &5);
    assert_eq!(page8.len(), 1);
    assert_eq!(page8.get_unchecked(0), org2_participation);
}

#[test]
fn test_get_all_organizations() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    let admin = Address::generate(&env);
    let org1 = Address::generate(&env);
    let org2 = Address::generate(&env);
    let name1 = str_to_sdkstring(&env, "Org_A");
    let name2 = str_to_sdkstring(&env, "Org_B");

    env.mock_all_auths();
    client.initialize(&admin);

    let initial_orgs = client.get_all_organizations();
    assert_eq!(initial_orgs.len(), 0);

    client.register_organization(&admin, &org1, &name1);
    client.register_organization(&admin, &org2, &name2);

    let current_orgs = client.get_all_organizations();
    assert_eq!(current_orgs.len(), 2);

    assert!(current_orgs.contains(&org1));
    assert!(current_orgs.contains(&org2));

    client.remove_organization(&admin, &org1);

    let final_orgs = client.get_all_organizations();
    assert_eq!(final_orgs.len(), 1);
    assert!(!final_orgs.contains(&org1));
    assert!(final_orgs.contains(&org2));
    assert_eq!(final_orgs.get_unchecked(0), org2); // Only org2 should remain
}