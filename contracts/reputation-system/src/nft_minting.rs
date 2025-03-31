use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

#[allow(dead_code)]
pub trait BadgeMinting {
    fn mint_achievement_badge(
        env: &Env,
        recipient: &Address,
        badge_type: Symbol,
    ) -> Result<(), String>;

    fn get_badge_multiplier(badge_type: &Symbol) -> u32;
}

#[allow(dead_code)]
pub struct StandardBadgeMinting;

impl BadgeMinting for StandardBadgeMinting {
    /// Mints an achievement badge for a recipient.
    ///
    /// # Parameters
    /// - `env`: The environment context.
    /// - `recipient`: The address of the recipient.
    /// - `badge_type`: The type of badge to mint.
    ///
    /// # Returns
    /// Returns Ok(()) if the minting is successful or an error if it fails.
    fn mint_achievement_badge(
        env: &Env,
        recipient: &Address,
        badge_type: Symbol,
    ) -> Result<(), String> {
        let mut badges = env
            .storage()
            .instance()
            .get(&crate::DataKey::Badges(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));

        badges.push_back(badge_type);
        env.storage()
            .instance()
            .set(&crate::DataKey::Badges(recipient.clone()), &badges);
        Ok(())
    }

    /// Gets the multiplier for a given badge type.
    ///
    /// # Parameters
    /// - `badge_type`: The type of badge to get the multiplier for.
    ///
    /// # Returns
    /// Returns the multiplier associated with the badge type.
    fn get_badge_multiplier(badge_type: &Symbol) -> u32 {
        match badge_type {
            s if s == &symbol_short!("GOLD") => 30,
            s if s == &symbol_short!("SILVER") => 20,
            s if s == &symbol_short!("BRONZE") => 10,
            _ => 5,
        }
    }
}
