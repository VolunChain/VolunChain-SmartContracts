#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod types;
mod project;
mod admin;
mod milestone;
mod storage;
mod transaction;
mod test;

pub use types::*;
pub use project::*;
pub use admin::*;
pub use milestone::*;
pub use storage::*;
pub use transaction::*;

#[contract]
pub struct BudgetAllocation;

#[contractimpl]
impl BudgetAllocation {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        admin::initialize(env, admin)
    }

    // Admin functions
    pub fn add_organization(env: Env, admin: Address, org: Address) {
        admin::add_organization(env, admin, org)
    }

    // Project functions
    pub fn allocate_project_budget(
        env: Env,
        org: Address,
        project_owner: Address,
        total_budget: u32,
        milestone_descriptions: Vec<String>,
        milestone_amounts: Vec<u32>,
    ) -> u32 {
        project::allocate_project_budget(env, org, project_owner, total_budget, milestone_descriptions, milestone_amounts)
    }

    // Milestone functions
    pub fn complete_milestone(env: Env, org: Address, project_id: u32, milestone_id: u32) {
        milestone::complete_milestone(env, org, project_id, milestone_id)
    }

    pub fn request_funds(env: Env, requester: Address, project_id: u32, milestone_id: u32) -> u32 {
        milestone::request_funds(env, requester, project_id, milestone_id)
    }

    pub fn release_funds(env: Env, org: Address, project_id: u32, request_id: u32) {
        milestone::release_funds(env, org, project_id, request_id)
    }

    // Transaction functions
    pub fn return_funds(env: Env, project_owner: Address, project_id: u32, amount: u32) {
        transaction::return_funds(env, project_owner, project_id, amount)
    }

    // Query functions
    pub fn get_project_budget(env: Env, project_id: u32) -> u32 {
        project::get_project_budget(env, project_id)
    }

    pub fn get_project_milestones(env: Env, project_id: u32) -> Vec<Milestone> {
        milestone::get_project_milestones(env, project_id)
    }

    pub fn get_fund_requests(env: Env, project_id: u32) -> Vec<FundRequest> {
        milestone::get_fund_requests(env, project_id)
    }

    pub fn get_transaction_history(env: Env) -> Vec<Transaction> {
        transaction::get_transaction_history(env)
    }

    pub fn get_project_transactions(env: Env, project_id: u32) -> Vec<Transaction> {
        transaction::get_project_transactions(env, project_id)
    }
}