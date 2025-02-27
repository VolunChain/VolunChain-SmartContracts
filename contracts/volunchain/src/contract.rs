use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

use crate::{core::VolunchainManager, error::ContractError, storage::types::Bounty};

#[contract]
pub struct VolunchainContract;

#[contractimpl]
impl VolunchainContract {
    pub fn create_bounty(e: Env, bounty_properties: Bounty) -> Result<Bounty, ContractError> {
        let created_bounty =
            VolunchainManager::create_bounty(e.clone(), bounty_properties.clone())?;
        e.events().publish((symbol_short!("crtd_bty"),), ());

        Ok(created_bounty)
    }

    pub fn withdraw_reward(e: Env, volunteer: Address, trustline: Address) -> Result<Bounty, ContractError> {
        let bounty = VolunchainManager::withdraw_reward(e.clone(), volunteer, trustline)?;
        e.events().publish((symbol_short!("wtdrw_rwd"),), ());

        Ok(bounty)
    }

    pub fn get_bounty(e: Env) -> Result<Bounty, ContractError> {
        VolunchainManager::get_bounty(e)
    }
}