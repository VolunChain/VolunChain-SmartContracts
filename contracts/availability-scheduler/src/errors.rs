use soroban_sdk::{Error as SorobanError, xdr::{ScErrorType, ScErrorCode}};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidDay,
    InvalidTimeRange,
    OverlappingTimeSlots,
    Unauthorized,
}

impl From<Error> for SorobanError {
    fn from(error: Error) -> Self {
        let (error_type, error_code) = match error {
            Error::InvalidDay => (ScErrorType::Contract, ScErrorCode::InvalidInput),
            Error::InvalidTimeRange => (ScErrorType::Contract, ScErrorCode::InvalidInput),
            Error::OverlappingTimeSlots => (ScErrorType::Contract, ScErrorCode::InvalidInput),
            Error::Unauthorized => (ScErrorType::Auth, ScErrorCode::InvalidAction),
        };
        SorobanError::from_type_and_code(error_type, error_code)
    }
}

impl From<&Error> for SorobanError {
    fn from(error: &Error) -> Self {
        (*error).into()
    }
}

impl From<SorobanError> for Error {
    fn from(error: SorobanError) -> Self {
        match error {
            _ => Error::Unauthorized,
        }
    }
} 