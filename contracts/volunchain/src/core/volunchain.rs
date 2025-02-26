use soroban_sdk::{token::TokenClient, Address, Env};
use crate::{error::ContractError, storage::types::{Bounty, DataKey}};

pub struct VolunchainManager;

impl VolunchainManager {
    pub fn create_bounty(e: Env, bounty_properties: Bounty) -> Result<Bounty, ContractError> {
        e.storage()
            .instance()
            .set(&DataKey::Bounty, &bounty_properties);

        Ok(bounty_properties)
    }

    pub fn withdraw_reward(e: Env, volunteer: Address, trustline: Address) -> Result<Bounty, ContractError> {
        let bounty_result = Self::get_bounty(e.clone());
        let bounty = match bounty_result {
            Ok(esc) => esc,
            Err(err) => return Err(err),
        };

        if bounty.volunteer != volunteer {
            return Err(ContractError::OnlyVolunteerCanWithdrawRewards);
        }
        volunteer.require_auth();

        if !bounty
            .tasks
            .iter()
            .all(|taks| taks.completed)
        {
            return Err(ContractError::TasksNotCompleted);
        }

        let token = TokenClient::new(&e, &trustline);
        let contract_address = e.current_contract_address();
        let contract_balance = token.balance(&contract_address);

        if contract_balance < bounty.amount {
            return Err(ContractError::ContractBalanceNotEnoughToSendRewards);
        }

        token.transfer(&contract_address, &bounty.volunteer, &contract_balance);

        Ok(bounty)
    }

    pub fn get_bounty(e: Env) -> Result<Bounty, ContractError> {
        let bounty = e
            .storage()
            .instance()
            .get::<_, Bounty>(&DataKey::Bounty)
            .ok_or(ContractError::BountyNotFound);
        Ok(bounty?)
    }
}