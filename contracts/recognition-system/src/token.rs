use crate::datatype::NFTMetadata;
use soroban_sdk::{contract, contractimpl};
use soroban_sdk::{Address, Env, String, Symbol, symbol_short, Vec};

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const TOKEN_COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&TOKEN_COUNTER, &0u32);
    }

    fn check_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        if caller != &admin {
            panic!("Unauthorized");
        }
    }

    pub fn mint_nft(
        env: Env,
        to: Address,
        ev_title: String,
        ev_date: String,
        ev_org: String,
        ev_task: String,
    ) -> u32 {
        to.require_auth();

        let mut current_id: u32 = env.storage().instance().get(&TOKEN_COUNTER).unwrap();
        current_id += 1;
        env.storage().instance().set(&TOKEN_COUNTER, &current_id);

        let metadata = NFTMetadata {
            ev_title,
            ev_date,
            ev_org,
            ev_task
        };

        env.storage().persistent().set(&current_id, &nft);

        current_id
    }

    pub fn burn_nft(env: Env, owner: Address, token_id: u32) {
        owner.require_auth();

        let nft: NFTMetadata = env
            .storage()
            .persistent()
            .get(&token_id)
            .expect("NFT not exist");

        if nft.owner != owner {
            panic!("Unauthorized sender");
        }

        env.storage().persistent().remove(&token_id);
    }
}
