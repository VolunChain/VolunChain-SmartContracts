use soroban_sdk::{contracterror, contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKeys {
    Admin,
    RecognitionBadge(Address),
    TokenCounter,
    VolunteerRecognition(Address),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecognitionNFT {
    pub owner: Address,
    pub metadata: NFTMetadata,
}

// Data Structure representing NFT Metadata
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub ev_org: Address,
    pub ev_title: String,
    pub ev_date: String,
    pub task: String,
}

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum AdminError {
    AlreadyInitialized = 1,
    UnauthorizedSender = 2,
}

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum NFTError {
    IDExists = 1,
    IDInvalid = 2,
}
