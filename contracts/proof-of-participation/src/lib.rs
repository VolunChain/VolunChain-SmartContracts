#![no_std]

use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec,
};

mod storage;
mod organization;
mod participation;
mod events;
mod error;
mod test;

pub use error::*;
pub use events::*;
pub use organization::*;
pub use participation::*;
pub use storage::*;

#[contract]
pub struct ProofOfParticipationContract;

#[contractimpl]
impl ProofOfParticipationContract {
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if storage::has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Register a new organization
    pub fn register_organization(
        env: Env, 
        admin: Address, 
        organization: Address,
        name: String,
    ) -> Result<(), ContractError> {
        admin.require_auth();
        organization::register_organization(&env, &admin, &organization, &name)
    }

    /// Remove an organization
    pub fn remove_organization(
        env: Env, 
        admin: Address, 
        organization: Address,
    ) -> Result<(), ContractError> {
        admin.require_auth();
        organization::remove_organization(&env, &admin, &organization)
    }

    /// Register a participation record
    pub fn register_participation(
        env: Env,
        organization: Address,
        volunteer: Address,
        task_id: String,
        task_name: String,
    ) -> Result<(), ContractError> {
        organization.require_auth();
        participation::register_participation(&env, &organization, &volunteer, &task_id, &task_name)
    }

    /// Verify if a volunteer has participated in a specific task
    pub fn verify_participation(
        env: Env,
        volunteer: Address,
        task_id: String,
    ) -> bool {
        participation::verify_participation(&env, &volunteer, &task_id)
    }

    /// Get the timestamp of a participation
    pub fn get_participation_timestamp(
        env: Env,
        volunteer: Address,
        task_id: String,
    ) -> Result<u64, ContractError> {
        participation::get_participation_timestamp(&env, &volunteer, &task_id)
    }

    /// Get all participations for a volunteer
    pub fn get_volunteer_participations(
        env: Env,
        volunteer: Address,
    ) -> Vec<participation::Participation> {
        participation::get_volunteer_participations(&env, &volunteer)
    }

    /// Get all volunteers who participated in a specific task
    pub fn get_task_volunteers(
        env: Env,
        task_id: String,
    ) -> Vec<Address> {
        participation::get_task_volunteers(&env, &task_id)
    }

    /// Check if an address is a registered organization
    pub fn is_organization(env: Env, organization: Address) -> bool {
        organization::is_organization(&env, &organization)
    }
}