use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // --- General Errors ---
    AlreadyInitialized = 1,
    NotAuthorized = 2, // Also used for non-admin actions

    // --- Input Validation Errors ---
    TaskNameTooLong = 10,
    MetadataTooLong = 11,
    InvalidPaginationArguments = 12, // If offset/limit are problematic

    // --- Organization Errors ---
    OrganizationAlreadyRegistered = 101,
    OrganizationNotRegistered = 102,

    // --- Participation Errors ---
    ParticipationAlreadyRegistered = 201,
    ParticipationNotFound = 202,
}