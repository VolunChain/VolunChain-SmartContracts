use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // General errors
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    
    // Organization errors
    OrganizationAlreadyRegistered = 101,
    OrganizationNotRegistered = 102,
    
    // Participation errors
    ParticipationAlreadyRegistered = 201,
    ParticipationNotFound = 202,
}