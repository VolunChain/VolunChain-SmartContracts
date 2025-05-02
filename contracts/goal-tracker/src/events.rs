// ================= Events =================

use soroban_sdk::{symbol_short, Symbol};

pub const GOAL_CREATED: Symbol = symbol_short!("created");
pub const GOAL_UPDATED: Symbol = symbol_short!("updated");
pub const GOAL_COMPLETED: Symbol = symbol_short!("completed");
pub const UPDATER_SET: Symbol = symbol_short!("updtr_set");
pub const ADMIN_SET: Symbol = symbol_short!("admin_set");
