#[cfg(test)]
mod tests {
    use crate::{FeedbackAndRating, FeedbackAndRatingClient};

 
    use soroban_sdk::{symbol_short, testutils::{Address as _, Events as _}, vec, Address, Env, IntoVal, String};

    fn setup_env() -> (Env, FeedbackAndRatingClient<'static>, Address, Address, u64) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(FeedbackAndRating, ());
        let client = FeedbackAndRatingClient::new(&env, &contract_id);

        let volunteer = Address::generate(&env);
        let organization = Address::generate(&env);
        let task_id = 1u64;

        // Register participants
        client.register_participant(&task_id, &volunteer);
        client.register_participant(&task_id, &organization);

        

        (env, client, volunteer, organization, task_id)
    }

    #[test]
    fn test_submit_feedback_success() {
        let (env, client, volunteer, organization, task_id) = setup_env();
        let rating = 4u32;
        let comment = String::from_str(&env, "Great work!");
    
        // Volunteer gives feedback to organization
        client.submit_feedback(&organization, &volunteer, &task_id, &rating, &comment);
    
        // Validate feedback was stored
        let feedbacks = client.get_feedbacks(&organization);
        assert_eq!(feedbacks.len(), 1);
        let feedback = feedbacks.first().unwrap();
        assert_eq!(feedback.giver, volunteer);
        assert_eq!(feedback.receiver, organization);
        assert_eq!(feedback.task_id, task_id);
        assert_eq!(feedback.rating, rating);
        assert_eq!(feedback.comment, comment);
        assert_eq!(feedback.timestamp, env.ledger().timestamp());
    
        // Validate emitted event
        let events = env.events().all();
        assert_eq!(events.len(), 1);
    
        let (emitter, topics, data) = events.first().unwrap(); // Destructure the tuple
    
        assert_eq!(
            topics,
            vec![
                &env,                               // test-utils expands this to the contract address Val
                symbol_short!("Feedback").into_val(&env),
                volunteer.clone().into_val(&env),
                organization.clone().into_val(&env),
                task_id.into_val(&env),
            ]
        );
    }
    
    #[test]
    #[should_panic(expected = "Feedback already submitted")]
    fn test_reject_duplicate_feedback() {
        let (env, client, volunteer, organization, task_id) = setup_env();
        let rating = 4u32;
        let comment = String::from_str(&env, "Great work!");

        client.submit_feedback(&volunteer, &organization, &task_id, &rating, &comment);
        client.submit_feedback(&volunteer, &organization, &task_id, &rating, &comment);
    }

    #[test]
    #[should_panic(expected = "Invalid rating")]
    fn test_reject_invalid_rating() {
        let (env, client, volunteer, organization, task_id) = setup_env();
        let invalid_rating = 6u32;
        let comment = String::from_str(&env, "Invalid rating test");

        client.submit_feedback(&volunteer, &organization, &task_id, &invalid_rating, &comment);
    }

    #[test]
    #[should_panic(expected = "Unauthorized participant")]
    fn test_reject_non_participant() {
        let (env, client, _, organization, task_id) = setup_env();
        let non_participant = Address::generate(&env);
        let rating = 4u32;
        let comment = String::from_str(&env, "Non-participant test");

        client.submit_feedback(&non_participant, &organization, &task_id, &rating, &comment);
    }

    #[test]
    #[should_panic(expected = "Comment too long")]
    fn test_reject_long_comment() {
        let (env, client, volunteer, organization, task_id) = setup_env();
        let rating = 4u32;
        let long_comment = String::from_str(&env, &"a".repeat(501));

        client.submit_feedback(&volunteer, &organization, &task_id, &rating, &long_comment);
    }


      #[test]
    fn test_get_feedbacks_and_count() {
        let (env, client, volunteer, organization, task_id) = setup_env();
        let rating1 = 4u32;
        let comment1 = String::from_str(&env, "Great organization!");
        let rating2 = 5u32;
        let comment2 = String::from_str(&env, "Excellent experience!");
    
        client.submit_feedback(&organization, &volunteer, &task_id, &rating1, &comment1);
        client.submit_feedback(&volunteer, &organization, &task_id, &rating2, &comment2);

        let org_feedbacks = client.get_feedbacks(&organization);
          assert_eq!(org_feedbacks.first().unwrap().rating, rating1);  // now receiver=org
    
        let vol_feedbacks = client.get_feedbacks(&volunteer);
           assert_eq!(vol_feedbacks.first().unwrap().rating, rating2);  // now receiver=vol
    }
}
