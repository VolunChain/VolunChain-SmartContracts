use soroban_sdk::{contracterror, contracttype, Address, String};

// Security constants for input validation
pub const MAX_TITLE_LEN: u32 = 128;
pub const MAX_DATE_LEN: u32 = 32;
pub const MAX_TASK_LEN: u32 = 256;
pub const MAX_PAGINATION_LIMIT: u32 = 100;

/// @notice Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKeys {
    Admin, // Contract admin address
    RecognitionBadge(Address), // Recognition badges for a specific volunteer
    TokenCounter, // Global counter for token IDs
    VolunteerRecognition(Address), // List of badge IDs owned by a volunteer
    ReputationContractId, // Contract ID
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

/// @notice NFT-related error codes
#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum NFTError {
    IDExists = 1,
    IDInvalid = 2,
    UnauthorizedOwner = 3,
    BadgeNotFound = 4,
    VolunteerNotEndorsed = 5,
    OrganizationNotAuthorized = 6,
    MetadataInvalid = 7,
    EventNotFound = 8,
    TokenCannotBeTransferred = 9,
    OperationNotPermitted = 10,
    TokenCounterOverflow = 11,
    TitleTooLong = 12,
    DateTooLong = 13,
    TaskTooLong = 14,
    InvalidAddress = 15,
    BadgeLimitExceeded = 16,
    InvalidDateFormat = 17,
    PaginationLimitExceeded = 18,
    ExternalContractError = 19,
}
