use soroban_sdk::{Address, Env, String, Vec};

use crate::error::ContractError;
use crate::events;
use crate::storage;
use crate::organization_storage;


pub fn register_organization(
    env: &Env,
    admin: &Address,
    organization: &Address,
    name: &String,
) -> Result<(), ContractError> {
    storage::check_admin(env, admin)?;

    // Validate address
    let org_str = organization.to_string();
    if org_str.len() == 0 {
        return Err(ContractError::InvalidAddress);
    }

    if organization_storage::is_organization_registered(env, organization) {
        return Err(ContractError::OrganizationAlreadyRegistered);
    }

    organization_storage::store_organization(env, organization, name)?;
    events::organization_registered(env, organization, name);

    Ok(())
}

pub fn remove_organization(
    env: &Env,
    admin: &Address,
    organization: &Address,
) -> Result<(), ContractError> {
    storage::check_admin(env, admin)?;

    if !organization_storage::is_organization_registered(env, organization) {
        return Err(ContractError::OrganizationNotRegistered);
    }

    organization_storage::remove_organization_from_storage(env, organization)?;
    events::organization_removed(env, organization);

    Ok(())
}

pub fn is_organization(env: &Env, organization: &Address) -> bool {
    organization_storage::is_organization_registered(env, organization)
}

pub fn verify_organization(env: &Env, organization: &Address) -> Result<(), ContractError> {
    if !organization_storage::is_organization_registered(env, organization) {
        Err(ContractError::OrganizationNotRegistered)
    } else {
        Ok(())
    }
}

pub fn get_all_organizations(env: &Env) -> Vec<Address> {
    organization_storage::get_all_organizations(env)
}