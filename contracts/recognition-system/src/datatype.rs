use soroban_sdk::{contracterror, contracttype, Address, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Recognition(Address),
    Badges(Address),
    Events  // ??
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub title: String,
    pub date: String,
    pub organization: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NFTMetadata {
    pub ev_title: String,
    pub ev_date: String,
    pub ev_org: String,
    pub ev_task: String,
}