#![no_std]

mod error;
mod events;
mod storage;
mod organization_storage;
mod participation_storage;
mod organization;
mod participation;
mod test;

use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec 
};

pub use error::*;
pub use events::*;
pub use organization::is_organization;
pub use participation::verify_participation;



#[contract]
pub struct ProofOfParticipationContract;

#[contractimpl]
impl ProofOfParticipationContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if storage::has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        Ok(())
    }

    pub fn register_organization(
        env: Env,
        admin: Address,
        organization: Address,
        name: String,
    ) -> Result<(), ContractError> {

        admin.require_auth();
        organization::register_organization(&env, &admin, &organization, &name)
    }

    pub fn remove_organization(
        env: Env,
        admin: Address,
        organization: Address,
    ) -> Result<(), ContractError> {
        admin.require_auth();
        organization::remove_organization(&env, &admin, &organization)
    }

    pub fn is_organization(env: Env, organization: Address) -> bool {
        organization::is_organization(&env, &organization)
    }

    pub fn get_all_organizations(env: Env) -> Vec<Address> {
        organization::get_all_organizations(&env)
    }

    pub fn register_participation(
        env: Env,
        organization: Address,
        volunteer: Address,
        task_id: String,
        task_name: String,
        metadata: Option<String>,
    ) -> Result<(), ContractError> {
        participation::register_participation(&env, &organization, &volunteer, &task_id, &task_name, metadata)
    }

    pub fn verify_participation(
        env: Env,
        volunteer: Address,
        task_id: String,
    ) -> bool {
        participation::verify_participation(&env, &volunteer, &task_id)
    }

    pub fn get_participation_details(
        env: Env,
        volunteer: Address,
        task_id: String,
    ) -> Result<participation_storage::Participation, ContractError> {
        participation::get_participation_details(&env, &volunteer, &task_id)
    }

    pub fn get_volunteer_participations(
        env: Env,
        volunteer: Address,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<participation_storage::Participation>, ContractError> {
        participation::get_volunteer_participations(&env, &volunteer, offset, limit)
    }

    pub fn get_task_volunteers(
        env: Env,
        task_id: String,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Address>, ContractError> {
        participation::get_task_volunteers(&env, &task_id, offset, limit)
    }

    pub fn get_organization_participations(
        env: Env,
        organization: Address,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<participation_storage::Participation>, ContractError> {
        participation::get_organization_participations(&env, &organization, offset, limit)
    }
}