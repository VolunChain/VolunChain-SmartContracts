// ================= Contract Errors =================
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    NotOwner = 4,
    NotUpdater = 5,
    GoalNotFound = 6,
    TargetMustBePositive = 7,
    AmountToAddMustBePositive = 8,
    GoalAlreadyCompleted = 9,
    InvalidUpdaterAddress = 10,
    InvalidOwnerAddress = 11,
    NotAuthorized = 12,
}
