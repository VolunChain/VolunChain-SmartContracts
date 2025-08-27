use soroban_sdk::{Address, Env, String, Vec};
use crate::storage::{DataKey, bump_persistent_ttl, get_vec_from_persistent_storage, bump_instance_ttl};
use crate::error::ContractError;

// Organization validation constants
pub const MAX_ORGANIZATION_NAME_LEN: u32 = 64;
pub const MAX_ORGANIZATIONS: u32 = 1000;

pub fn store_organization(env: &Env, organization: &Address, name: &String) -> Result<(), ContractError> {
    // Validate organization name
    if name.len() == 0 {
        return Err(ContractError::OrganizationNameEmpty);
    }
    if name.len() as u32 > MAX_ORGANIZATION_NAME_LEN {
        return Err(ContractError::OrganizationNameTooLong);
    }

    // Validate address
    let org_str = organization.to_string();
    if org_str.len() == 0 {
        return Err(ContractError::InvalidAddress);
    }

    let org_key = DataKey::Organization(organization.clone());
    env.storage().persistent().set(&org_key, name);
    bump_persistent_ttl(env, &org_key); // Bump TTL for the specific org entry

    let list_key = DataKey::OrganizationList;
    let mut organizations: Vec<Address> = get_vec_from_persistent_storage(env, &list_key);

    // Check storage limits
    if organizations.len() as u32 >= MAX_ORGANIZATIONS {
        return Err(ContractError::TooManyOrganizations);
    }

    if !organizations.contains(organization) {
        organizations.push_back(organization.clone());
        env.storage().persistent().set(&list_key, &organizations);
    }
    // bump list TTL on modification
    bump_persistent_ttl(env, &list_key);
    bump_instance_ttl(env); // Bump instance TTL for general contract activity

    Ok(())
}

pub fn remove_organization_from_storage(env: &Env, organization: &Address) -> Result<(), ContractError> {
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
        // Only bump TTL if we actually modified the list
        bump_persistent_ttl(env, &list_key);
        bump_instance_ttl(env); // Bump instance TTL for general contract activity
    }

    Ok(())
}

pub fn is_organization_registered(env: &Env, organization: &Address) -> bool {
     env.storage().persistent().has(&DataKey::Organization(organization.clone()))
}

#[allow(dead_code)]
pub fn get_organization_name(env: &Env, organization: &Address) -> Option<String> {
    env.storage().persistent().get(&DataKey::Organization(organization.clone()))
}

pub fn get_all_organizations(env: &Env) -> Vec<Address> {
    get_vec_from_persistent_storage(env, &DataKey::OrganizationList)
}