use soroban_sdk::{
    contract, contractimpl, contracttype, map, symbol_short, Address, Env, Map, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Feedback {
    pub giver: Address,
    pub receiver: Address,
    pub task_id: u64,
    pub rating: u32,
    pub comment: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Feedbacks,    // Address => Vec<Feedback>
    HasFeedback,  // (giver, receiver, task_id) => bool
    Participants, // task_id => Vec<Address>
}

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
            .get(&DataKey::Participants)
            .unwrap_or(map![&env]);

        let mut participants = task_participants
            .get(task_id)
            .unwrap_or_else(|| Vec::new(&env));

        if !participants.contains(&participant) {
            participants.push_back(participant.clone());
            task_participants.set(task_id, participants);
            env.storage()
                .persistent()
                .set(&DataKey::Participants, &task_participants);
        }
    }

    pub fn submit_feedback(
        env: Env,
        receiver: Address,
        giver: Address,
        task_id: u64,
        rating: u32,
        comment: String,
    ) -> Vec<Feedback> {
        // 1. Auth & sanity checks
        giver.require_auth();
        if receiver == giver {
            panic!("Cannot give feedback to self");
        }

        // 2. Participant validation
        let participants_map: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&DataKey::Participants)
            .unwrap_or(map![&env]);
        let participants = participants_map
            .get(task_id)
            .unwrap_or_else(|| Vec::new(&env));
        if !participants.contains(&giver) || !participants.contains(&receiver) {
            panic!("Unauthorized participant");
        }

        // 3. Duplicate check
        let mut has_fb_map: Map<(Address, Address, u64), bool> = env
            .storage()
            .persistent()
            .get(&DataKey::HasFeedback)
            .unwrap_or(map![&env]);
        let fb_key = (giver.clone(), receiver.clone(), task_id);
        if has_fb_map.get(fb_key.clone()).unwrap_or(false) {
            panic!("Feedback already submitted");
        }

        // 4. Validation
        if rating < 1 || rating > 5 {
            panic!("Invalid rating");
        }
        if comment.len() > 500 {
            panic!("Comment too long");
        }

        // 5. Build feedback entry
        let feedback = Feedback {
            giver: giver.clone(),
            receiver: receiver.clone(),
            task_id,
            rating,
            comment: comment.clone(),
            timestamp: env.ledger().timestamp(),
        };

        // Append to feedback list
        let mut feedbacks_map: Map<Address, Vec<Feedback>> = env
            .storage()
            .persistent()
            .get(&DataKey::Feedbacks)
            .unwrap_or(map![&env]);
        let mut feedback_list = feedbacks_map
            .get(receiver.clone())
            .unwrap_or_else(|| Vec::new(&env));

        feedback_list.push_back(feedback.clone());
        feedbacks_map.set(receiver.clone(), feedback_list.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Feedbacks, &feedbacks_map);

        // Mark feedback as submitted
        has_fb_map.set(fb_key, true);
        env.storage()
            .persistent()
            .set(&DataKey::HasFeedback, &has_fb_map);

        // Emit event
        let contract_addr = env.current_contract_address();
        env.events().publish(
            (
                contract_addr,
                symbol_short!("Feedback"),
                giver.clone(),
                receiver.clone(),
                task_id,
            ),
            (), // event payload (empty)
        );

        feedback_list
    }

    pub fn get_feedbacks(env: Env, user: Address) -> Vec<Feedback> {
        let feedbacks_map: Map<Address, Vec<Feedback>> = env
            .storage()
            .persistent()
            .get(&DataKey::Feedbacks)
            .unwrap_or(map![&env]);
        feedbacks_map.get(user).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_feedback_count(env: Env, user: Address) -> u32 {
        Self::get_feedbacks(env, user).len()
    }

    pub fn get_participents(env: Env, task_id: u64) -> Vec<Address> {
        let participants_map: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&DataKey::Participants)
            .unwrap_or(map![&env]);
        participants_map
            .get(task_id)
            .unwrap_or_else(|| Vec::new(&env))
    }
}
