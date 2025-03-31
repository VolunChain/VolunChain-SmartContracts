use soroban_sdk::{contractimpl, Env, Address, Symbol, require_auth, CheckedAdd, CheckedSub};

pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    // ...existing code...

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin = get_admin(&env);
        require_auth(&admin); // Ensure only admin can mint tokens

        // Prevent overflow
        let current_balance = get_balance(&env, &to);
        let new_balance = current_balance.checked_add(amount).expect("Overflow detected");
        set_balance(&env, &to, new_balance);
    }

    pub fn distribute(env: Env, from: Address, to: Address, amount: i128) {
        require_auth(&from); // Ensure only the sender can distribute their tokens

        // Prevent underflow
        let sender_balance = get_balance(&env, &from);
        let new_sender_balance = sender_balance.checked_sub(amount).expect("Underflow detected");
        set_balance(&env, &from, new_sender_balance);

        // Prevent overflow
        let recipient_balance = get_balance(&env, &to);
        let new_recipient_balance = recipient_balance.checked_add(amount).expect("Overflow detected");
        set_balance(&env, &to, new_recipient_balance);
    }

    pub fn get_balance(env: Env, address: Address) -> i128 {
        // Read-only function
        env.storage().get(&address).unwrap_or(0)
    }

    fn set_balance(env: &Env, address: &Address, balance: i128) {
        env.storage().set(address, &balance);
    }

    fn get_admin(env: &Env) -> Address {
        // Retrieve the admin address from storage or configuration
        env.storage().get(&Symbol::new(env, "admin")).expect("Admin not set")
    }
}
