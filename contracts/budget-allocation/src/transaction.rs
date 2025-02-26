use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn return_funds(env: Env, project_owner: Address, project_id: u32, amount: u32) {
    project_owner.require_auth();

    // Verify is project owner
    let stored_owner: Address = env
        .storage()
        .instance()
        .get(&DataKey::ProjectOwner(project_id))
        .unwrap();
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
    record_transaction(
        &env,
        project_id,
        amount,
        TransactionType::Return,
        &project_owner,
        &org,
    );
}

pub fn record_transaction(
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
        .unwrap_or_else(|| Vec::new(&env));

    transactions.push_back(transaction);
    env.storage()
        .instance()
        .set(&DataKey::Transactions, &transactions);
}

pub fn get_transaction_history(env: Env) -> Vec<Transaction> {
    env.storage()
        .instance()
        .get(&DataKey::Transactions)
        .unwrap_or_else(|| Vec::new(&env))
}

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
