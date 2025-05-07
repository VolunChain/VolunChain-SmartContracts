use soroban_sdk::{contracterror, contracttype, Address, String};

/// @notice Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKeys {
    Admin, // Contract admin address
    RecognitionBadge(Address), // Recognition badges for a specific volunteer
    TokenCounter, // Global counter for token IDs
    VolunteerRecognition(Address), // List of badge IDs owned by a volunteer
}

/// @notice Structure representing a soulbound NFT badge
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecognitionNFT {
    pub owner: Address, // Address of the badge owner (volunteer)
    pub metadata: NFTMetadata, // Badge metadata containing event/contribution details
}

/// @notice Metadata structure for recognition badges
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub ev_org: Address, // Organization that endorsed this contribution
    pub ev_title: String, // Title of the event or contribution
    pub ev_date: String, // Date of the event or contribution
    pub task: String, // Specific task performed by the volunteer
}

/// @notice Admin-related error codes
#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum AdminError {
    AlreadyInitialized = 1,
    UnauthorizedSender = 2,
}


