use soroban_sdk::{symbol_short, Address, Env, Symbol};

const PROPOSAL_CREATED: Symbol = symbol_short!("PROP_CRT");
const VOTE_CAST: Symbol = symbol_short!("VOTE_CST");
const PROPOSAL_STATUS: Symbol = symbol_short!("PROP_STS");
const PROPOSAL_EXECUTED: Symbol = symbol_short!("PROP_EXE");

pub fn emit_proposal_created(env: &Env, proposal_id: u32, creator: Address) {
    env.events()
        .publish((PROPOSAL_CREATED, proposal_id), creator);
}

/// Emits an event when a vote is cast
pub fn emit_vote_cast(env: &Env, proposal_id: u32, voter: Address, support: bool) {
    env.events()
        .publish((VOTE_CAST, proposal_id, voter), support);
}

/// Emits an event when a proposal is finalized (approved/rejected)
pub fn emit_proposal_finalized(env: &Env, proposal_id: u32, approved: bool) {
    env.events()
        .publish((PROPOSAL_STATUS, proposal_id), approved);
}

/// Emits an event when the DAO configuration is updated
pub fn emit_config_updated(env: &Env) {
    env.events().publish((PROPOSAL_EXECUTED,), ()); // No additional data)
}

// Event for contract initialization
pub fn emit_contract_initialized(env: &Env) {
    env.events().publish((symbol_short!("INIT"),), ());
}

pub fn emit_proposal_executed(env: &Env, proposal_id: u32) {
    env.events().publish(("proposal_executed", proposal_id), ());
}
