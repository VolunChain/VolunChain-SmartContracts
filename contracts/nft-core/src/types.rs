use soroban_sdk::{contracterror, contracttype, Address, String, Map, Vec};

#[contracttype]
pub enum DataKey {
    NFT(u128),                  // Maps token ID to NFT
    OwnerTokens(Address),       // Maps owner to their tokens 
    TokenCount,                 // Counter for token IDs
    Admin,                      // Admin address
    AuthorizedMinters,          // List of authorized minters
    ContractVersion,            // Contract version for upgrades
    URIBase,                    // Base URI for external metadata
    Paused,                     // Contract pause status
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

#[contracttype]
#[derive(Clone, Debug)]
pub struct NFTMintBatch {
    pub recipients: Vec<Address>,
    pub titles: Vec<String>,
    pub descriptions: Vec<String>,
    pub attributes: Vec<Vec<(String, String)>>,
    pub transferable: Vec<bool>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ExternalURIMetadata {
    pub base_uri: String,
    pub token_uri_suffix: String,
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
    InvalidAddress = 9,
    BatchDataMismatch = 10,
    MetadataTooLarge = 11,
    UpgradeNotAuthorized = 12,
    ContractPaused = 13,
    Unauthorized = 14,
    InitializationError = 15,
    InvalidInput = 16,
    InvalidBatchData = 17,
    InvalidRecipient = 18,
    BatchTooLarge = 19,
    ContractError = 20,


    IDInvalid = 21,
    BadgeNotFound = 22,
    OrganizationNotAuthorized = 23,
    MetadataInvalid = 24,
    TokenCannotBeTransferred = 25,
    UnauthorizedOwner = 26,
}
