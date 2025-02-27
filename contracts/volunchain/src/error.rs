use core::fmt;
use soroban_sdk::contracterror;

#[derive(Debug, Copy, Clone, PartialEq)]
#[contracterror]

pub enum ContractError {
    BountyNotFound = 1,
    ContractBalanceNotEnoughToSendRewards = 2,
    TasksNotCompleted = 3,
    OnlyVolunteerCanWithdrawRewards = 4,
    AdminNotFound = 5,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::BountyNotFound => write!(f, "Bounty not found"),
            ContractError::ContractBalanceNotEnoughToSendRewards => write!(f, "The contract does not have sufficient balance to send the reward"),
            ContractError::TasksNotCompleted => write!(f, "The tasks is not completed"),
            ContractError::OnlyVolunteerCanWithdrawRewards => write!(f, "Only the volunteer can withdraw the reward"),
            ContractError::AdminNotFound => write!(f, "Admin not found"),
        }
    }
}