#![cfg(test)]

extern crate std;

use crate::storage::types::{Bounty, Task};
use crate::token::token::{Token, TokenClient};
use crate::{contract::VolunchainContract, VolunchainContractClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, IntoVal, String};

fn create_usdc_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register(Token, ()));
    token.initialize(admin, &7, &"USDC".into_val(e), &"USDC".into_val(e));
    token
}

#[test]
fn test_create_bounty() {
    let env = Env::default();
    env.mock_all_auths();

    let title = String::from_str(&env, "Bounty title");
    let description = String::from_str(&env, "Bounty description");
    let amount: i128 = 100_000_000;
    let owner = Address::generate(&env);
    let volunteer = Address::generate(&env);

    let tasks = vec![
        &env,
        Task {
            description: String::from_str(&env, "First task"),
            status: String::from_str(&env, "Delivered"),
            completed: true,
        },
        Task {
            description: String::from_str(&env, "Second task"),
            status: String::from_str(&env, "Delivered"),
            completed: true,
        },
    ];

    let contract_address = env.register(VolunchainContract, ());
    let contract_client = VolunchainContractClient::new(&env, &contract_address);

    let bounty_properties: Bounty = Bounty {
        title: title.clone(),
        description: description.clone(),
        amount: amount,
        owner: owner,
        volunteer: volunteer,
        tasks: tasks,
    };

    contract_client.create_bounty(&bounty_properties.clone());

    let bounty = contract_client.get_bounty();
    assert_eq!(bounty.title, bounty_properties.title);
    assert_eq!(bounty.description, bounty_properties.description);
    assert_eq!(bounty.amount, bounty_properties.amount);
    assert_eq!(bounty.owner, bounty_properties.owner);
    assert_eq!(bounty.volunteer, bounty_properties.volunteer);
    assert_eq!(bounty.tasks, bounty_properties.tasks);
}

#[test]
fn test_withdraw_the_reward() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let title = String::from_str(&env, "Bounty title");
    let description = String::from_str(&env, "Bounty description");
    let amount: i128 = 100_000_000;
    let owner = Address::generate(&env);
    let volunteer = Address::generate(&env);

    let tasks = vec![
        &env,
        Task {
            description: String::from_str(&env, "First task"),
            status: String::from_str(&env, "Delivered"),
            completed: true,
        },
        Task {
            description: String::from_str(&env, "Second task"),
            status: String::from_str(&env, "Delivered"),
            completed: true,
        },
    ];

    let contract_address = env.register(VolunchainContract, ());
    let contract_client = VolunchainContractClient::new(&env, &contract_address);
    let usdc_token = create_usdc_token(&env, &admin);

    let bounty_properties: Bounty = Bounty {
        title: title.clone(),
        description: description.clone(),
        amount: amount,
        owner: owner,
        volunteer: volunteer.clone(),
        tasks: tasks,
    };

    contract_client.create_bounty(&bounty_properties.clone());
    usdc_token.mint(&contract_address, &(amount as i128));
    let contract_balance = usdc_token.balance(&contract_address);

    assert_eq!(contract_balance, amount);

    contract_client.withdraw_reward(&volunteer, &usdc_token.address);

    let final_contract_balance = usdc_token.balance(&contract_address);
    let volunteer_balance = usdc_token.balance(&volunteer);

    assert_eq!(volunteer_balance, amount);
    assert_eq!(final_contract_balance, 0);
}