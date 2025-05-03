#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, map, symbol_short, vec, Address, Env, Map, String,
    Symbol, Vec,
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

        let mut task_participants: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&PARTICIPANTS)
            .unwrap_or(map![&env]);

        let mut participants = task_participants.get(task_id).unwrap_or(vec![&env]);
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
        giver.require_auth();

        if receiver == giver {
            panic!("Cannot give feedback to self");
        }

        let task_participants: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&PARTICIPANTS)
            .unwrap_or(map![&env]);
        let participants = task_participants.get(task_id).unwrap_or(vec![&env]);
        if !participants.contains(&giver) || !participants.contains(&receiver) {
            panic!("Unauthorized participant");
        }

        let feedback_key = (giver.clone(), receiver.clone(), task_id);
        let has_feedback: Map<(Address, Address, u64), bool> = env
            .storage()
            .persistent()
            .get(&HAS_FEEDBACK)
            .unwrap_or(map![&env]);
        if has_feedback.get(feedback_key.clone()).unwrap_or(false) {
            panic!("Feedback already submitted");
        }

        if rating < 1 || rating > 5 {
            panic!("Invalid rating");
        }

        if comment.len() > 500 {
            panic!("Comment too long");
        }

        let feedback = Feedback {
            giver: giver.clone(),
            receiver: receiver.clone(),
            task_id,
            rating,
            comment: comment.clone(),
            timestamp: env.ledger().timestamp(),
        };
    
        // Get existing feedbacks or create new map
        let mut feedbacks: Map<Address, Vec<Feedback>> = env
            .storage()
            .persistent()
            .get(&FEEDBACKS)
            .unwrap_or_else(|| map![&env]);
    
        // Get user's feedbacks or create new vector
        let mut user_feedbacks = feedbacks.get(receiver.clone()).unwrap_or_else(|| vec![&env]);
    
        // Add new feedback
        user_feedbacks.push_back(feedback.clone());
        
        // Update storage
        feedbacks.set(receiver.clone(), user_feedbacks);
        env.storage().persistent().set(&FEEDBACKS, &feedbacks);

        let mut has_feedback = has_feedback;
        has_feedback.set(feedback_key, true);
        env.storage().persistent().set(&HAS_FEEDBACK, &has_feedback);

        env.events().publish(
            (symbol_short!("Feedback"), giver, receiver, task_id),
            (rating, comment.clone(), env.ledger().timestamp()),
        );
    }

    pub fn get_feedbacks(env: Env, user: Address) -> Vec<Feedback> {
        let feedbacks: Map<Address, Vec<Feedback>> = env
            .storage()
            .persistent()
            .get(&FEEDBACKS)
            .unwrap_or(map![&env]);
        feedbacks.get(user).unwrap_or(vec![&env])
    }

    pub fn get_feedback_count(env: Env, user: Address) -> u32 {
        Self::get_feedbacks(env, user).len()
    }
}