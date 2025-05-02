#![no_std]
use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, Env, Map, Symbol, Vec,
};

mod error;
mod events;
mod test;
mod types;

use error::*;
use events::*;
use types::*;

#[contract]
pub struct GoalTrackerContract;

#[contractimpl]
impl GoalTrackerContract {
    // ================= Initialization =================

    /// Initializes the contract, setting the admin and the initial progress updater.

    pub fn initialize(env: Env, admin: Address, updater: Address) -> Result<(), ContractError> {
        let storage = env.storage().instance();
        if storage.has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        admin.require_auth();

        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Updater, &updater);
        storage.set(&DataKey::NextId, &1u64); // Starting id from 1
        storage.set(&DataKey::Goals, &Map::<u64, Goal>::new(&env));

        // Publish events
        env.events()
            .publish((ADMIN_SET, symbol_short!("admin")), admin.clone());
        env.events()
            .publish((UPDATER_SET, symbol_short!("updater")), updater.clone());
        Ok(())
    }

    // ================= Configuration =================

    /// Allows the current admin to set a new progress updater address.
    /// Requires authorization from the current admin.

    pub fn set_updater(env: Env, new_updater: Address) -> Result<bool, ContractError> {
        let storage = env.storage().instance();
        let admin: Address = match storage.get(&DataKey::Admin) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };

        admin.require_auth();

        storage.set(&DataKey::Updater, &new_updater);
        env.events()
            .publish((UPDATER_SET, symbol_short!("updater")), new_updater);
        Ok(true)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<bool, ContractError> {
        let storage = env.storage().instance();
        let admin: Address = match storage.get(&DataKey::Admin) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };
        admin.require_auth();

        storage.set(&DataKey::Admin, &new_admin);
        env.events()
            .publish((ADMIN_SET, symbol_short!("admin")), new_admin);
        Ok(true)
    }

    pub fn remove_updater(env: Env) -> Result<bool, ContractError> {
        let storage = env.storage().instance();
        let admin: Address = match storage.get(&DataKey::Admin) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };

        admin.require_auth();
        storage.remove(&DataKey::Updater);

        env.events().publish(
            (UPDATER_SET, symbol_short!("updater")),
            Symbol::new(&env, "none"),
        );
        Ok(true)
    }

    // ================= Core Goal Logic =================

    pub fn create_goal(
        env: Env,
        volunteer: Address,
        goal_type: GoalType,
        target_amount: u64,
    ) -> Result<u64, ContractError> {
        if target_amount == 0 {
            return Err(ContractError::TargetMustBePositive);
        }

        volunteer.require_auth();

        let storage = env.storage().instance();

        // Get and increment the next goal ID
        let goal_id = match storage.get(&DataKey::NextId) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };
        storage.set(&DataKey::NextId, &(goal_id + 1));

        // Create  goal struct
        let goal = Goal {
            id: goal_id,
            volunteer: volunteer.clone(),
            goal_type,
            target_amount,
            current_amount: 0,
            is_completed: false,
        };

        // Store  goal in the main map
        let mut goals_map: Map<u64, Goal> = match storage.get(&DataKey::Goals) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };
        goals_map.set(goal_id, goal.clone());
        storage.set(&DataKey::Goals, &goals_map);

        // Add  goal ID to the users list
        let user_key = DataKey::UserGoals(volunteer.clone());
        let mut user_goals_vec = match storage.get::<DataKey, Vec<u64>>(&user_key) {
            Some(x) => x,
            None => Vec::new(&env),
        };
        user_goals_vec.push_back(goal_id);
        storage.set(&user_key, &user_goals_vec);

        env.events().publish(
            (GOAL_CREATED, symbol_short!("goal")),
            (goal_id, volunteer, goal_type, target_amount),
        );

        Ok(goal_id)
    }

    pub fn update_progress(
        env: Env,
        goal_id: u64,
        amount_to_add: u64,
    ) -> Result<bool, ContractError> {
        if amount_to_add == 0 {
            panic_with_error!(&env, ContractError::AmountToAddMustBePositive);
        }

        let storage = env.storage().instance();

        let updater: Address = match storage.get(&DataKey::Updater) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };
        updater.require_auth();

        // Get the goals map
        let mut goals_map: Map<u64, Goal> = match storage.get(&DataKey::Goals) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };

        let mut goal = match goals_map.get(goal_id) {
            Some(x) => x,
            None => return Err(ContractError::GoalNotFound),
        };

        if goal.is_completed {
            return Err(ContractError::GoalAlreadyCompleted);
        }

        goal.current_amount = goal.current_amount.saturating_add(amount_to_add);

        env.events().publish(
            (GOAL_UPDATED, symbol_short!("goal_prog")),
            (goal_id, goal.current_amount, amount_to_add),
        );

        if goal.current_amount >= goal.target_amount {
            goal.is_completed = true;

            env.events().publish(
                (GOAL_COMPLETED, symbol_short!("goal_done")),
                (goal_id, goal.volunteer.clone(), goal.goal_type),
            );
        }

        goals_map.set(goal_id, goal);
        storage.set(&DataKey::Goals, &goals_map);

        Ok(true)
    }

    // ================= View Functions =================

    pub fn get_goal(env: Env, goal_id: u64) -> Result<Goal, ContractError> {
        let storage = env.storage().instance();
        let goals_map: Map<u64, Goal> = match storage.get(&DataKey::Goals) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        };
        Ok(match goals_map.get(goal_id) {
            Some(x) => x,
            None => return Err(ContractError::GoalNotFound),
        })
    }

    pub fn get_goals_by_user(env: Env, volunteer: Address) -> Result<Vec<u64>, ContractError> {
        let storage = env.storage().instance();

        if !storage.has(&DataKey::Admin) {
            return Err(ContractError::NotInitialized);
        }
        let user_key = DataKey::UserGoals(volunteer);

        Ok(match storage.get::<DataKey, Vec<u64>>(&user_key) {
            Some(x) => x,
            None => Vec::new(&env),
        })
    }

    /// Retrieves the address of the current progress updater.
    pub fn get_updater(env: Env) -> Result<Address, ContractError> {
        Ok(match env.storage().instance().get(&DataKey::Updater) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        })
    }

    /// Retrieves the address of the current admin.
    pub fn get_admin(env: Env) -> Result<Address, ContractError> {
        Ok(match env.storage().instance().get(&DataKey::Admin) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        })
    }

    /// Retrieves the next ID that will be assigned to a goal.
    pub fn get_next_id(env: Env) -> Result<u64, ContractError> {
        Ok(match env.storage().instance().get(&DataKey::NextId) {
            Some(x) => x,
            None => return Err(ContractError::NotInitialized),
        })
    }
}
