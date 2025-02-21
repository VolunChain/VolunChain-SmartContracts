use soroban_sdk::{Env, Symbol, String, Vec};

#[allow(dead_code)]
#[derive(Clone)]
pub struct ReputationMetadata {
    pub categories: Vec<Symbol>,
    pub level: String,
    pub last_updated: u64,
}

#[allow(dead_code)]
impl ReputationMetadata {
    pub fn new(env: &Env) -> Self {
        Self {
            categories: Vec::new(env),
            level: String::from_str(env, "Beginner"),
            last_updated: env.ledger().timestamp(),
        }
    }

    pub fn calculate_level(&mut self, env: &Env, reputation: u32) {
        self.level = String::from_str(env, match reputation {
            0..=99 => "Beginner",
            100..=499 => "Intermediate",
            500..=999 => "Advanced",
            _ => "Expert",
        });
    }
} 