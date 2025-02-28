use soroban_sdk::{contracttype, Address, String};

#[contracttype]
pub enum DataKey {
    ProjectBudget(u32),        // Project ID -> total budget
    ProjectOwner(u32),         // Project ID -> owner address
    ProjectOrg(u32),           // Project ID -> organization address
    Milestones(u32),           // Project ID -> milestone list
    MilestoneStatus(u32, u32), // (Project ID, Milestone ID) -> completed status
    FundRequests(u32),         // Project ID -> fund requests list
    NextProjectId,             // Counter for project IDs
    Organizations,             // Authorized organizations
    Transactions,              // Public ledger of transactions
}

#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub id: u32,
    pub description: String,
    pub amount: u32,     // Amount allocated for this milestone
    pub completed: bool, // Whether milestone is completed
    pub released: bool,  // Whether funds have been released
}

#[contracttype]
#[derive(Clone)]
pub struct FundRequest {
    pub id: u32,
    pub milestone_id: u32,
    pub amount: u32,
    pub status: RequestStatus,
    pub requester: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    pub project_id: u32,
    pub amount: u32,
    pub transaction_type: TransactionType,
    pub from: Address,
    pub to: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(PartialEq, Clone)]
pub enum RequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[contracttype]
#[derive(Clone)]
pub enum TransactionType {
    Allocation, // Initial fund allocation to project
    Release,    // Release funds for milestone
    Return,     // Return unused funds
}
