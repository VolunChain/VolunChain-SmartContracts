#![no_std]

mod core;
mod storage;
mod error;
mod token;
mod tests;
mod contract;

pub use crate::contract::VolunchainContractClient;