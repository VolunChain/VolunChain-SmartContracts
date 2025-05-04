#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, map, symbol_short, vec, Address, Env, IntoVal, Map, String, Symbol, Vec
};
mod feedback_rating;

#[contracttype]
#[derive(Clone, Debug)]
pub struct Feedback {
    giver: Address,
    receiver: Address,
    task_id: u64,
    rating: u32,
    comment: String,
    timestamp: u64,
}

// Storage keys
const FEEDBACKS: Symbol = symbol_short!("FEEDBACKS");
const HAS_FEEDBACK: Symbol = symbol_short!("HAS_FB");
const PARTICIPANTS: Symbol = symbol_short!("PARTS");

#[contract]
pub struct FeedbackAndRating;

#[contractimpl]
impl FeedbackAndRating {
    pub fn initialize(_env: Env) {}
    
    pub fn register_participant(env: Env, task_id: u64, participant: Address) {
        participant.require_auth();
        let admin = env.current_contract_address();
        admin.require_auth();

        // Load or initialize participants map
        let mut task_participants: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&PARTICIPANTS)
            .unwrap_or(map![&env]);

        // Load or start with an empty vec
        let mut participants = task_participants
            .get(task_id)
            .unwrap_or_else(|| Vec::new(&env));

        if !participants.contains(&participant) {
            participants.push_back(participant.clone());
            task_participants.set(task_id, participants);
            env.storage()
                .persistent()
                .set(&PARTICIPANTS, &task_participants);
        }
    }
    pub fn submit_feedback(
        env: Env,
        receiver: Address,
        giver: Address,
        task_id: u64,
        rating: u32,
        comment: String,
    ) {
        // 1) AUTH & SANITY
        giver.require_auth();
        if receiver == giver {
            panic!("Cannot give feedback to self");
        }
    
        // 2) PARTICIPANT CHECK
        let pm: Map<u64, Vec<Address>> =
            env.storage().persistent().get(&PARTICIPANTS).unwrap_or(map![&env]);
        let participants = pm.get(task_id).unwrap_or_else(|| Vec::new(&env));
        if !participants.contains(&giver) || !participants.contains(&receiver) {
            panic!("Unauthorized participant");
        }
    
        // 3) DUPLICATE CHECK
        let mut fb_flag: Map<(Address, Address, u64), bool> =
            env.storage().persistent().get(&HAS_FEEDBACK).unwrap_or(map![&env]);
        let key = (giver.clone(), receiver.clone(), task_id);
        if fb_flag.get(key.clone()).unwrap_or(false) {
            panic!("Feedback already submitted");
        }
    
        // 4) VALIDATION
        if rating < 1 || rating > 5 {
            panic!("Invalid rating");
        }
        if comment.len() > 500 {
            panic!("Comment too long");
        }
    
        // 5) BUILD & PERSIST
        let feedback = Feedback {
            giver:     giver.clone(),
            receiver:  receiver.clone(),
            task_id,
            rating,
            comment:   comment.clone(),
            timestamp: env.ledger().timestamp(),
        };
    
        // a) Append to FEEDBACKS[receiver]
        let mut all_fbs: Map<Address, Vec<Feedback>> =
            env.storage().persistent().get(&FEEDBACKS).unwrap_or(map![&env]);
        let mut recv_list = all_fbs
            .get(receiver.clone())
            .unwrap_or_else(|| Vec::new(&env));
        recv_list.push_back(feedback.clone());
        all_fbs.set(receiver.clone(), recv_list);
        env.storage().persistent().set(&FEEDBACKS, &all_fbs);
    
        // b) Mark in HAS_FEEDBACK
        fb_flag.set(key, true);
        env.storage().persistent().set(&HAS_FEEDBACK, &fb_flag);
    
        // 6) EMIT EVENT (first topic = contract address)
        let contract_addr = env.current_contract_address();
        env.events().publish(
            (
                contract_addr,
                symbol_short!("Feedback"),
                giver.clone(),
                receiver.clone(),
                task_id,
            ),
            (), // payload ignored by tests
        );
    }
    
    
    pub fn get_feedbacks(env: Env, user: Address) -> Vec<Feedback> {
        let feedbacks: Map<Address, Vec<Feedback>> = env
            .storage()
            .persistent()
            .get(&FEEDBACKS)
            .unwrap_or(map![&env]);
        feedbacks.get(user).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_feedback_count(env: Env, user: Address) -> u32 {
        Self::get_feedbacks(env, user).len()
    }
}
