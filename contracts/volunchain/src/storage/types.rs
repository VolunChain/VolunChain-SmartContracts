use soroban_sdk::{contracttype, Address, String, Vec};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bounty {
    pub title: String,
    pub description: String,
    pub amount: i128,
    pub owner: Address,
    pub volunteer: Address,
    pub tasks: Vec<Task>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Difficulty {
    Low,
    Medium,
    Hard,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub description: String,
    pub status: String,
    pub completed: bool,
    pub difficulty: Difficulty
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Bounty,
    Balance(Address),
    Allowance(AllowanceDataKey),
    Admin
}