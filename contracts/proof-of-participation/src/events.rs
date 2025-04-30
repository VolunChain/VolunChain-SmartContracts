use soroban_sdk::{Address, Env, String, Symbol, contracttype};

#[contracttype]
pub enum ParticipationEventType {
    OrganizationRegistered,
    OrganizationRemoved,
    ParticipationRegistered,
}

// Emit an event when an organization is registered
pub fn organization_registered(env: &Env, organization: &Address, name: &String) {
    let topics = (Symbol::new(env, "organization_registered"), organization.clone());
    env.events().publish(topics, name.clone());
}

// Emit an event when an organization is removed
pub fn organization_removed(env: &Env, organization: &Address) {
    let topics = (Symbol::new(env, "organization_removed"), organization.clone());
    env.events().publish(topics, ());
}

// Emit an event when a participation is registered
pub fn participation_registered(
    env: &Env,
    organization: &Address,
    volunteer: &Address,
    task_id: &String,
    task_name: &String,
    timestamp: u64,
) {
    let topics = (
        Symbol::new(env, "participation_registered"),
        organization.clone(),
        volunteer.clone(),
        task_id.clone(),
    );
    
    // Create event data with task name and timestamp
    let data = (task_name.clone(), timestamp);
    
    env.events().publish(topics, data);
}