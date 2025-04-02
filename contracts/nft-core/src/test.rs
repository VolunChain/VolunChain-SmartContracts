#![cfg(test)]

use crate::{NFTCore, types::{NFT, NFTError}};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec, Val, IntoVal};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    // Use register instead of register_contract
    let contract_id = env.register_contract(None, NFTCore {});
    let client = NFTCoreClient::new(&env, &contract_id);
    
    // Authorize transaction
    env.mock_all_auths();
    
    // Initialize the contract
    let result = client.initialize(&admin);
    assert!(result.is_ok());
}

#[test]
fn test_mint_nft() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let contract_id = env.register_contract(None, NFTCore {});
    let client = NFTCoreClient::new(&env, &contract_id);
    
    // Authorize transaction
    env.mock_all_auths();
    
    // Initialize the contract
    client.initialize(&admin).expect("Failed to initialize");
    
    // Create attributes
    let attributes = vec![
        &env,
        (String::from_str(&env, "trait_type"), String::from_str(&env, "badge")),
        (String::from_str(&env, "value"), String::from_str(&env, "recognition"))
    ];
    
    // Mint NFT
    let token_id_result = client.mint_nft(
        &admin,
        &recipient,
        &String::from_str(&env, "Test NFT"),
        &String::from_str(&env, "This is a test NFT"),
        &attributes,
        &false
    );
    
    assert!(token_id_result.is_ok());
    let token_id = token_id_result.unwrap();
    
    // Get the NFT
    let nft_result = client.get_nft(&token_id);
    assert!(nft_result.is_ok());
    let nft = nft_result.unwrap();
    
    assert_eq!(nft.owner, recipient);
}

// Test burning NFT
#[test]
fn test_burn_nft() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let contract_id = env.register_contract(None, NFTCore {});
    let client = NFTCoreClient::new(&env, &contract_id);
    
    // Authorize transaction
    env.mock_all_auths();
    
    // Initialize the contract
    client.initialize(&admin).expect("Failed to initialize");
    
    // Create attributes
    let attributes = vec![
        &env,
        (String::from_str(&env, "trait_type"), String::from_str(&env, "badge")),
        (String::from_str(&env, "value"), String::from_str(&env, "recognition"))
    ];
    
    // Mint NFT
    let token_id = client.mint_nft(
        &admin,
        &recipient,
        &String::from_str(&env, "Test NFT"),
        &String::from_str(&env, "This is a test NFT"),
        &attributes,
        &false
    ).expect("Failed to mint NFT");
    
    // Burn the NFT
    let result = client.burn_nft(&recipient, &token_id);
    assert!(result.is_ok());
    
    // Try to get the NFT - should fail
    let nft = client.get_nft(&token_id);
    assert!(nft.is_err());
    assert_eq!(nft.unwrap_err(), NFTError::TokenNotFound);
}

// Test adding authorized minter
#[test]
fn test_add_authorized_minter() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let minter = Address::generate(&env);
    
    let contract_id = env.register_contract(None, NFTCore {});
    let client = NFTCoreClient::new(&env, &contract_id);
    
    // Authorize transaction
    env.mock_all_auths();
    
    // Initialize the contract
    client.initialize(&admin).expect("Failed to initialize");
    
    // Add minter
    let result = client.add_authorized_minter(&admin, &minter);
    assert!(result.is_ok());
    
    // Check minter is authorized
    let is_authorized = client.is_authorized_minter(&minter);
    assert!(is_authorized);
}

// Client for testing
struct NFTCoreClient<'a> {
    env: &'a Env,
    contract_id: Address,
}

impl<'a> NFTCoreClient<'a> {
    fn new(env: &'a Env, contract_id: &Address) -> Self {
        Self { 
            env, 
            contract_id: contract_id.clone(),
        }
    }
    
    fn initialize(&self, admin: &Address) -> Result<(), NFTError> {
        self.env.invoke_contract(
            &self.contract_id,
            &"init", // Shortened function name
            &[self.env.to_val(), admin.to_val()],
        )
    }
    
    fn mint_nft(
        &self,
        minter: &Address,
        recipient: &Address,
        title: &String,
        description: &String,
        attributes: &Vec<(String, String)>,
        transferable: &bool
    ) -> Result<u128, NFTError> {
        self.env.invoke_contract(
            &self.contract_id,
            &"mint", // Shortened function name
            &[
                self.env.to_val(), 
                minter.to_val(), 
                recipient.to_val(), 
                title.to_val(self.env), 
                description.to_val(self.env), 
                attributes.to_val(self.env), 
                transferable.to_val(self.env)
            ],
        )
    }
    
    fn burn_nft(&self, owner: &Address, token_id: &u128) -> Result<(), NFTError> {
        self.env.invoke_contract(
            &self.contract_id,
            &"burn", // Shortened function name
            &[self.env.to_val(), owner.to_val(), token_id.to_val(self.env)],
        )
    }
    
    fn get_nft(&self, token_id: &u128) -> Result<NFT, NFTError> {
        self.env.invoke_contract(
            &self.contract_id,
            &"get", // Shortened function name
            &[self.env.to_val(), token_id.to_val(self.env)],
        )
    }
    
    fn add_authorized_minter(&self, admin: &Address, minter: &Address) -> Result<(), NFTError> {
        self.env.invoke_contract(
            &self.contract_id,
            &"add", // Shortened function name
            &[self.env.to_val(), admin.to_val(), minter.to_val()],
        )
    }
    
    fn is_authorized_minter(&self, minter: &Address) -> bool {
        self.env.invoke_contract(
            &self.contract_id,
            &"auth", // Shortened function name
            &[self.env.to_val(), minter.to_val()],
        )
    }
}