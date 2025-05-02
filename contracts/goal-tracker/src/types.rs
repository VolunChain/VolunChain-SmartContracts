// ================= Data Structures =================

use soroban_sdk::{contracttype, Address};

// Enum for Goal Types
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum GoalType {
    TasksCompleted = 0,
    HoursVolunteered = 1,
    CertificationsEarned = 2,
}

// Struct for Goal Data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Goal {
    pub id: u64,
    pub volunteer: Address,
    pub goal_type: GoalType,
    pub target_amount: u64,
    pub current_amount: u64,
    pub is_completed: bool,
}

// Keys for persistent storage
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Updater,
    NextId,             // Counter for the next goal ID
    Goals,              // Map<u64, Goal>
    UserGoals(Address), // Vec<u64>
}
