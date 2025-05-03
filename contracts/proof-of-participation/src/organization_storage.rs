use soroban_sdk::{Address, Env, String, Vec};
use crate::storage::{DataKey, bump_persistent_ttl, get_vec_from_persistent_storage, bump_instance_ttl};

pub fn store_organization(env: &Env, organization: &Address, name: &String) {
    let org_key = DataKey::Organization(organization.clone());
    env.storage().persistent().set(&org_key, name);
    bump_persistent_ttl(env, &org_key); // Bump TTL for the specific org entry

    let list_key = DataKey::OrganizationList;
    let mut organizations: Vec<Address> = get_vec_from_persistent_storage(env, &list_key);

    if !organizations.contains(organization) {
        organizations.push_back(organization.clone());
        env.storage().persistent().set(&list_key, &organizations);
    }
    // bump list TTL on modification
    bump_persistent_ttl(env, &list_key);
    bump_instance_ttl(env); // Bump instance TTL for general contract activity
}

pub fn remove_organization_from_storage(env: &Env, organization: &Address) {
    let org_key = DataKey::Organization(organization.clone());
    env.storage().persistent().remove(&org_key);

    let list_key = DataKey::OrganizationList;
    let organizations: Vec<Address> = get_vec_from_persistent_storage(env, &list_key);

    let mut updated_orgs = Vec::new(env);
    let mut found = false;
    for org in organizations.iter() {
        if org != *organization {
            updated_orgs.push_back(org);
        } else {
            found = true;
        }
    }

    if found {
        env.storage().persistent().set(&list_key, &updated_orgs);
    }
    bump_persistent_ttl(env, &list_key);
    bump_instance_ttl(env); // Bump instance TTL for general contract activity
}

pub fn is_organization_registered(env: &Env, organization: &Address) -> bool {
     env.storage().persistent().has(&DataKey::Organization(organization.clone()))
}

pub fn get_organization_name(env: &Env, organization: &Address) -> Option<String> {
    env.storage().persistent().get(&DataKey::Organization(organization.clone()))
}

pub fn get_all_organizations(env: &Env) -> Vec<Address> {
    get_vec_from_persistent_storage(env, &DataKey::OrganizationList)
}