use soroban_sdk::{Address, Env, String, Symbol};

const ORG_REGISTERED_TOPIC: &str = "pop_org_registered_v1";
const ORG_REMOVED_TOPIC: &str = "pop_org_removed_v1";
const PART_REGISTERED_TOPIC: &str = "pop_part_registered_v1";


pub fn organization_registered(env: &Env, organization: &Address, name: &String) {
    let topics = (Symbol::new(env, ORG_REGISTERED_TOPIC), organization.clone()); 
    env.events().publish(topics, name.clone());
}


pub fn organization_removed(env: &Env, organization: &Address) {
    let topics = (Symbol::new(env, ORG_REMOVED_TOPIC), organization.clone());
    env.events().publish(topics, ());
}

pub fn participation_registered(
    env: &Env,
    organization: &Address,
    volunteer: &Address,
    task_id: &String,
    task_name: &String,
    metadata: &Option<String>,
    timestamp: u64,
) {
    let topics = (
        Symbol::new(env, PART_REGISTERED_TOPIC),
        organization.clone(),
        volunteer.clone(),
        task_id.clone(), 
    );
    
    let data = (task_name.clone(), timestamp, metadata.clone());

    env.events().publish(topics, data);
}