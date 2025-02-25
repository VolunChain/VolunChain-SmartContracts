#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec};

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
pub struct Milestone {
    pub id: u32,
    pub description: String,
    pub amount: u32,     // Amount allocated for this milestone
    pub completed: bool, // Whether milestone is completed
    pub released: bool,  // Whether funds have been released
}

#[contracttype]
pub struct FundRequest {
    pub id: u32,
    pub milestone_id: u32,
    pub amount: u32,
    pub status: RequestStatus,
    pub requester: Address,
    pub timestamp: u64,
}

#[contracttype]
pub struct Transaction {
    pub project_id: u32,
    pub amount: u32,
    pub transaction_type: TransactionType,
    pub from: Address,
    pub to: Address,
    pub timestamp: u64,
}

#[contracttype]
pub enum RequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[contracttype]
pub enum TransactionType {
    Allocation, // Initial fund allocation to project
    Release,    // Release funds for milestone
    Return,     // Return unused funds
}

#[contract]
pub struct BudgetAllocation;

    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        // Initialize authorized organizations list
        let organizations: Vec<Address> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::Organizations, &organizations);

        // Initialize transaction ledger
        let transactions: Vec<Transaction> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::Transactions, &transactions);

        // Initialize project counter
        env.storage().instance().set(&DataKey::NextProjectId, &1u32);
    }

    // Add an organization to authorized list
    pub fn add_organization(env: Env, admin: Address, org: Address) {
        admin.require_auth();

        let mut organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap();
        organizations.push_back(org);
        env.storage()
            .instance()
            .set(&DataKey::Organizations, &organizations);
    }

    // Create a new project with budget allocation
    pub fn allocate_project_budget(
        env: Env,
        org: Address,
        project_owner: Address,
        total_budget: u32,
        milestone_descriptions: Vec<String>,
        milestone_amounts: Vec<u32>,
    ) -> u32 {
        org.require_auth();

        // Verify organization is authorized
        let organizations: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Organizations)
            .unwrap();
        if !organizations.contains(&org) {
            panic!("Unauthorized organization");
        }

        // Verify milestone amounts sum up to total budget
        let mut sum = 0u32;
        for amount in milestone_amounts.iter() {
            sum += amount;
        }
        if sum != total_budget {
            panic!("Milestone amounts must sum to total budget");
        }

        // Get next project ID
        let project_id = env
            .storage()
            .instance()
            .get(&DataKey::NextProjectId)
            .unwrap_or(1u32);
        env.storage()
            .instance()
            .set(&DataKey::NextProjectId, &(project_id + 1));

        // Store project budget
        env.storage()
            .instance()
            .set(&DataKey::ProjectBudget(project_id), &total_budget);

        // Store project owner and organization
        env.storage()
            .instance()
            .set(&DataKey::ProjectOwner(project_id), &project_owner);
        env.storage()
            .instance()
            .set(&DataKey::ProjectOrg(project_id), &org);

        // Create milestones
        let mut milestones: Vec<Milestone> = Vec::new(&env);
        for i in 0..milestone_descriptions.len() {
            let milestone = Milestone {
                id: i as u32,
                description: milestone_descriptions.get(i).unwrap(),
                amount: milestone_amounts.get(i).unwrap(),
                completed: false,
                released: false,
            };
            milestones.push_back(milestone);
        }
        env.storage()
            .instance()
            .set(&DataKey::Milestones(project_id), &milestones);

        // Initialize fund requests
        let fund_requests: Vec<FundRequest> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::FundRequests(project_id), &fund_requests);

        // Record transaction
        Self::record_transaction(
            &env,
            project_id,
            total_budget,
            TransactionType::Allocation,
            &org,
            &project_owner,
        );

        project_id
    }

    // Mark a milestone as completed
    pub fn complete_milestone(env: Env, org: Address, project_id: u32, milestone_id: u32) {
        org.require_auth();

        // Verify organization is authorized and owns the project
        let project_org: Address = env
            .storage()
            .instance()
            .get(&DataKey::ProjectOrg(project_id))
            .unwrap();
        if project_org != org {
            panic!("Only the project's organization can mark milestones as completed");
        }

        // Update milestone status
        let mut milestones: Vec<Milestone> = env.storage().instance().get(&DataKey::Milestones(project_id)).unwrap();
        let mut milestone = milestones.get(milestone_id).unwrap();

        if milestone.completed {
            panic!("Milestone already completed");
        }

        milestone.completed = true;
        milestones.set(milestone_id, milestone);
        env.storage().instance().set(&DataKey::Milestones(project_id), &milestones);
    }

    // Request funds for a completed milestone
    pub fn request_funds(env: Env, requester: Address, project_id: u32, milestone_id: u32) -> u32 {
        requester.require_auth();

        // Verify requester is project owner
        let project_owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::ProjectOwner(project_id))
            .unwrap();
        if project_owner != requester {
            panic!("Only project owner can request funds");
        }

        // Check if milestone is completed but not yet released
        let milestones: Vec<Milestone> = env.storage().instance().get(&DataKey::Milestones(project_id)).unwrap();
        let milestone = milestones.get(milestone_id).unwrap();
        
        if !milestone.completed {
            panic!("Milestone not completed yet");
        }

        if milestone.released {
            panic!("Funds already released for this milestone");
        }

        // Create fund request
        let mut fund_requests: Vec<FundRequest> = env
            .storage()
            .instance()
            .get(&DataKey::FundRequests(project_id))
            .unwrap();
        let request_id = fund_requests.len() as u32;

        let request = FundRequest {
            id: request_id,
            milestone_id,
            amount: milestone.amount,
            status: RequestStatus::Pending,
            requester: requester.clone(),
            timestamp: env.ledger().timestamp(),
        };

        fund_requests.push_back(request);
        env.storage()
            .instance()
            .set(&DataKey::FundRequests(project_id), &fund_requests);

        request_id
    }

    // Release funds for a milestone (approve fund request)
    pub fn release_funds(env: Env, org: Address, project_id: u32, request_id: u32) {
        org.require_auth();

        // Verify organization is authorized and owns the project
        let project_org = env
            .storage()
            .instance()
            .get(&DataKey::ProjectOrg(project_id))
            .unwrap();
        if project_org != org {
            panic!("Only the project's organization can release funds");
        }

         // Get the fund request
         let mut fund_requests: Vec<FundRequest> = env.storage().instance().get(&DataKey::FundRequests(project_id)).unwrap();
         let mut request = fund_requests.get(request_id).unwrap();
         
         if request.status != RequestStatus::Pending {
             panic!("Request is not in pending state");
         }

          // Update request status
        request.status = RequestStatus::Approved;
        fund_requests.set(request_id, request.clone());
        env.storage().instance().set(&DataKey::FundRequests(project_id), &fund_requests);
        
        // Update milestone status to released
        let mut milestones: Vec<Milestone> = env
            .storage()
            .instance()
            .get(&DataKey::Milestones(project_id))
            .unwrap();
        let mut milestone = milestones.get(request.milestone_id).unwrap();
        milestone.released = true;
        milestones.set(request.milestone_id, milestone);
        env.storage().instance().set(&DataKey::Milestones(project_id), &milestones);
        
        // Record transaction
        let project_owner = env
            .storage()
            .instance()
            .get(&DataKey::ProjectOwner(project_id))
            .unwrap();
        Self::record_transaction(
            &env,
            project_id,
            request.amount,
            TransactionType::Release,
            &org,
            &project_owner,
        );
    }

    // Return unused funds
    pub fn return_funds(env: Env, project_owner: Address, project_id: u32, amount: u32) {
        project_owner.require_auth();

        // Verify is project owner
        let stored_owner: Address = env.storage().instance().get(&DataKey::ProjectOwner(project_id)).unwrap();
        if stored_owner != project_owner {
            panic!("Only project owner can return funds");
        }

        // Get project organization
        let org = env
            .storage()
            .instance()
            .get(&DataKey::ProjectOrg(project_id))
            .unwrap();

        // Record transaction
        Self::record_transaction(
            &env,
            project_id,
            amount,
            TransactionType::Return,
            &project_owner,
            &org,
        );
    }

    // Record a transaction in the public ledger
    fn record_transaction(
        env: &Env,
        project_id: u32,
        amount: u32,
        transaction_type: TransactionType,
        from: &Address,
        to: &Address,
    ) {
        let transaction = Transaction {
            project_id,
            amount,
            transaction_type,
            from: from.clone(),
            to: to.clone(),
            timestamp: env.ledger().timestamp(),
        };

        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&DataKey::Transactions)
            .unwrap();
        transactions.push_back(transaction);
        env.storage()
            .instance()
            .set(&DataKey::Transactions, &transactions);
    }

    // Get project budget info
    pub fn get_project_budget(env: Env, project_id: u32) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProjectBudget(project_id))
            .unwrap_or(0)
    }

    // Get project milestones
    pub fn get_project_milestones(env: Env, project_id: u32) -> Vec<Milestone> {
        env.storage()
            .instance()
            .get(&DataKey::Milestones(project_id))
            .unwrap_or_else(|| Vec::new(&env))
    }

    // Get project fund requests
    pub fn get_fund_requests(env: Env, project_id: u32) -> Vec<FundRequest> {
        env.storage()
            .instance()
            .get(&DataKey::FundRequests(project_id))
            .unwrap_or_else(|| Vec::new(&env))
    }

    // Get transaction history
    pub fn get_transaction_history(env: Env) -> Vec<Transaction> {
        env.storage()
            .instance()
            .get(&DataKey::Transactions)
            .unwrap_or_else(|| Vec::new(&env))
    }

    // Get project-specific transaction history
    pub fn get_project_transactions(env: Env, project_id: u32) -> Vec<Transaction> {
        let all_transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&DataKey::Transactions)
            .unwrap_or_else(|| Vec::new(&env));
        let mut project_transactions: Vec<Transaction> = Vec::new(&env);

        for i in 0..all_transactions.len() {
            let tx = all_transactions.get(i).unwrap();
            if tx.project_id == project_id {
                project_transactions.push_back(tx);
            }
        }

        project_transactions
    }
}
#[contractimpl]
impl BudgetAllocation {

}