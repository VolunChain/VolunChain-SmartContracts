use reputation_system::ReputationSystem;
use soroban_sdk::{contract, contracterror, contracttype, Address, Env, String, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub title: String,
    pub date: String,
    pub organization: Address,
}

#[contract]
pub struct EventContract;

impl EventContract{
    pub fn create_event(
        env: Env,
        title: String,
        date: String,
        organization: Address,
    ) -> u64 {
        organization.require_auth();

        // Verify organization is authorized
        let organizations: Vec<Address> = ReputationSystem::get_organizations(&env);
        if !organizations.contains(&organization) {
            panic!("Unauthorized organization");
        }

        // Update event count and 
        let mut event_id: u64 = env.storage().instance().get(&DataKey::EventCounter).unwrap_or(0);
        event_id += 1;
        env.storage().instance().set(&DataKey::EventCounter, &event_id);
        
        let event = Event {
            title,
            date,
            organization: organization.clone()
        };
        env.storage().persistent().set(&event_id, &event);
        
        event_id
    }
}
    