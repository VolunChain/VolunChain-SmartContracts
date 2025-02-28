use crate::{
    datatype::{DataKeys, NFTMetadata, RecognitionNFT, NFTError},
    interfaces::{MetadataOperations, MintingOperations},
    RecognitionSystemContract, RecognitionSystemContractArgs, RecognitionSystemContractClient,
};
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, TryFromVal,
    U256, Vec,
};

impl MintingOperations for RecognitionSystemContract {
    fn mint_recognition_badge(
        env: &Env,
        recipient: Address,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<U256, NFTError> {
        recipient.require_auth();

        let mut current_id: U256 = env
            .storage()
            .instance()
            .get(&DataKeys::TokenCounter)
            .unwrap_or(U256::from(&env, 0));
        current_id = current_id + 1;
        env.storage()
            .instance()
            .set(&DataKeys::TokenCounter, &current_id);

        let metadata = MetadataOperations::new(env, organization, title, date, task)?;
        let nft = RecognitionNFT {
            owner: recipient.clone(),
            metadata,
        };

        let token_id = current_id;
        env.storage().persistent().set(&token_id, &nft);

        let mut volunteer_tokens: Vec<U256> = env
            .storage()
            .persistent()
            .get(&DataKeys::VolunteerRecognition(recipient.clone()))
            .unwrap_or_else(|| Vec::new(env));
        volunteer_tokens.push_back(token_id);
        env.storage().persistent().set(
            &DataKeys::VolunteerRecognition(recipient.clone()),
            &volunteer_tokens,
        );

        Ok(token_id)
    }
}
