use soroban_sdk::{contracterror, contracttype, Address, String, Map};

#[contracttype]
pub enum DataKey {
    NFT(u128),                  // Maps token ID to NFT
    OwnerTokens(Address),       // Maps owner to their tokens 
    TokenCount,                 // Counter for token IDs
    Admin,                      // Admin address
    AuthorizedMinters,          // List of authorized minters
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NFT {
    pub id: u128,
    pub owner: Address,
    pub metadata: NFTMetadata,
    pub transferable: bool,
    pub minted_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub issuer: Address,
    pub title: String,
    pub description: String,
    pub creation_date: u64,
    pub attributes: Map<String, String>,
}

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NFTError {
    TokenNotFound = 1,
    UnauthorizedMinter = 2,
    NotTokenOwner = 3,
    AdminRequired = 4,
    TokenAlreadyExists = 5,
    InvalidMetadata = 6,
    TokenNotTransferable = 7,
    ContractAlreadyInitialized = 8,
}