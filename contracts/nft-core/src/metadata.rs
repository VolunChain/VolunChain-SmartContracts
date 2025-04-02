// In nft-core/src/metadata.rs
use soroban_sdk::{Address, Env, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    // Common fields for all NFTs
    pub title: String,
    pub description: String,
    pub creation_date: String,
    pub issuer: Address,
    
    // Optional fields - stored as a serialized map
    pub attributes: Map<String, String>,
    
    // Link to external metadata (e.g., IPFS hash)
    pub external_url: Option<String>,
}

impl NFTMetadata {
    pub fn new(
        env: &Env,
        title: String,
        description: String,
        issuer: Address,
    ) -> Self {
        Self {
            title,
            description,
            creation_date: format_timestamp(env.ledger().timestamp()),
            issuer,
            attributes: Map::new(env),
            external_url: None,
        }
    }
    
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.set(key, value);
        self
    }
    
    pub fn with_external_url(mut self, url: String) -> Self {
        self.external_url = Some(url);
        self
    }
}