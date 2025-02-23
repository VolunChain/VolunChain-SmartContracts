#[allow(dead_code)]
pub trait ReputationDistribution {
    fn calculate_voting_power(&self, reputation: u32) -> u32;
    fn check_bounty_access(&self, reputation: u32, min_required: u32) -> bool;
}

#[allow(dead_code)]
pub struct StandardDistribution;

impl ReputationDistribution for StandardDistribution {
    fn calculate_voting_power(&self, reputation: u32) -> u32 {
        // Basic formula: 1 vote power per 100 reputation points
        reputation / 100
    }

    fn check_bounty_access(&self, reputation: u32, min_required: u32) -> bool {
        reputation >= min_required
    }
} 